// SPDX-License-Identifier: GPL-3.0-or-later

//! Checks that USBGuard is not merely installed, but actually protecting the
//! machine.
//!
//! Every check here is an *empirical* probe against the running system — a
//! systemd unit state, a live D-Bus call, a daemon parameter read back over
//! IPC. None of it is inferred from configuration files, because a USBGuard
//! install can be present, syntactically valid and completely inert: the
//! service can be disabled, the D-Bus bridge can be missing, the policy can be
//! empty, or `InsertedDevicePolicy` can be set to `allow`, which authorises
//! every new device before anyone can be asked about it.

use std::collections::HashMap;

use crate::constants::{
    INSERTED_POLICY_PREFERRED, INSERTED_POLICY_UNSAFE, PARAM_INSERTED_DEVICE_POLICY, UNIT_DAEMON,
    UNIT_DBUS,
};
use crate::debug_log;

use super::client::{Client, Error};
use super::proxy::{PolkitProxy, SystemdProxy};

/// How badly a failed check compromises protection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Working as intended.
    Ok,
    /// Works, but something is not as it should be.
    Warning,
    /// USBGuard is not protecting this machine.
    Critical,
}

/// Which check a result belongs to.
///
/// An enum rather than a string so the UI can localise the label and attach
/// the right remedy without matching on prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckId {
    /// `usbguard.service` is running.
    DaemonRunning,
    /// `usbguard.service` starts at boot.
    DaemonEnabled,
    /// `usbguard-dbus.service` is running.
    DbusRunning,
    /// `usbguard-dbus.service` starts at boot.
    DbusEnabled,
    /// The daemon answers our IPC calls.
    IpcReachable,
    /// This user is permitted to make policy decisions.
    IpcPermission,
    /// This user is permitted to *undo* one.
    DecisionsReversible,
    /// New devices are held for a decision rather than auto-authorised.
    InsertedDevicePolicy,
    /// The policy is not empty.
    PolicyNotEmpty,
}

/// The outcome of one check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    /// Which check this is.
    pub id: CheckId,
    /// How bad the outcome is.
    pub severity: Severity,
    /// Free-text detail, e.g. the observed value.
    pub detail: String,
    /// A shell command that would fix it, if there is a simple one.
    pub remedy: Option<String>,
}

impl Check {
    fn ok(id: CheckId, detail: impl Into<String>) -> Self {
        Self {
            id,
            severity: Severity::Ok,
            detail: detail.into(),
            remedy: None,
        }
    }

    fn warn(id: CheckId, detail: impl Into<String>, remedy: Option<String>) -> Self {
        Self {
            id,
            severity: Severity::Warning,
            detail: detail.into(),
            remedy,
        }
    }

    fn critical(id: CheckId, detail: impl Into<String>, remedy: Option<String>) -> Self {
        Self {
            id,
            severity: Severity::Critical,
            detail: detail.into(),
            remedy,
        }
    }
}

/// The full health picture.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Health {
    /// One entry per check, in display order.
    pub checks: Vec<Check>,
}

impl Health {
    /// The worst severity across all checks.
    pub fn worst(&self) -> Severity {
        self.checks
            .iter()
            .map(|c| c.severity)
            .max()
            .unwrap_or(Severity::Critical)
    }

    /// Whether everything passed.
    pub fn is_healthy(&self) -> bool {
        self.worst() == Severity::Ok
    }

    /// Look up a single check.
    pub fn check(&self, id: CheckId) -> Option<&Check> {
        self.checks.iter().find(|c| c.id == id)
    }

    /// Checks that did not pass, worst first.
    pub fn problems(&self) -> Vec<&Check> {
        let mut problems: Vec<&Check> = self
            .checks
            .iter()
            .filter(|c| c.severity != Severity::Ok)
            .collect();
        problems.sort_by_key(|check| std::cmp::Reverse(check.severity));
        problems
    }
}

/// Live state of a systemd unit.
#[derive(Debug, Clone, Default)]
struct UnitState {
    active: Option<String>,
    file_state: Option<String>,
}

