// SPDX-License-Identifier: GPL-3.0-or-later

//! Icon names.
//!
//! Every name here was checked against the icon themes shipped with COSMIC —
//! a missing icon renders as a blank square with no error, so guessing at
//! plausible names is how an applet ends up with an invisible indicator.
//! `tests/icons.rs` re-checks them against the installed themes.

use crate::usbguard::{Device, Health, Severity};

/// Generic USB device.
pub const DEVICE: &str = "drive-harddisk-usb-symbolic";
/// Mass storage.
pub const STORAGE: &str = "media-removable-symbolic";
/// Keyboard, mouse, or anything else that can generate input.
pub const INPUT: &str = "input-keyboard-symbolic";
/// Network adapter.
pub const NETWORK: &str = "network-wired-symbolic";
/// Audio device.
pub const AUDIO: &str = "audio-card-usb-symbolic";
/// Camera or scanner.
pub const CAMERA: &str = "camera-web-symbolic";
/// Printer.
pub const PRINTER: &str = "printer-symbolic";

/// Everything is as it should be.
pub const OK: &str = "emblem-ok-symbolic";
/// Something needs attention but protection is intact.
pub const WARNING: &str = "dialog-warning-symbolic";
/// Protection is not in place.
pub const ERROR: &str = "dialog-error-symbolic";
/// A decision is pending.
pub const QUESTION: &str = "dialog-question-symbolic";
/// A device is blocked.
pub const BLOCKED: &str = "changes-prevent-symbolic";
/// Remove or revoke.
pub const REMOVE: &str = "list-remove-symbolic";
/// Copy to clipboard.
pub const COPY: &str = "edit-copy-symbolic";
/// Reload.
pub const REFRESH: &str = "view-refresh-symbolic";

/// Panel icon: protecting, nothing outstanding.
pub const PANEL_OK: &str = "security-high-symbolic";
/// Panel icon: running, but something wants attention.
pub const PANEL_WARNING: &str = "security-medium-symbolic";
/// Panel icon: not protecting this machine.
pub const PANEL_CRITICAL: &str = "security-low-symbolic";

/// The icon that best describes what a device *is*.
///
/// Ordered by how much the class matters to a security decision, not by what
/// the device is mostly used for: a headset that also claims a keyboard
/// interface is shown as an input device, because that is the interface worth
/// noticing.
pub fn for_device(device: &Device) -> &'static str {
    if device.is_input_capable() {
        return INPUT;
    }
    if device.is_network() {
        return NETWORK;
    }
    if device.is_storage() {
        return STORAGE;
    }
    for interface in &device.interfaces {
        match interface.class {
            0x01 => return AUDIO,
            0x06 | 0x0e => return CAMERA,
            0x07 => return PRINTER,
            _ => {}
        }
    }
    DEVICE
}

/// The panel indicator icon for the current state.
///
/// `pending` is the number of devices waiting for a decision. A pending
/// decision outranks a healthy install: the user needs to act either way, and
/// showing the calm icon while a device sits blocked would be misleading.
pub fn for_status(connected: bool, health: &Health, pending: usize) -> &'static str {
    if !connected {
        return PANEL_CRITICAL;
    }
    match health.worst() {
        Severity::Critical => PANEL_CRITICAL,
        Severity::Warning => PANEL_WARNING,
        Severity::Ok if pending > 0 => PANEL_WARNING,
        Severity::Ok => PANEL_OK,
    }
}

/// The icon for a health severity.
pub fn for_severity(severity: Severity) -> &'static str {
    match severity {
        Severity::Ok => OK,
        Severity::Warning => WARNING,
        Severity::Critical => ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usbguard::health::{Check, CheckId};

    fn device(rule: &str) -> Device {
        Device::from_rule(1, rule).unwrap()
    }

    fn health(severity: Severity) -> Health {
        let check = match severity {
            Severity::Ok => Check {
                id: CheckId::DaemonRunning,
                severity: Severity::Ok,
                detail: String::new(),
                remedy: None,
            },
            other => Check {
                id: CheckId::DaemonRunning,
                severity: other,
                detail: String::new(),
                remedy: None,
            },
        };
        Health {
            checks: vec![check],
        }
    }

    #[test]
    fn input_capability_outranks_the_devices_other_classes() {
        // A "headset" that also claims a keyboard interface is the classic
        // BadUSB shape; it must not be shown as a harmless audio device.
        let sneaky = device("block id 1234:5678 with-interface { 01:01:00 03:01:01 }");
        assert_eq!(for_device(&sneaky), INPUT);
    }

    #[test]
    fn picks_class_appropriate_icons() {
        assert_eq!(
            for_device(&device("allow with-interface 08:06:50")),
            STORAGE
        );
        assert_eq!(
            for_device(&device("allow with-interface 02:06:00")),
            NETWORK
        );
        assert_eq!(for_device(&device("allow with-interface 01:01:00")), AUDIO);
        assert_eq!(
            for_device(&device("allow with-interface 07:01:02")),
            PRINTER
        );
        assert_eq!(for_device(&device("allow with-interface 09:00:00")), DEVICE);
        assert_eq!(for_device(&device("allow")), DEVICE);
    }

    #[test]
    fn disconnected_always_shows_the_critical_icon() {
        // Whatever the last known health was, we cannot claim to be protecting
        // anything while we cannot see the daemon.
        assert_eq!(for_status(false, &health(Severity::Ok), 0), PANEL_CRITICAL);
    }

    #[test]
    fn a_pending_decision_is_never_shown_as_all_clear() {
        assert_eq!(for_status(true, &health(Severity::Ok), 1), PANEL_WARNING);
        assert_eq!(for_status(true, &health(Severity::Ok), 0), PANEL_OK);
    }

    #[test]
    fn health_severity_drives_the_panel_icon() {
        assert_eq!(
            for_status(true, &health(Severity::Critical), 0),
            PANEL_CRITICAL
        );
        assert_eq!(
            for_status(true, &health(Severity::Warning), 0),
            PANEL_WARNING
        );
    }
}
