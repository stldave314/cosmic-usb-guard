// SPDX-License-Identifier: GPL-3.0-or-later

//! High-level client for the USBGuard daemon.

use std::fmt;

use crate::constants::{DBUS_CALL_TIMEOUT, PARAM_INSERTED_DEVICE_POLICY};
use crate::debug_log;

use super::model::{Device, PolicyRule};
use super::proxy::{DevicesProxy, PolicyProxy, UsbGuardProxy};
use super::rule::{ParseError, Target};

/// Something went wrong talking to USBGuard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// `usbguard-dbus` is not running and could not be activated. Almost
    /// always means the USBGuard service itself is stopped or not installed.
    ServiceUnavailable(String),
    /// Polkit refused, or the user is not in USBGuard's IPC allow-list.
    PermissionDenied(String),
    /// The call did not return within [`DBUS_CALL_TIMEOUT`].
    Timeout,
    /// Any other D-Bus failure.
    Dbus(String),
    /// The daemon returned a rule string we could not parse.
    Parse(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceUnavailable(m) => write!(f, "USBGuard service unavailable: {m}"),
            Self::PermissionDenied(m) => write!(f, "permission denied: {m}"),
            Self::Timeout => f.write_str("timed out waiting for USBGuard"),
            Self::Dbus(m) => write!(f, "D-Bus error: {m}"),
            Self::Parse(e) => write!(f, "could not parse rule: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl Error {
    /// Whether retrying later might succeed without the user doing anything.
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::ServiceUnavailable(_) | Self::Timeout)
    }
}

/// Classify a zbus error.
///
/// D-Bus reports "service is not running" and "you may not do that" as errors
/// that look alike at the transport level but need very different responses
/// from the UI, so they are separated here by error name.
fn classify(error: zbus::Error) -> Error {
    if let zbus::Error::MethodError(name, message, _) = &error {
        return classify_method_error(name.as_str(), message.clone().unwrap_or_default());
    }
    Error::Dbus(error.to_string())
}

/// Classify a D-Bus method error by its error name and message.
///
/// Split out from [`classify`] so it can be tested: constructing a
/// `zbus::Error::MethodError` requires a real `zbus::Message`, which needs a
/// live bus connection.
fn classify_method_error(name: &str, message: String) -> Error {
    match name {
        "org.freedesktop.DBus.Error.ServiceUnknown"
        | "org.freedesktop.DBus.Error.NameHasNoOwner"
        | "org.freedesktop.DBus.Error.NoReply"
        | "org.freedesktop.DBus.Error.Spawn.ChildExited"
        | "org.freedesktop.DBus.Error.Spawn.ExecFailed"
        | "org.freedesktop.DBus.Error.Spawn.ServiceNotFound" => Error::ServiceUnavailable(message),
        "org.freedesktop.DBus.Error.AccessDenied"
        | "org.freedesktop.DBus.Error.AuthFailed"
        | "org.freedesktop.DBus.Error.InteractiveAuthorizationRequired" => {
            Error::PermissionDenied(message)
        }
        // USBGuard wraps its own failures in a generic exception, so fall
        // back to inspecting the message for the authorisation case.
        _ => {
            let lowered = message.to_lowercase();
            if lowered.contains("not authorized")
                || lowered.contains("not authorised")
                || lowered.contains("permission denied")
            {
                Error::PermissionDenied(message)
            } else {
                Error::Dbus(format!("{name}: {message}"))
            }
        }
    }
}

/// Make a call that changes policy, allowing an authentication prompt.
///
/// The D-Bus `ALLOW_INTERACTIVE_AUTHORIZATION` flag is opt-in, and without it
/// a Polkit action marked `auth_admin` comes straight back as `AccessDenied`
/// with no dialog shown — the user clicks a button and nothing happens. That
/// is not hypothetical: the polkit rules Debian and Ubuntu ship for USBGuard
/// grant `appendRule` and `applyDevicePolicy` to the `sudo` and `plugdev`
/// groups but leave `removeRule` at `auth_admin`, so on a stock install every
/// rule removal takes this path.
///
/// Only mutating calls get the flag. The read calls are made on a timer, and
/// a timer that can raise password prompts is a good way to train someone to
/// type their password into whatever asks.
async fn call_interactive<'a, B, R>(
    proxy: &zbus::Proxy<'a>,
    method: &str,
    body: &B,
) -> Result<R, Error>
where
    B: serde::ser::Serialize + zbus::zvariant::DynamicType,
    R: for<'d> zbus::zvariant::DynamicDeserialize<'d>,
{
    let call = proxy.call_with_flags::<_, _, R>(
        method,
        zbus::proxy::MethodFlags::AllowInteractiveAuth.into(),
        body,
    );
    match tokio::time::timeout(DBUS_CALL_TIMEOUT, call).await {
        Err(_) => Err(Error::Timeout),
        Ok(Err(e)) => Err(classify(e)),
        // `None` only comes back for a no-reply call, which none of these are.
        Ok(Ok(None)) => Err(Error::Dbus(format!("{method} returned no reply"))),
        Ok(Ok(Some(value))) => Ok(value),
    }
}

