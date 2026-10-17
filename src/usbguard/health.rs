// SPDX-License-Identifier: GPL-3.0-or-later

//! Checks that USBGuard is not merely installed, but actually protecting the
//! machine.
//!
//! Every check here is an *empirical* probe against the running system — a
//! systemd unit state, a live D-Bus call, a daemon parameter read back over
//! IPC. None of it is inferred from configuration files, because a USBGuard
//! install can be present, syntactically valid and completely inert: the
//! service can be disabled, the D-Bus bridge can be missing, the policy can be
//! empty, or `InsertedDevicePolicy` can be set to `allow`, which authorizes
//! every new device before anyone can be asked about it.

use std::collections::HashMap;

use crate::constants::{
    INSERTED_POLICY_PREFERRED, INSERTED_POLICY_UNSAFE, PARAM_INSERTED_DEVICE_POLICY, UNIT_DAEMON,
    UNIT_DBUS,
};
use crate::debug_log;

use super::client::{Client, Error};
use super::model::PolicyRule;
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
/// An enum rather than a string so the UI can localize the label and attach
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
    /// New devices are held for a decision rather than auto-authorized.
    InsertedDevicePolicy,
    /// The policy is not empty.
    PolicyNotEmpty,
    /// Thunderbolt/USB4 devices need authorization before they get PCIe.
    ThunderboltSecurity,
    /// The IOMMU shields memory from Thunderbolt/USB4 devices.
    ThunderboltDmaProtection,
    /// The bolt service, which authorizes Thunderbolt devices, is reachable.
    ThunderboltService,
    /// This user may authorize Thunderbolt devices.
    ThunderboltPermission,
}

/// Which part of the system a check is about, for grouping on the Status page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckGroup {
    /// USBGuard and its D-Bus bridge.
    UsbGuard,
    /// Thunderbolt/USB4 and the bolt service.
    Thunderbolt,
}

impl CheckId {
    /// The group this check is listed under.
    pub fn group(self) -> CheckGroup {
        match self {
            Self::ThunderboltSecurity
            | Self::ThunderboltDmaProtection
            | Self::ThunderboltService
            | Self::ThunderboltPermission => CheckGroup::Thunderbolt,
            _ => CheckGroup::UsbGuard,
        }
    }
}

/// Instructions for a problem no command can fix.
///
/// Some settings live only in the computer's firmware. Offering a command for
/// those would be a lie, and offering nothing leaves the user stuck, so these
/// name a set of translated, numbered steps instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Steps {
    /// Raise the Thunderbolt security level in firmware setup.
    ThunderboltSecurity,
    /// Turn on the IOMMU / Kernel DMA Protection in firmware setup.
    DmaProtection,
    /// Use USBGuard from the active, local session.
    ///
    /// Polkit grants USBGuard's read calls only to the session at the
    /// computer, so a refusal there is about where the app is running, not a
    /// setting a command could change.
    InactiveSession,
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
    /// Steps to follow when the fix is not a command.
    pub steps: Option<Steps>,
}

impl Check {
    pub(crate) fn ok(id: CheckId, detail: impl Into<String>) -> Self {
        Self {
            id,
            severity: Severity::Ok,
            detail: detail.into(),
            remedy: None,
            steps: None,
        }
    }

    pub(crate) fn warn(id: CheckId, detail: impl Into<String>, remedy: Option<String>) -> Self {
        Self {
            id,
            severity: Severity::Warning,
            detail: detail.into(),
            remedy,
            steps: None,
        }
    }

    pub(crate) fn critical(id: CheckId, detail: impl Into<String>, remedy: Option<String>) -> Self {
        Self {
            id,
            severity: Severity::Critical,
            detail: detail.into(),
            remedy,
            steps: None,
        }
    }

    /// Attach instructions for a fix that is not a command.
    pub(crate) fn with_steps(mut self, steps: Steps) -> Self {
        self.steps = Some(steps);
        self
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
        None => Check::warn(id, crate::fl!("observed-unknown"), None),
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
        None => Check::warn(id, crate::fl!("observed-unknown"), None),
    }
}

/// Polkit action guarding a decision about a plugged-in device.
const ACTION_APPLY_POLICY: &str = "org.usbguard.Devices1.applyDevicePolicy";

/// Polkit action guarding rule removal, which is what undoing a decision needs.
const ACTION_REMOVE_RULE: &str = "org.usbguard.Policy1.removeRule";

/// A polkit answer about one of the calls this app makes, as a check.
///
/// Asked of Polkit rather than of USBGuard, because there is no harmless way
/// to ask USBGuard: the calls change policy, so probing with the real one
/// would mean making a decision — or deleting a rule — to find out whether
/// that is allowed.
///
/// The three outcomes are genuinely different and are reported as such:
/// allowed outright, allowed after authenticating (workable, so a warning),
/// and refused.
async fn polkit_permission(connection: &zbus::Connection, id: CheckId, action: &str) -> Check {
    match polkit_check(connection, action).await {
        Ok(Authorization::Granted) => Check::ok(id, crate::fl!("observed-granted")),
        Ok(Authorization::Challenge) => Check::warn(
            id,
            crate::fl!("observed-password"),
            Some(POLKIT_REMEDY.to_string()),
        ),
        Ok(Authorization::Refused) => Check::critical(
            id,
            crate::fl!("observed-refused"),
            Some(POLKIT_REMEDY.to_string()),
        ),
        Err(detail) => Check::warn(id, detail, None),
    }
}

