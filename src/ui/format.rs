// SPDX-License-Identifier: GPL-3.0-or-later

//! Turning domain types into the strings the UI shows.
//!
//! Kept apart from the widgets so the wording can be tested without building
//! an iced tree.

use crate::constants::DESCRIPTION_ELIDE_CHARS;
use crate::fl;
use crate::journal::{Actor, Entry, Kind};
use crate::usbguard::health::CheckId;
use crate::usbguard::{Device, Severity, Target};

/// Label for a device's current authorisation.
pub fn target_label(target: Target) -> String {
    match target {
        Target::Allow => fl!("state-allowed"),
        Target::Block => fl!("state-blocked"),
        Target::Reject => fl!("state-rejected"),
        Target::Match | Target::Unknown => fl!("state-unknown"),
    }
}

/// Label for a journal event kind.
pub fn kind_label(kind: Kind) -> String {
    match kind {
        Kind::Inserted => fl!("event-inserted"),
        Kind::Removed => fl!("event-removed"),
        Kind::Updated => fl!("event-updated"),
        Kind::Allowed => fl!("event-allowed"),
        Kind::Blocked => fl!("event-blocked"),
        Kind::Rejected => fl!("event-rejected"),
        Kind::Revoked => fl!("event-revoked"),
        Kind::ServiceUp => fl!("event-service-up"),
        Kind::ServiceDown => fl!("event-service-down"),
        Kind::HealthProblem => fl!("event-health-problem"),
    }
}

/// Label describing who caused a journal entry.
pub fn actor_label(actor: Actor) -> String {
    match actor {
        Actor::User => fl!("actor-user"),
        Actor::Policy => fl!("actor-policy"),
        Actor::External => fl!("actor-external"),
        Actor::System => fl!("actor-system"),
    }
}

/// Human-readable name of a health check.
pub fn check_label(id: CheckId) -> String {
    match id {
        CheckId::DaemonRunning => fl!("check-daemon-running"),
        CheckId::DaemonEnabled => fl!("check-daemon-enabled"),
        CheckId::DbusRunning => fl!("check-dbus-running"),
        CheckId::DbusEnabled => fl!("check-dbus-enabled"),
        CheckId::IpcReachable => fl!("check-ipc-reachable"),
        CheckId::IpcPermission => fl!("check-ipc-permission"),
        CheckId::InsertedDevicePolicy => fl!("check-inserted-policy"),
        CheckId::PolicyNotEmpty => fl!("check-policy-not-empty"),
    }
}

/// Overall status headline for a severity.
pub fn status_headline(severity: Severity) -> String {
    match severity {
        Severity::Ok => fl!("status-ok"),
        Severity::Warning => fl!("status-warning"),
        Severity::Critical => fl!("status-critical"),
    }
}

/// One-line summary under a device's name: what it is and where it is plugged.
pub fn device_summary(device: &Device) -> String {
    let mut parts = Vec::new();

    let id = device.usb_id();
    if !id.is_empty() {
        parts.push(id);
    }

    let classes = device.interface_classes();
    if !classes.is_empty() {
        parts.push(classes.join(", "));
    }

    if !device.via_port.is_empty() {
        parts.push(device.via_port.clone());
    }

    elide(&parts.join(" · "), DESCRIPTION_ELIDE_CHARS)
}

/// The warnings that apply to a device, most serious first.
///
/// Input capability leads because it is the property that turns an unknown
/// device from unwanted into dangerous.
pub fn device_warnings(device: &Device) -> Vec<String> {
    let mut warnings = Vec::new();
    if device.is_input_capable() {
        warnings.push(fl!("warning-input-capable"));
    }
    if device.is_network() {
        warnings.push(fl!("warning-network"));
    }
    if device.is_storage() {
        warnings.push(fl!("warning-storage"));
    }
    warnings
}

/// Value for a device detail field, or a placeholder when the device reported
/// nothing.
pub fn field_or_placeholder(value: &str) -> String {
    if value.trim().is_empty() {
        fl!("field-none")
    } else {
        value.to_string()
    }
}

/// Timestamp for a journal entry, in the user's local time zone.
pub fn entry_time(entry: &Entry) -> String {
    entry.local_time().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Shorten `text` to at most `limit` characters, appending an ellipsis.
///
/// Counts characters rather than bytes so a multi-byte device name cannot be
/// cut mid-character.
pub fn elide(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let mut out: String = text.chars().take(limit.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(rule: &str) -> Device {
        Device::from_rule(1, rule).unwrap()
    }

    #[test]
    fn elide_counts_characters_not_bytes() {
        // Naive byte slicing here would panic or produce invalid UTF-8.
        let text = "ααααααααααα";
        assert_eq!(elide(text, 5).chars().count(), 5);
        assert!(elide(text, 5).ends_with('…'));
    }

    #[test]
    fn elide_leaves_short_text_alone() {
        assert_eq!(elide("short", 10), "short");
        assert_eq!(elide("exactly10!", 10), "exactly10!");
    }

    #[test]
    fn summary_joins_what_is_known_and_skips_what_is_not() {
        let full =
            device(r#"allow id 0781:5567 name "Stick" via-port "1-2" with-interface 08:06:50"#);
        let summary = device_summary(&full);
        assert!(summary.contains("0781:5567"));
        assert!(summary.contains("Mass storage"));
        assert!(summary.contains("1-2"));

        let bare = device("allow");
        assert_eq!(device_summary(&bare), "");
    }

    #[test]
    fn input_warning_comes_first() {
        let both = device("block id 1:2 with-interface { 08:06:50 03:01:01 }");
        let warnings = device_warnings(&both);
        assert_eq!(warnings.len(), 2);
        assert!(
            warnings[0].contains("keyboard"),
            "input warning should lead: {warnings:?}"
        );
    }

    #[test]
    fn a_plain_device_has_no_warnings() {
        assert!(device_warnings(&device("allow with-interface 09:00:00")).is_empty());
    }

    #[test]
    fn empty_fields_get_a_placeholder_rather_than_a_blank() {
        assert_ne!(field_or_placeholder("  "), "  ");
        assert!(!field_or_placeholder("").is_empty());
        assert_eq!(field_or_placeholder("AA123"), "AA123");
    }

    #[test]
    fn every_target_and_kind_has_a_label() {
        // A missing arm would show an empty string in the UI rather than fail.
        for target in [
            Target::Allow,
            Target::Block,
            Target::Reject,
            Target::Match,
            Target::Unknown,
        ] {
            assert!(!target_label(target).is_empty(), "{target:?}");
        }

        for kind in [
            Kind::Inserted,
            Kind::Removed,
            Kind::Updated,
            Kind::Allowed,
            Kind::Blocked,
            Kind::Rejected,
            Kind::Revoked,
            Kind::ServiceUp,
            Kind::ServiceDown,
            Kind::HealthProblem,
        ] {
            assert!(!kind_label(kind).is_empty(), "{kind:?}");
        }

        for id in [
            CheckId::DaemonRunning,
            CheckId::DaemonEnabled,
            CheckId::DbusRunning,
            CheckId::DbusEnabled,
            CheckId::IpcReachable,
            CheckId::IpcPermission,
            CheckId::InsertedDevicePolicy,
            CheckId::PolicyNotEmpty,
        ] {
            assert!(!check_label(id).is_empty(), "{id:?}");
        }
    }
}