/// Await a D-Bus call with an upper bound on how long it may block.
///
/// Calls are Polkit-mediated, so a slow reply usually means an authentication
/// dialog is waiting on the user rather than that anything is wrong; the bound
/// exists so a lost reply cannot wedge the UI forever.
async fn with_timeout<T>(future: impl Future<Output = zbus::Result<T>>) -> Result<T, Error> {
    match tokio::time::timeout(DBUS_CALL_TIMEOUT, future).await {
        Err(_) => Err(Error::Timeout),
        Ok(Err(e)) => Err(classify(e)),
        Ok(Ok(value)) => Ok(value),
    }
}

/// A connected USBGuard client.
///
/// Cheap to clone: the underlying zbus connection is reference-counted.
#[derive(Clone)]
pub struct Client {
    connection: zbus::Connection,
    root: UsbGuardProxy<'static>,
    devices: DevicesProxy<'static>,
    policy: PolicyProxy<'static>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

impl Client {
    /// Connect to the system bus and build proxies for the USBGuard objects.
    ///
    /// This does not prove the daemon is reachable — D-Bus proxies are created
    /// lazily. Use [`Client::probe`] for that.
    pub async fn connect() -> Result<Self, Error> {
        debug_log!(crate::debug::DBUS, "connecting to the system bus");
        let connection = zbus::Connection::system().await.map_err(classify)?;

        let root = UsbGuardProxy::new(&connection).await.map_err(classify)?;
        let devices = DevicesProxy::new(&connection).await.map_err(classify)?;
        let policy = PolicyProxy::new(&connection).await.map_err(classify)?;

        debug_log!(crate::debug::DBUS, "proxies constructed");
        Ok(Self {
            connection,
            root,
            devices,
            policy,
        })
    }

    /// The underlying connection, for callers that need their own proxies.
    pub fn connection(&self) -> &zbus::Connection {
        &self.connection
    }

    /// The `Devices1` proxy, for signal subscription.
    pub fn devices_proxy(&self) -> &DevicesProxy<'static> {
        &self.devices
    }

    /// Make a cheap call that proves the daemon is actually answering.
    ///
    /// Reads `InsertedDevicePolicy`, which every supported daemon exposes and
    /// which the UI needs anyway.
    pub async fn probe(&self) -> Result<String, Error> {
        self.get_parameter(PARAM_INSERTED_DEVICE_POLICY).await
    }

    /// Every device USBGuard currently knows about.
    ///
    /// A device whose rule fails to parse is skipped rather than failing the
    /// whole listing: one malformed entry should not blank the device list.
    pub async fn list_devices(&self) -> Result<Vec<Device>, Error> {
        let raw = with_timeout(self.devices.list_devices("match")).await?;
        debug_log!(
            crate::debug::DEVICE,
            "listDevices returned {} rows",
            raw.len()
        );

        let mut devices = Vec::with_capacity(raw.len());
        for (id, rule) in raw {
            match Device::from_rule(id, &rule) {
                Ok(device) => devices.push(device),
                Err(e) => {
                    crate::error_log!(
                        crate::debug::DEVICE,
                        "skipping device {id}: {e} (rule: {rule})"
                    );
                }
            }
        }
        Ok(devices)
    }

    /// Apply an authorisation decision to a device.
    ///
    /// When `permanent` is set the daemon writes a matching rule into the
    /// policy so the decision survives a replug and a reboot.
    pub async fn apply_policy(
        &self,
        id: u32,
        target: Target,
        permanent: bool,
    ) -> Result<u32, Error> {
        debug_log!(
            crate::debug::DEVICE,
            "applyDevicePolicy id={id} target={target} permanent={permanent}"
        );
        call_interactive(
            self.devices.inner(),
            "applyDevicePolicy",
            &(id, target.to_dbus(), permanent),
        )
        .await
    }