/// What Polkit would say if this session attempted an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authorization {
    /// Allowed outright.
    Granted,
    /// Allowed once the user authenticates.
    Challenge,
    /// Not allowed.
    Refused,
}

/// Ask Polkit whether this connection may perform `action`.
///
/// `flags` is 0 — no user interaction — so asking never raises a dialog of its
/// own. `Err` carries a short description of why Polkit could not be asked.
pub async fn polkit_check(
    connection: &zbus::Connection,
    action: &str,
) -> Result<Authorization, String> {
    let Some(name) = connection.unique_name() else {
        return Err(crate::fl!("observed-unknown"));
    };

    // No Polkit at all: nothing to report, since the service's own
    // authorization is then whatever it is.
    let polkit = PolkitProxy::new(connection)
        .await
        .map_err(|_| crate::fl!("observed-polkit-unavailable"))?;

    let subject = (
        "system-bus-name",
        HashMap::from([("name", zbus::zvariant::Value::from(name.as_str()))]),
    );

    match polkit
        .check_authorization(&subject, action, HashMap::new(), 0, "")
        .await
    {
        Ok((true, _, _)) => Ok(Authorization::Granted),
        Ok((false, true, _)) => Ok(Authorization::Challenge),
        Ok((false, false, _)) => Ok(Authorization::Refused),
        Err(e) => Err(e.to_string()),
    }
}

/// Shell command that grants the three calls this app makes — making a
/// decision, writing a rule, and removing one — to active local members of
/// the administrator and `plugdev` groups.
///
/// Debian and Ubuntu already grant the first two to `sudo` and `plugdev`; the
/// upstream defaults ask for an administrator password for all three, and
/// removal is left at that everywhere. Granting removal is not a widening in
/// practice: `appendRule` takes an insert position, so anyone who may append
/// a rule can put an `allow` at the top of the policy.
///
/// Written as a drop-in file rather than an edit to the packaged rules, so a
/// package upgrade cannot silently revert it and nothing already there is
/// disturbed. `50-` sorts before the shipped `org.usbguard1.rules`; Polkit
/// takes the first rule that returns a result. The file an earlier version
/// suggested, which granted removal alone, is removed so the two cannot
/// disagree.
///
/// `install -m 0644`, not `sudo tee`. polkitd runs as its own `polkitd` user
/// and has to be able to read the file, and `tee` creates it with whatever
/// mode the caller's umask leaves — under a umask of `0007`, sudo's combined
/// umask makes it `0640 root:root`, which polkitd cannot open. It logs only
/// "Error loading script", the rule silently never applies, and the command
/// itself printed nothing to suggest anything went wrong. `install` sets the
/// mode explicitly, and replacing the file is also what makes polkit reload.
pub(crate) const POLKIT_REMEDY: &str = concat!(
    "sudo install -m 0644 /dev/stdin /etc/polkit-1/rules.d/50-usbguard-desktop.rules <<'EOF'",
    " && sudo rm -f /etc/polkit-1/rules.d/50-usbguard-remove-rule.rules\n",
    "polkit.addRule(function(action, subject) {\n",
    "    if ((action.id == \"org.usbguard.Devices1.applyDevicePolicy\" ||\n",
    "         action.id == \"org.usbguard.Policy1.appendRule\" ||\n",
    "         action.id == \"org.usbguard.Policy1.removeRule\") &&\n",
    "        subject.active && subject.local &&\n",
    "        (subject.isInGroup(\"plugdev\") || subject.isInGroup(\"sudo\") ||\n",
    "         subject.isInGroup(\"wheel\"))) {\n",
    "        return polkit.Result.YES;\n",
    "    }\n",
    "});\nEOF"
);

/// Command that writes a policy from the devices plugged in now and makes the
/// daemon use it.
///
/// `set-parameter` and a rewritten rules file both stop at the running
/// daemon or the file respectively; the restart is what puts the new policy in
/// force.
const EMPTY_POLICY_REMEDY: &str = concat!(
    "sudo sh -c 'usbguard generate-policy > /etc/usbguard/rules.conf",
    " && chmod 0600 /etc/usbguard/rules.conf'",
    " && sudo systemctl restart usbguard.service"
);

