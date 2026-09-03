// SPDX-License-Identifier: GPL-3.0-or-later

//! Async work both binaries need: applying decisions and refreshing state.
//!
//! The [`Client`] is cached across calls so a decision does not pay for a new
//! D-Bus connection, and is dropped whenever the daemon turns out to be gone
//! so the next call reconnects rather than reusing a dead handle.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::debug_log;
use crate::usbguard::{Client, Device, DeviceKey, Error, Health, PolicyRule, Target, health};

/// The shared client, created on first use.
fn cache() -> &'static Mutex<Option<Client>> {
    static CACHE: std::sync::OnceLock<Mutex<Option<Client>>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Get the cached client, connecting if necessary.
async fn client() -> Result<Client, Error> {
    let mut guard = cache().lock().await;
    if let Some(client) = guard.as_ref() {
        return Ok(client.clone());
    }
    let client = Client::connect().await?;
    *guard = Some(client.clone());
    Ok(client)
}

/// Drop the cached client so the next call reconnects.
async fn invalidate() {
    *cache().lock().await = None;
    debug_log!(crate::debug::DBUS, "cached client invalidated");
}

/// Run `f` with the client, dropping the cache if the service turns out to be
/// unavailable.
async fn with_client<T, F, Fut>(f: F) -> Result<T, Error>
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let client = match client().await {
        Ok(client) => client,
        Err(e) => {
            invalidate().await;
            return Err(e);
        }
    };

    match f(client).await {
        Err(e) if matches!(e, Error::ServiceUnavailable(_)) => {
            invalidate().await;
            Err(e)
        }
        other => other,
    }
}

/// Everything a refresh gathers, in one round trip's worth of calls.
#[derive(Debug, Clone)]
pub struct Refreshed {
    /// The device list, or the error that prevented reading it.
    pub devices: Result<Vec<Device>, String>,
    /// The health report. Always present: health checks describe failure as
    /// well as success, so there is nothing to fail into.
    pub health: Health,
    /// The daemon's rule set, when the policy could be read.
    ///
    /// `None` means "not known", which the state treats as "ask anyway"
    /// rather than "no rules exist".
    pub rules: Option<Vec<PolicyRule>>,
}

/// Why a refresh is happening.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    /// A timer fired.
    Automatic,
    /// The user pressed Refresh, or an action completed.
    Manual,
}

/// Records that a Polkit-mediated call was refused, so the timer stops making
/// it.
///
/// Every USBGuard method goes through Polkit. On a system where the user is
/// not in USBGuard's IPC allow-list, a refusal can mean an authentication
/// dialog — and a refresh on a timer would then raise one every few seconds,
/// which is both unusable and a good way to train someone to type their
/// password into whatever asks. Once a call is refused it is not retried
/// automatically; pressing Refresh clears the latch and tries again.
mod denied {
    use std::sync::atomic::{AtomicBool, Ordering};

    pub static DEVICES: AtomicBool = AtomicBool::new(false);
    pub static POLICY: AtomicBool = AtomicBool::new(false);

    pub fn set(flag: &AtomicBool) {
        flag.store(true, Ordering::Relaxed);
    }

    pub fn is_set(flag: &AtomicBool) -> bool {
        flag.load(Ordering::Relaxed)
    }

    pub fn clear_all() {
        DEVICES.store(false, Ordering::Relaxed);
        POLICY.store(false, Ordering::Relaxed);
    }
}

/// Re-read devices, policy and health.
pub async fn refresh(trigger: Trigger) -> Arc<Refreshed> {
    if trigger == Trigger::Manual {
        // An explicit request is the user asking us to try again, and is the
        // only context in which re-prompting for authentication is reasonable.
        denied::clear_all();
    }

    let connected = client().await.ok();
    let connection = connected.as_ref().map(|c| c.connection().clone());

    // Health is systemd state plus a parameter read, and is safe to poll: the
    // permission-sensitive parts of it latch through the same flags below.
    let health = health::evaluate(connected.as_ref(), connection.as_ref()).await;

    let devices = match connected.as_ref() {
        _ if denied::is_set(&denied::DEVICES) => Err(crate::fl!("error-permission-denied")),
        Some(client) => match client.list_devices().await {
            Ok(devices) => Ok(devices),
            Err(e) => {
                if matches!(e, Error::PermissionDenied(_)) {
                    denied::set(&denied::DEVICES);
                }
                Err(e.to_string())
            }
        },
        None => Err(crate::fl!("error-service-unavailable")),
    };

    let rules = match connected.as_ref() {
        _ if denied::is_set(&denied::POLICY) => None,
        Some(client) => match client.list_rules().await {
            Ok(rules) => Some(rules),
            Err(e) => {
                if matches!(e, Error::PermissionDenied(_)) {
                    debug_log!(
                        crate::debug::POLICY,
                        "policy read refused; not retrying until the user asks"
                    );
                    denied::set(&denied::POLICY);
                }
                // `None` means "unknown", which makes the prompt logic ask
                // about a device rather than assume it is already covered.
                None
            }
        },
        None => None,
    };

    if devices.is_err() {
        invalidate().await;
    }

    Arc::new(Refreshed {
        devices,
        health,
        rules,
    })
}

/// Everything a decision needs that the async task cannot look up for itself.
///
/// Built on the UI thread from the [`Device`] the user clicked, because by the
/// time the task runs the device list may already have been replaced.
#[derive(Debug, Clone)]
pub struct Decision {
    /// Which device, for clearing the busy marker when the call returns.
    pub key: DeviceKey,
    /// Daemon-assigned ID, or `None` when the device is not plugged in.
    pub daemon_id: Option<u32>,
    /// Descriptor hash. Empty when the device did not report one.
    pub hash: String,
    /// The rule to write when the device is not plugged in and there is
    /// nothing to authorise now.
    pub rule: Option<String>,
}