    /// The daemon's current rule set.
    pub async fn list_rules(&self) -> Result<Vec<PolicyRule>, Error> {
        let raw = with_timeout(self.policy.list_rules("")).await?;
        debug_log!(
            crate::debug::POLICY,
            "listRules returned {} rows",
            raw.len()
        );

        let mut rules = Vec::with_capacity(raw.len());
        for (id, text) in raw {
            match PolicyRule::from_pair(id, &text) {
                Ok(rule) => rules.push(rule),
                Err(e) => {
                    crate::error_log!(
                        crate::debug::POLICY,
                        "skipping rule {id}: {e} (rule: {text})"
                    );
                }
            }
        }
        Ok(rules)
    }

    /// Append a rule to the end of the policy, returning its ID.
    pub async fn append_rule(&self, rule: &str, temporary: bool) -> Result<u32, Error> {
        debug_log!(
            crate::debug::POLICY,
            "appendRule temporary={temporary}: {rule}"
        );
        // `u32::MAX` is USBGuard's "append at the end" sentinel.
        call_interactive(
            self.policy.inner(),
            "appendRule",
            &(rule, u32::MAX, temporary),
        )
        .await
    }

    /// Remove a rule from the policy.
    pub async fn remove_rule(&self, id: u32) -> Result<(), Error> {
        debug_log!(crate::debug::POLICY, "removeRule id={id}");
        call_interactive(self.policy.inner(), "removeRule", &(id,)).await
    }

    /// Read a daemon runtime parameter.
    pub async fn get_parameter(&self, name: &str) -> Result<String, Error> {
        with_timeout(self.root.get_parameter(name)).await
    }

    /// Set a daemon runtime parameter, returning the previous value.
    pub async fn set_parameter(&self, name: &str, value: &str) -> Result<String, Error> {
        debug_log!(crate::debug::POLICY, "setParameter {name}={value}");
        call_interactive(self.root.inner(), "setParameter", &(name, value)).await
    }

    /// Remove every policy rule pinned to the given device hash.
    ///
    /// This is what "revoke" means for a device the user previously allowed
    /// permanently: the standing rule has to go, not just the live
    /// authorisation. Returns the number of rules removed.
    pub async fn remove_rules_for_hash(&self, hash: &str) -> Result<usize, Error> {
        if hash.is_empty() {
            return Ok(0);
        }
        let rules = self.list_rules().await?;
        let matching: Vec<u32> = rules
            .iter()
            .filter(|r| r.hash() == Some(hash))
            .map(|r| r.id)
            .collect();

        debug_log!(
            crate::debug::POLICY,
            "removing {} rule(s) for hash {hash}",
            matching.len()
        );

        // Removing a rule renumbers the ones after it, so work from the
        // highest ID down and the earlier IDs stay valid.
        let mut removed = 0;
        for id in matching.into_iter().rev() {
            self.remove_rule(id).await?;
            removed += 1;
        }
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classify_error(name: &str, message: &str) -> Error {
        classify_method_error(name, message.to_string())
    }

    #[test]
    fn classifies_a_stopped_service_as_unavailable() {
        let error = classify_error(
            "org.freedesktop.DBus.Error.ServiceUnknown",
            "The name org.usbguard1 was not provided",
        );
        assert!(matches!(error, Error::ServiceUnavailable(_)));
        assert!(error.is_transient());
    }

    #[test]
    fn classifies_polkit_refusal_as_permission_denied() {
        let error = classify_error("org.freedesktop.DBus.Error.AccessDenied", "Not authorized");
        assert!(matches!(error, Error::PermissionDenied(_)));
        // Retrying on a timer would just re-prompt the user forever.
        assert!(!error.is_transient());
    }

    #[test]
    fn classifies_usbguard_exception_text_as_permission_denied() {
        // USBGuard wraps its own failures in a generic exception name, so the
        // authorisation case is only visible in the message.
        let error = classify_error(
            "org.usbguard.Exception",
            "The caller is Not authorized to perform this action",
        );
        assert!(matches!(error, Error::PermissionDenied(_)));
    }

    #[test]
    fn classifies_other_errors_as_generic_dbus() {
        let error = classify_error("org.usbguard.Exception", "Device not found");
        assert!(matches!(error, Error::Dbus(_)));
        assert!(!error.is_transient());
    }

    #[test]
    fn a_missing_notification_daemon_is_not_mistaken_for_a_policy_refusal() {
        // Both surface as errors on the same bus; conflating them would make
        // the UI claim the user lacks permission when nothing is wrong.
        let error = classify_error(
            "org.freedesktop.DBus.Error.ServiceUnknown",
            "no such service",
        );
        assert!(!matches!(error, Error::PermissionDenied(_)));
    }
}