/// Run every check against the running system.
///
/// `client` may be `None` when a connection could not be established at all;
/// the systemd-derived checks still run, which is usually what explains it.
///
/// `rules` is the policy as the refresh read it, or `None` when it could not
/// be read. Passing it in rather than reading it again here keeps health from
/// making its own policy call on every tick, past the latch that stops a
/// refused call from being retried until the user asks.
pub async fn evaluate(
    client: Option<&Client>,
    connection: Option<&zbus::Connection>,
    rules: Option<&[PolicyRule]>,
) -> Health {
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
            crate::fl!("observed-no-bus"),
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
            checks.push(Check::ok(
                CheckId::IpcReachable,
                crate::fl!("observed-responding"),
            ));
            checks.push(inserted_policy_check(&value));
        }
        Err(Error::PermissionDenied(_)) => {
            // Reaching the daemon at all proves it is running; this session is
            // just not allowed to read from it. `usbguard-dbus` runs as root,
            // so USBGuard's own IPC allow-list (`usbguard add-user`) does not
            // come into it: this is Polkit, which grants USBGuard's read calls
            // to the active local session and refuses everything else.
            checks.push(Check::ok(
                CheckId::IpcReachable,
                crate::fl!("observed-responding"),
            ));
            checks.push(
                Check::critical(CheckId::IpcPermission, crate::fl!("observed-refused"), None)
                    .with_steps(Steps::InactiveSession),
            );
            debug_log!(crate::debug::HEALTH, "IPC reachable but reads refused");
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

    // Making and undoing a decision are separate Polkit actions, and either
    // can be refused while reading works. Both are invisible until someone
    // presses a button, so they are asked about here.
    if let Some(connection) = connection {
        checks
            .push(polkit_permission(connection, CheckId::IpcPermission, ACTION_APPLY_POLICY).await);
        checks.push(
            polkit_permission(connection, CheckId::DecisionsReversible, ACTION_REMOVE_RULE).await,
        );
    }

    checks.push(policy_check(rules));

    let health = Health { checks };
    debug_log!(
        crate::debug::HEALTH,
        "health evaluated: worst={:?}",
        health.worst()
    );
    health
}

/// Judge the policy as read: empty, populated, or unreadable.
///
/// Reading the policy needs its own Polkit action, which a user may lack
/// while still being able to make device decisions, so unreadable is a
/// warning rather than a failure.
fn policy_check(rules: Option<&[PolicyRule]>) -> Check {
    match rules {
        Some([]) => Check::warn(
            CheckId::PolicyNotEmpty,
            crate::fl!("observed-rules", count = 0),
            // Restarted, because the daemon reads the file only at start.
            // Shown with a warning (`format::remedy_caution`): this allows
            // every device plugged in at the moment it runs.
            Some(EMPTY_POLICY_REMEDY.into()),
        ),
        Some(rules) => Check::ok(
            CheckId::PolicyNotEmpty,
            crate::fl!("observed-rules", count = rules.len()),
        ),
        None => Check::warn(
            CheckId::PolicyNotEmpty,
            crate::fl!("observed-unreadable"),
            None,
        ),
    }
}

/// Judge `InsertedDevicePolicy`, which decides what happens to a device
/// between it being plugged in and the user deciding about it.
fn inserted_policy_check(value: &str) -> Check {
    let normalized = value.trim().to_lowercase();
    if normalized == INSERTED_POLICY_PREFERRED {
        Check::ok(CheckId::InsertedDevicePolicy, normalized)
    } else if INSERTED_POLICY_UNSAFE.contains(&normalized.as_str()) {
        // The device is authorized the moment it is plugged in, so a prompt
        // would be asking permission for something that already happened.
        Check::critical(
            CheckId::InsertedDevicePolicy,
            normalized,
            Some(format!(
                "sudo usbguard set-parameter {PARAM_INSERTED_DEVICE_POLICY} {INSERTED_POLICY_PREFERRED} && sudo sed -i 's/^{PARAM_INSERTED_DEVICE_POLICY}=.*/{PARAM_INSERTED_DEVICE_POLICY}={INSERTED_POLICY_PREFERRED}/' /etc/usbguard/usbguard-daemon.conf"
            )),
        )
    } else {
        // `block` and `reject` are safe but skip the policy, so a device the
        // user already allowed permanently is still blocked on insert.
        Check::warn(
            CheckId::InsertedDevicePolicy,
            normalized,
            Some(format!(
                "sudo usbguard set-parameter {PARAM_INSERTED_DEVICE_POLICY} {INSERTED_POLICY_PREFERRED} && sudo sed -i 's/^{PARAM_INSERTED_DEVICE_POLICY}=.*/{PARAM_INSERTED_DEVICE_POLICY}={INSERTED_POLICY_PREFERRED}/' /etc/usbguard/usbguard-daemon.conf"
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_polkit_remedy_writes_a_file_polkitd_can_read() {
        // polkitd is not root. A file created through the caller's umask can
        // come out unreadable to it, and then the rule never loads.
        assert!(
            POLKIT_REMEDY.starts_with("sudo install -m 0644 /dev/stdin "),
            "the remedy must set the file mode itself: {POLKIT_REMEDY}"
        );
        assert!(!POLKIT_REMEDY.contains("tee"));
    }

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