impl Decision {
    /// Build a decision for `device` with the given target.
    pub fn new(device: &Device, target: Target) -> Self {
        Self {
            key: device.key(),
            daemon_id: device.daemon_id(),
            hash: device.hash.clone(),
            rule: device.retargeted_rule(target),
        }
    }
}

/// Apply a decision to a device.
///
/// Returns the device key so the caller can clear its busy marker without
/// threading it through the message type.
///
/// A permanent decision *replaces* any rule already pinned to this device
/// rather than adding one. `usbguard-rules.conf(5)`: "the daemon scans the
/// existing rules sequentially. If a matching rule is found, it either
/// authorizes (allows), deauthorizes (blocks) or removes (rejects) the
/// device". Rules are appended at the end, so a new `allow hash X` written
/// while `reject hash X` is still in the policy would sit behind it and never
/// be reached — the user would click Allow, see no error, and find the device
/// rejected again on the next replug. Removing first is what makes the undo
/// real rather than apparent.
pub async fn decide(
    decision: Decision,
    target: Target,
    permanent: bool,
) -> (DeviceKey, Result<(), String>) {
    let Decision {
        key,
        daemon_id,
        hash,
        rule,
    } = decision;

    let result = with_client(|client| async move {
        if permanent && !hash.is_empty() {
            client
                .remove_rules_for_hash(&hash)
                .await
                .map_err(irreversible)?;
        }

        match daemon_id {
            // Plugged in: let the daemon apply the decision and, when it is
            // permanent, generate the rule itself. Its generated rule carries
            // the name, serial and interface list, which ours would not.
            Some(id) => {
                client.apply_policy(id, target, permanent).await?;
            }
            // Not plugged in: there is no device to authorise, so the decision
            // can only be recorded for next time.
            None => {
                let rule = rule.ok_or_else(|| {
                    Error::Dbus(
                        "device has no descriptor hash, so no rule can be pinned to it".into(),
                    )
                })?;
                client.append_rule(&rule, false).await?;
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string());

    (key, result)
}

/// Revoke a device: remove its standing rules, then block it now.
///
/// Both halves matter. Removing the rule alone leaves the device authorised
/// until it is unplugged; blocking alone leaves a rule that re-authorises it
/// on the next replug. The rule is removed first so that, if the block fails,
/// the device is not left with a stale allow rule.
pub async fn revoke(
    key: DeviceKey,
    daemon_id: u32,
    hash: String,
) -> (DeviceKey, Result<usize, String>) {
    let result = with_client(|client| async move {
        let removed = client
            .remove_rules_for_hash(&hash)
            .await
            .map_err(irreversible)?;
        client.apply_policy(daemon_id, Target::Block, false).await?;
        Ok(removed)
    })
    .await
    .map_err(|e| e.to_string());

    (key, result)
}

/// Remove a device's standing rules and leave its live state alone.
///
/// The undo for a remembered decision. With no rule pinned to it the device
/// falls back to USBGuard's implicit target, which is to block and ask — so
/// the user gets the question again instead of having to commit to the
/// opposite answer.
pub async fn forget(key: DeviceKey, hash: String) -> (DeviceKey, Result<usize, String>) {
    let result = with_client(|client| async move {
        client
            .remove_rules_for_hash(&hash)
            .await
            .map_err(irreversible)
    })
    .await
    .map_err(|e| e.to_string());

    (key, result)
}

/// Turn a refused rule removal into an error that says what to do about it.
///
/// "permission denied: ..." is true but useless here, because the user *can*
/// make decisions — `appendRule` and `applyDevicePolicy` are granted by the
/// polkit rules Debian and Ubuntu ship, and only `removeRule` is left at
/// `auth_admin`. Being told they lack permission moments after successfully
/// blocking something would just look broken.
fn irreversible(error: Error) -> Error {
    match error {
        Error::PermissionDenied(_) => {
            Error::PermissionDenied(crate::fl!("error-cannot-remove-rule"))
        }
        other => other,
    }
}

/// Read a daemon runtime parameter.
pub async fn get_parameter(name: String) -> Result<String, String> {
    with_client(|client| async move { client.get_parameter(&name).await })
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The latch and its reset are what stand between a refused call and an
    /// authentication dialog every twenty seconds, so both directions are
    /// checked rather than assumed.
    ///
    /// Serialised into one test because the flags are process-global statics;
    /// separate `#[test]` functions would race under the default harness.
    #[test]
    fn a_refusal_latches_until_the_user_asks_again() {
        denied::clear_all();
        assert!(!denied::is_set(&denied::DEVICES));
        assert!(!denied::is_set(&denied::POLICY));

        denied::set(&denied::POLICY);
        assert!(denied::is_set(&denied::POLICY));
        assert!(
            !denied::is_set(&denied::DEVICES),
            "a policy refusal must not suppress the device listing too"
        );

        denied::set(&denied::DEVICES);
        assert!(denied::is_set(&denied::DEVICES));

        // Only an explicit request clears it.
        denied::clear_all();
        assert!(!denied::is_set(&denied::DEVICES));
        assert!(!denied::is_set(&denied::POLICY));
    }

    #[test]
    fn only_a_manual_refresh_is_treated_as_permission_to_re_prompt() {
        // Guards the branch in `refresh`: if these ever compared equal, the
        // timer would clear the latch on every tick and the latch would do
        // nothing at all.
        assert_ne!(Trigger::Automatic, Trigger::Manual);
    }
}
