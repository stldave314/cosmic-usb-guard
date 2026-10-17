// SPDX-License-Identifier: GPL-3.0-or-later

//! Async work both binaries need: applying decisions and refreshing state.
//!
//! The [`Client`] is cached across calls so a decision does not pay for a new
//! D-Bus connection, and is dropped whenever the daemon turns out to be gone
//! so the next call reconnects rather than reusing a dead handle.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::debug_log;
use crate::thunderbolt;
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
    /// Thunderbolt devices and domains, or `None` on a machine without
    /// Thunderbolt or USB4.
    pub thunderbolt: Option<thunderbolt::Snapshot>,
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

    // USBGuard and Thunderbolt are read at the same time, so a bolt that is
    // slow to answer cannot hold up the USB results. Thunderbolt's checks join
    // the same list, so the banner, the tray icon and the Status page weigh
    // them like any other.
    let usb = async {
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

        // Health is systemd state, a parameter read and Polkit questions, all
        // safe to poll; the policy it judges is the one just read, so it
        // makes no call of its own past the latch.
        let health =
            health::evaluate(connected.as_ref(), connection.as_ref(), rules.as_deref()).await;
        (devices, rules, health)
    };

    let ((devices, rules, mut health), bolt) = futures::join!(usb, read_thunderbolt());
    if let Some((result, bolt_connection)) = &bolt {
        health.checks.extend(
            thunderbolt::health::evaluate(result, bolt_connection.as_ref().or(connection.as_ref()))
                .await,
        );
    }

    if devices.is_err() {
        invalidate().await;
    }

    Arc::new(Refreshed {
        devices,
        health,
        rules,
        thunderbolt: bolt.map(|(result, _)| snapshot_or_error(result)),
    })
}

/// The shared bolt client, created on first use.
fn bolt_cache() -> &'static Mutex<Option<thunderbolt::Client>> {
    static CACHE: std::sync::OnceLock<Mutex<Option<thunderbolt::Client>>> =
        std::sync::OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Read bolt, with the connection it was read over.
///
/// `None` on a machine without Thunderbolt or USB4, so nothing is asked of a
/// service that has nothing to manage. The connection is handed back for the
/// Polkit check, which must ask about the same bus connection it describes.
async fn read_thunderbolt() -> Option<(
    Result<thunderbolt::Snapshot, Error>,
    Option<zbus::Connection>,
)> {
    if !thunderbolt::sysfs::present() {
        return None;
    }

    let client = {
        let mut guard = bolt_cache().lock().await;
        match guard.as_ref() {
            Some(client) => client.clone(),
            None => match thunderbolt::Client::connect().await {
                Ok(client) => {
                    *guard = Some(client.clone());
                    client
                }
                Err(e) => return Some((Err(e), None)),
            },
        }
    };

    let result = client.snapshot().await;
    if result.is_err() {
        // bolt may have been restarted or removed; reconnect next time.
        *bolt_cache().lock().await = None;
    }
    Some((result, Some(client.connection().clone())))
}

/// A snapshot to hand the UI, with a failure described rather than dropped.
fn snapshot_or_error(result: Result<thunderbolt::Snapshot, Error>) -> thunderbolt::Snapshot {
    result.unwrap_or_else(|e| thunderbolt::Snapshot {
        error: Some(thunderbolt_error(&e)),
        ..thunderbolt::Snapshot::default()
    })
}

/// Re-read Thunderbolt alone, after bolt signalled a change.
///
/// `None` on a machine without Thunderbolt or USB4.
pub async fn refresh_thunderbolt() -> Option<thunderbolt::Snapshot> {
    read_thunderbolt()
        .await
        .map(|(result, _)| snapshot_or_error(result))
}

/// Something the user can do to a Thunderbolt device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThunderboltAction {
    /// Authorize it for this connection.
    Authorize,
    /// Authorize it and have bolt remember it.
    Enroll,
    /// Have bolt forget it.
    Forget,
}

/// Carry out a Thunderbolt action, returning the device UID with the result.
pub async fn thunderbolt_action(
    uid: String,
    action: ThunderboltAction,
) -> (String, Result<(), String>) {
    let result = async {
        let client = match bolt_cache().lock().await.clone() {
            Some(client) => client,
            None => thunderbolt::Client::connect().await?,
        };
        match action {
            ThunderboltAction::Authorize => client.authorize(&uid).await,
            ThunderboltAction::Enroll => client.enroll(&uid).await,
            ThunderboltAction::Forget => client.forget(&uid).await,
        }
    }
    .await
    .map_err(|e| thunderbolt_error(&e));

    (uid, result)
}