/// Query systemd for the two units USBGuard needs.
///
/// A failure here is not fatal: systemd may be unreachable in a container, in
/// which case the D-Bus probe still tells us whether USBGuard works.
async fn unit_states(connection: &zbus::Connection) -> Option<(UnitState, UnitState)> {
    let systemd = SystemdProxy::new(connection).await.ok()?;

    let mut daemon = UnitState::default();
    let mut dbus = UnitState::default();

    if let Ok(units) = systemd.list_units_by_names(&[UNIT_DAEMON, UNIT_DBUS]).await {
        for (name, _desc, _load, active, _sub, _followed, _path, _job, _jt, _jp) in units {
            if name == UNIT_DAEMON {
                daemon.active = Some(active);
            } else if name == UNIT_DBUS {
                dbus.active = Some(active);
            }
        }
    }

    daemon.file_state = systemd.get_unit_file_state(UNIT_DAEMON).await.ok();
    dbus.file_state = systemd.get_unit_file_state(UNIT_DBUS).await.ok();

    Some((daemon, dbus))
}

fn check_unit_active(id: CheckId, unit: &str, state: &UnitState) -> Check {
    match state.active.as_deref() {
        Some("active") => Check::ok(id, "active"),
        Some(other) => Check::critical(
            id,
            other.to_string(),
            Some(format!("sudo systemctl start {unit}")),
        ),
        // systemd could not be queried; the IPC probe is the authority.
        None => Check::warn(id, "unknown", None),
    }
}

fn check_unit_enabled(id: CheckId, unit: &str, state: &UnitState) -> Check {
    match state.file_state.as_deref() {
        // `static` and `indirect` units have no enablement of their own; they
        // are pulled in by something else, which is fine.
        Some("enabled" | "enabled-runtime" | "static" | "indirect" | "alias") => {
            Check::ok(id, state.file_state.clone().unwrap_or_default())
        }
        Some("masked" | "masked-runtime") => {
            Check::critical(id, "masked", Some(format!("sudo systemctl unmask {unit}")))
        }
        Some(other) => Check::warn(
            id,
            other.to_string(),
            Some(format!("sudo systemctl enable {unit}")),
        ),
        None => Check::warn(id, "unknown", None),
    }
}

/// Polkit action guarding rule removal, which is what undoing a decision needs.
const ACTION_REMOVE_RULE: &str = "org.usbguard.Policy1.removeRule";

/// Whether this session may remove a policy rule.
///
/// Asked of Polkit rather than of USBGuard, because there is no harmless way
/// to ask USBGuard: `removeRule` takes a rule ID and deletes it, so probing
/// with the real call would mean deleting one of the user's rules to find out
/// whether deleting rules is allowed.
///
/// `flags` is 0 — no user interaction — so this check never raises a dialog of
/// its own. The three outcomes are genuinely different and are reported as
/// such: allowed outright, allowed after authenticating, and refused.
async fn check_decisions_reversible(connection: &zbus::Connection) -> Check {
    let Some(name) = connection.unique_name() else {
        return Check::warn(CheckId::DecisionsReversible, "unknown", None);
    };

    let polkit = match PolkitProxy::new(connection).await {
        Ok(polkit) => polkit,
        // No Polkit at all: nothing to report, since the daemon's own
        // authorisation is then whatever it is and `IpcPermission` covers it.
        Err(_) => return Check::warn(CheckId::DecisionsReversible, "polkit unavailable", None),
    };

    let subject = (
        "system-bus-name",
        HashMap::from([("name", zbus::zvariant::Value::from(name.as_str()))]),
    );

    match polkit
        .check_authorization(&subject, ACTION_REMOVE_RULE, HashMap::new(), 0, "")
        .await
    {
        Ok((true, _, _)) => Check::ok(CheckId::DecisionsReversible, "granted"),
        // Permitted, but every removal will ask for an administrator password.
        // Workable, so a warning rather than a failure.
        Ok((false, true, _)) => Check::warn(
            CheckId::DecisionsReversible,
            "requires an administrator password",
            Some(POLKIT_REMEDY.to_string()),
        ),
        Ok((false, false, _)) => Check::critical(
            CheckId::DecisionsReversible,
            "refused",
            Some(POLKIT_REMEDY.to_string()),
        ),
        Err(e) => Check::warn(CheckId::DecisionsReversible, e.to_string(), None),
    }
}

/// Shell command that grants rule removal to the same groups the distribution
/// already trusts with rule *creation*.
///
/// Written as a drop-in file rather than an edit to the packaged rules, so a
/// package upgrade cannot silently revert it and nothing already there is
/// disturbed. `50-` sorts before the shipped `org.usbguard1.rules`; Polkit
/// takes the first rule that returns a result.
const POLKIT_REMEDY: &str = concat!(
    "sudo tee /etc/polkit-1/rules.d/50-usbguard-remove-rule.rules >/dev/null <<'EOF'\n",
    "polkit.addRule(function(action, subject) {\n",
    "    if (action.id == \"org.usbguard.Policy1.removeRule\" &&\n",
    "        subject.active && subject.local &&\n",
    "        (subject.isInGroup(\"plugdev\") || subject.isInGroup(\"sudo\"))) {\n",
    "        return polkit.Result.YES;\n",
    "    }\n",
    "});\nEOF"
);

/// Run every check against the running system.
///
/// `client` may be `None` when a connection could not be established at all;
/// the systemd-derived checks still run, which is usually what explains it.
pub async fn evaluate(client: Option<&Client>, connection: Option<&zbus::Connection>) -> Health {
    debug_log!(crate::debug::HEALTH, "evaluating installation health");
    let mut checks = Vec::new();

    let units = match connection {
        Some(connection) => unit_states(connection).await,
        None => None,
    };
    let (daemon, dbus) = units.unwrap_or_default();

    checks.push(check_unit_active(
        CheckId::DaemonRunning,
        UNIT_DAEMON,
        &daemon,
    ));
    checks.push(check_unit_enabled(
        CheckId::DaemonEnabled,
        UNIT_DAEMON,
        &daemon,
    ));
    checks.push(check_unit_active(CheckId::DbusRunning, UNIT_DBUS, &dbus));
    checks.push(check_unit_enabled(CheckId::DbusEnabled, UNIT_DBUS, &dbus));

    let Some(client) = client else {
        checks.push(Check::critical(
            CheckId::IpcReachable,
            "no connection to the system bus",
            None,
        ));
        debug_log!(
            crate::debug::HEALTH,
            "no client; worst severity is critical"
        );
        return Health { checks };
    };

    // The authoritative test: does the daemon actually answer?
    match client.get_parameter(PARAM_INSERTED_DEVICE_POLICY).await {
        Ok(value) => {
            checks.push(Check::ok(CheckId::IpcReachable, "responding"));
            checks.push(inserted_policy_check(&value));
        }
        Err(Error::PermissionDenied(message)) => {
            // Reaching the daemon at all proves it is running; we are just not
            // allowed to talk to it.
            checks.push(Check::ok(CheckId::IpcReachable, "responding"));
            checks.push(Check::critical(
                CheckId::IpcPermission,
                message,
                Some(format!(
                    "sudo usbguard add-user {}",
                    whoami().unwrap_or_else(|| "$USER".into())
                )),
            ));
            debug_log!(crate::debug::HEALTH, "IPC reachable but permission denied");
            return Health { checks };
        }
        Err(e) => {
            checks.push(Check::critical(
                CheckId::IpcReachable,
                e.to_string(),
                Some(format!("sudo systemctl start {UNIT_DBUS}")),
            ));
            debug_log!(crate::debug::HEALTH, "IPC unreachable: {e}");
            return Health { checks };
        }
    }

    // Listing devices is the operation the UI depends on, so permission is
    // tested with the real call rather than a proxy for it.
    match client.list_devices().await {
        Ok(_) => checks.push(Check::ok(CheckId::IpcPermission, "granted")),
        Err(Error::PermissionDenied(message)) => checks.push(Check::critical(
            CheckId::IpcPermission,
            message,
            Some(format!(
                "sudo usbguard add-user {}",
                whoami().unwrap_or_else(|| "$USER".into())
            )),
        )),
        Err(e) => checks.push(Check::warn(CheckId::IpcPermission, e.to_string(), None)),
    }

    // Whether a decision can be taken back is part of whether this app works
    // at all, and it is invisible until someone tries.
    if let Some(connection) = connection {
        checks.push(check_decisions_reversible(connection).await);
    }

    match client.list_rules().await {
        Ok(rules) if rules.is_empty() => checks.push(Check::warn(
            CheckId::PolicyNotEmpty,
            "0 rules",
            Some("sudo sh -c 'usbguard generate-policy > /etc/usbguard/rules.conf'".into()),
        )),
        Ok(rules) => checks.push(Check::ok(
            CheckId::PolicyNotEmpty,
            format!("{} rules", rules.len()),
        )),
        // Reading the policy needs its own Polkit action, which a user may
        // lack while still being able to make device decisions.
        Err(e) => checks.push(Check::warn(CheckId::PolicyNotEmpty, e.to_string(), None)),
    }

    let health = Health { checks };
    debug_log!(
        crate::debug::HEALTH,
        "health evaluated: worst={:?}",
        health.worst()
    );
    health
}