/// A Thunderbolt failure in words the user can act on.
fn thunderbolt_error(error: &Error) -> String {
    match error {
        Error::PermissionDenied(_) => crate::fl!("error-thunderbolt-permission"),
        Error::ServiceUnavailable(_) => crate::fl!("error-bolt-unavailable"),
        other => crate::fl!("error-thunderbolt", message = other.to_string()),
    }
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
    /// The rule to write ourselves, when the daemon's own would not do.
    ///
    /// Two cases. Not plugged in: there is nothing to authorize, so a rule is
    /// all the decision can be. Bound to a port: the rules the daemon writes
    /// for a permanent decision never carry `via-port`. Otherwise `None`, and
    /// the daemon writes the rule.
    pub rule: Option<String>,
    /// The port the standing rule is bound to, or `None` for any port.
    pub port: Option<String>,
}

impl Decision {
    /// Build a decision for `device` with the given target.
    ///
    /// `bind` pins the rule to the port a plugged-in device is in now. A
    /// device that is not plugged in keeps whatever binding its rule already
    /// has, since there is no current port to bind it to.
    pub fn new(device: &Device, target: Target, bind: bool) -> Self {
        let (rule, port) = if device.is_connected() {
            // The port counts only if a rule carrying it was built; a bind
            // request whose rule could not be built falls back to an unbound
            // decision, and must not then be compared as if it were bound.
            let bound = if bind && device.can_bind_port() {
                let port = device.via_port.clone();
                device
                    .rule_with_port(target, Some(&port))
                    .map(|rule| (rule, port))
            } else {
                None
            };
            match bound {
                Some((rule, port)) => (Some(rule), Some(port)),
                None => (None, None),
            }
        } else {
            let port = (!device.via_port.is_empty()).then(|| device.via_port.clone());
            (device.retargeted_rule(target), port)
        };

        Self {
            key: device.key(),
            daemon_id: device.daemon_id(),
            hash: device.hash.clone(),
            rule,
            port,
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
        port,
    } = decision;

    let result = with_client(|client| async move {
        // Nothing to replace when the policy already says this. Skipping the
        // removal matters beyond tidiness: `removeRule` is the one call the
        // stock polkit rules leave at `auth_admin`, so removing and re-adding
        // an identical rule turns a no-op into a password prompt, and into a
        // failure when that prompt is refused.
        let recorded = permanent
            && !hash.is_empty()
            && already_recorded(&client.list_rules().await?, &hash, target, port.as_deref());

        if permanent && !hash.is_empty() && !recorded {
            client
                .remove_rules_for_hash(&hash)
                .await
                .map_err(irreversible)?;
        }

        match daemon_id {
            // Plugged in. Normally the daemon applies the decision and, when it
            // is permanent, generates the rule itself: its rule carries the
            // name, serial and interface list. A port-bound rule is the
            // exception, because the daemon does not write `via-port` in its
            // default configuration (`DeviceRulesWithPort=false`). That rule
            // is appended here, after the old ones were removed above, and the
            // decision is then applied without the daemon writing a second.
            // Removal comes first because it is the step a refused password
            // stops, and stopping there leaves everything as it was.
            Some(id) => {
                let write_own = permanent && !recorded && rule.is_some();
                if write_own && let Some(rule) = rule.as_deref() {
                    client.append_rule(rule, false).await?;
                }
                let daemon_writes = permanent && !recorded && rule.is_none();
                client.apply_policy(id, target, daemon_writes).await?;
            }
            // Not plugged in and already recorded: the rule is the whole
            // decision, and it is there.
            None if recorded => {}
            // Not plugged in: there is no device to authorize, so the decision
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

/// Whether the rules pinned to `hash` already say `target` on `port`, and
/// nothing else.
///
/// "Nothing else" because the daemon takes the first match: an `allow` behind
/// a `reject` for the same hash is not an allow, and still has to be replaced.
/// The port is compared too, so binding or unbinding a decision that otherwise
/// stands is never mistaken for a no-op.
fn already_recorded(rules: &[PolicyRule], hash: &str, target: Target, port: Option<&str>) -> bool {
    let mut pinned = rules
        .iter()
        .filter(|rule| rule.hash() == Some(hash))
        .peekable();
    pinned.peek().is_some() && pinned.all(|rule| rule.rule.target == target && rule.port() == port)
}

/// Replace a device's standing rules with `rule`, and optionally apply a
/// decision to the device as it stands.
///
/// Used to bind a standing decision to a port or release it. The old rules are
/// removed first, the same order as [`decide`]: removal is the step a refused
/// password stops, and stopping there leaves the policy exactly as it was.
/// The reverse order would leave the new rule appended *behind* the old one,
/// which the daemon reaches second and so never applies.
///
/// `apply` carries a plugged-in device's ID and the rule's target when the
/// rewritten rule covers a device the old one did not — a device blocked
/// because it was on another port. The daemon does not re-evaluate a device
/// already present when its rule changes, so without this the device would
/// stay blocked under a rule that says otherwise.
pub async fn replace_rule(
    key: DeviceKey,
    hash: String,
    rule: String,
    apply: Option<(u32, Target)>,
) -> (DeviceKey, Result<(), String>) {
    let result = with_client(|client| async move {
        client
            .remove_rules_for_hash(&hash)
            .await
            .map_err(irreversible)?;
        client.append_rule(&rule, false).await?;
        if let Some((id, target)) = apply {
            client.apply_policy(id, target, false).await?;
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string());

    (key, result)
}

/// Revoke a device: remove its standing rules, then block it now.
///
/// Both halves matter. Removing the rule alone leaves the device authorized
/// until it is unplugged; blocking alone leaves a rule that re-authorizes it
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
    /// Serialized into one test because the flags are process-global statics;
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

    fn rules(texts: &[&str]) -> Vec<PolicyRule> {
        texts
            .iter()
            .enumerate()
            .map(|(i, text)| PolicyRule::from_pair(i as u32, text).unwrap())
            .collect()
    }

    #[test]
    fn a_decision_the_policy_already_records_needs_no_rule_removed() {
        let policy = rules(&[
            r#"allow id 0781:55a9 hash "stick""#,
            r#"block id 1234:5678 hash "other""#,
        ]);

        // The case that failed: Allow on a device that is already always
        // allowed went through `removeRule` and was refused.
        assert!(already_recorded(&policy, "stick", Target::Allow, None));

        // A different answer, or no rule at all, still has work to do.
        assert!(!already_recorded(&policy, "stick", Target::Block, None));
        assert!(!already_recorded(&policy, "other", Target::Allow, None));
        assert!(!already_recorded(&policy, "absent", Target::Allow, None));
    }

    #[test]
    fn a_matching_rule_behind_a_contradicting_one_is_not_already_recorded() {
        // First match wins in the daemon, so the allow here is unreachable and
        // the pair has to be replaced rather than left alone.
        let policy = rules(&[
            r#"reject id 0781:55a9 hash "stick""#,
            r#"allow id 0781:55a9 hash "stick""#,
        ]);
        assert!(!already_recorded(&policy, "stick", Target::Allow, None));
    }

    #[test]
    fn binding_or_unbinding_is_never_skipped_as_already_recorded() {
        let unbound = rules(&[r#"allow id 0781:55a9 hash "stick""#]);
        let bound = rules(&[r#"allow id 0781:55a9 hash "stick" via-port "3-5""#]);

        assert!(!already_recorded(
            &unbound,
            "stick",
            Target::Allow,
            Some("3-5")
        ));
        assert!(!already_recorded(&bound, "stick", Target::Allow, None));
        assert!(!already_recorded(
            &bound,
            "stick",
            Target::Allow,
            Some("1-2")
        ));
        assert!(already_recorded(
            &bound,
            "stick",
            Target::Allow,
            Some("3-5")
        ));
    }

    fn device(rule: &str, connected: bool) -> Device {
        if connected {
            Device::from_rule(7, rule).unwrap()
        } else {
            Device::remembered(rule).unwrap()
        }
    }

    #[test]
    fn a_bound_decision_writes_its_own_rule_with_the_current_port() {
        let stick = device(
            r#"block id 0781:55a9 hash "stick" parent-hash "hub" via-port "3-5""#,
            true,
        );

        let bound = Decision::new(&stick, Target::Allow, true);
        assert_eq!(bound.port.as_deref(), Some("3-5"));
        assert_eq!(
            bound.rule.as_deref(),
            Some(r#"allow id 0781:55a9 hash "stick" parent-hash "hub" via-port "3-5""#)
        );

        // Unbound, the daemon writes the rule, and it never carries a port.
        let unbound = Decision::new(&stick, Target::Allow, false);
        assert_eq!(unbound.rule, None);
        assert_eq!(unbound.port, None);
    }

    #[test]
    fn an_unplugged_device_keeps_the_binding_its_rule_already_has() {
        let bound = device(r#"block id 0781:55a9 hash "stick" via-port "3-5""#, false);
        let decision = Decision::new(&bound, Target::Allow, false);
        assert_eq!(decision.port.as_deref(), Some("3-5"));
        assert_eq!(
            decision.rule.as_deref(),
            Some(r#"allow id 0781:55a9 hash "stick" via-port "3-5""#)
        );
    }

    #[test]
    fn only_a_manual_refresh_is_treated_as_permission_to_re_prompt() {
        // Guards the branch in `refresh`: if these ever compared equal, the
        // timer would clear the latch on every tick and the latch would do
        // nothing at all.
        assert_ne!(Trigger::Automatic, Trigger::Manual);
    }
}