/// Judge `InsertedDevicePolicy`, which decides what happens to a device
/// between it being plugged in and the user deciding about it.
fn inserted_policy_check(value: &str) -> Check {
    let normalised = value.trim().to_lowercase();
    if normalised == INSERTED_POLICY_PREFERRED {
        Check::ok(CheckId::InsertedDevicePolicy, normalised)
    } else if INSERTED_POLICY_UNSAFE.contains(&normalised.as_str()) {
        // The device is authorised the moment it is plugged in, so a prompt
        // would be asking permission for something that already happened.
        Check::critical(
            CheckId::InsertedDevicePolicy,
            normalised,
            Some(format!(
                "sudo usbguard set-parameter {PARAM_INSERTED_DEVICE_POLICY} {INSERTED_POLICY_PREFERRED}"
            )),
        )
    } else {
        // `block` and `reject` are safe but skip the policy, so a device the
        // user already allowed permanently is still blocked on insert.
        Check::warn(
            CheckId::InsertedDevicePolicy,
            normalised,
            Some(format!(
                "sudo usbguard set-parameter {PARAM_INSERTED_DEVICE_POLICY} {INSERTED_POLICY_PREFERRED}"
            )),
        )
    }
}

/// The current user's login name, for building an `add-user` remedy.
fn whoami() -> Option<String> {
    std::env::var("USER").ok().filter(|u| !u.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_on_insert_is_critical_because_it_defeats_prompting() {
        let check = inserted_policy_check("allow");
        assert_eq!(check.severity, Severity::Critical);
        assert!(check.remedy.is_some());
    }

    #[test]
    fn keep_on_insert_is_critical() {
        assert_eq!(inserted_policy_check("keep").severity, Severity::Critical);
    }

    #[test]
    fn apply_policy_on_insert_is_ok() {
        assert_eq!(inserted_policy_check("apply-policy").severity, Severity::Ok);
    }

    #[test]
    fn apply_policy_is_matched_case_insensitively() {
        assert_eq!(
            inserted_policy_check("  Apply-Policy  ").severity,
            Severity::Ok
        );
    }

    #[test]
    fn block_on_insert_is_only_a_warning() {
        assert_eq!(inserted_policy_check("block").severity, Severity::Warning);
    }

    #[test]
    fn inactive_unit_is_critical_with_a_remedy() {
        let state = UnitState {
            active: Some("failed".into()),
            file_state: Some("disabled".into()),
        };
        let check = check_unit_active(CheckId::DaemonRunning, UNIT_DAEMON, &state);
        assert_eq!(check.severity, Severity::Critical);
        assert_eq!(
            check.remedy.as_deref(),
            Some("sudo systemctl start usbguard.service")
        );
    }

    #[test]
    fn masked_unit_is_critical_and_suggests_unmasking() {
        let state = UnitState {
            active: Some("inactive".into()),
            file_state: Some("masked".into()),
        };
        let check = check_unit_enabled(CheckId::DaemonEnabled, UNIT_DAEMON, &state);
        assert_eq!(check.severity, Severity::Critical);
        assert!(check.remedy.unwrap().contains("unmask"));
    }

    #[test]
    fn static_units_count_as_enabled() {
        let state = UnitState {
            active: Some("active".into()),
            file_state: Some("static".into()),
        };
        assert_eq!(
            check_unit_enabled(CheckId::DbusEnabled, UNIT_DBUS, &state).severity,
            Severity::Ok
        );
    }

    #[test]
    fn worst_severity_wins_and_problems_sort_worst_first() {
        let health = Health {
            checks: vec![
                Check::ok(CheckId::DaemonRunning, "active"),
                Check::warn(CheckId::DbusEnabled, "disabled", None),
                Check::critical(CheckId::IpcReachable, "no reply", None),
            ],
        };
        assert_eq!(health.worst(), Severity::Critical);
        assert!(!health.is_healthy());

        let problems = health.problems();
        assert_eq!(problems.len(), 2);
        assert_eq!(problems[0].id, CheckId::IpcReachable);
        assert_eq!(problems[1].id, CheckId::DbusEnabled);
    }

    #[test]
    fn an_empty_health_is_not_reported_as_healthy() {
        // A health struct with no checks means evaluation never ran. Reporting
        // that as "all good" is exactly the silently-skipping green build the
        // checks exist to prevent.
        let health = Health::default();
        assert_eq!(health.worst(), Severity::Critical);
        assert!(!health.is_healthy());
    }
}
