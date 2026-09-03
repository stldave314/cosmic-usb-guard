// SPDX-License-Identifier: GPL-3.0-or-later

//! Raw zbus proxies for the USBGuard D-Bus interfaces.
//!
//! Every method and signal name is spelled out explicitly. USBGuard uses
//! `lowerCamelCase` method names, whereas zbus would otherwise derive
//! `PascalCase` from the Rust identifier and the calls would fail at runtime
//! with `UnknownMethod`.
//!
//! The interface definitions here were taken from the introspection XML
//! embedded in `usbguard-dbus` (1.1.2).

// `zbus::proxy` expands each trait into proxy structs and signal-argument
// types that carry no doc comments of their own. The trait methods below are
// documented; this only exempts the generated items.
#![allow(missing_docs)]

use std::collections::HashMap;

/// `org.usbguard1` — daemon runtime parameters.
#[zbus::proxy(
    interface = "org.usbguard1",
    default_service = "org.usbguard1",
    default_path = "/org/usbguard1"
)]
pub trait UsbGuard {
    /// Read a runtime parameter, e.g. `InsertedDevicePolicy`.
    #[zbus(name = "getParameter")]
    fn get_parameter(&self, name: &str) -> zbus::Result<String>;

    /// Set a runtime parameter, returning the previous value.
    #[zbus(name = "setParameter")]
    fn set_parameter(&self, name: &str, value: &str) -> zbus::Result<String>;

    /// A runtime parameter changed.
    #[zbus(signal, name = "PropertyParameterChanged")]
    fn property_parameter_changed(
        &self,
        name: String,
        value_old: String,
        value_new: String,
    ) -> zbus::Result<()>;

    /// The daemon raised an exception.
    #[zbus(signal, name = "ExceptionMessage")]
    fn exception_message(
        &self,
        context: String,
        object: String,
        reason: String,
    ) -> zbus::Result<()>;
}

/// `org.usbguard.Devices1` — the device list and authorisation decisions.
#[zbus::proxy(
    interface = "org.usbguard.Devices1",
    default_service = "org.usbguard1",
    default_path = "/org/usbguard1/Devices"
)]
pub trait Devices {
    /// List devices matching `query`.
    ///
    /// `query` is a rule-shaped filter; `"match"` matches every device.
    /// Returns `(device_id, device_rule)` pairs.
    #[zbus(name = "listDevices")]
    fn list_devices(&self, query: &str) -> zbus::Result<Vec<(u32, String)>>;

    /// Authorise, de-authorise or reject a device.
    ///
    /// `target` uses USBGuard's numbering (see
    /// [`Target::to_dbus`](super::rule::Target::to_dbus)). When `permanent` is
    /// true the daemon also writes a matching rule into the policy, and
    /// returns its rule ID; otherwise the returned ID is meaningless.
    #[zbus(name = "applyDevicePolicy")]
    fn apply_device_policy(&self, id: u32, target: u32, permanent: bool) -> zbus::Result<u32>;

    /// A device appeared, changed or went away.
    #[zbus(signal, name = "DevicePresenceChanged")]
    fn device_presence_changed(
        &self,
        id: u32,
        event: u32,
        target: u32,
        device_rule: String,
        attributes: HashMap<String, String>,
    ) -> zbus::Result<()>;

    /// A device's authorisation changed.
    #[zbus(signal, name = "DevicePolicyChanged")]
    fn device_policy_changed(
        &self,
        id: u32,
        target_old: u32,
        target_new: u32,
        device_rule: String,
        rule_id: u32,
        attributes: HashMap<String, String>,
    ) -> zbus::Result<()>;

    /// A policy decision was applied to a device.
    #[zbus(signal, name = "DevicePolicyApplied")]
    fn device_policy_applied(
        &self,
        id: u32,
        target_new: u32,
        device_rule: String,
        rule_id: u32,
        attributes: HashMap<String, String>,
    ) -> zbus::Result<()>;
}

/// `org.usbguard.Policy1` — the persistent rule set.
#[zbus::proxy(
    interface = "org.usbguard.Policy1",
    default_service = "org.usbguard1",
    default_path = "/org/usbguard1/Policy"
)]
pub trait Policy {
    /// List policy rules whose label matches `label`; `""` lists all of them.
    #[zbus(name = "listRules")]
    fn list_rules(&self, label: &str) -> zbus::Result<Vec<(u32, String)>>;

    /// Append a rule, returning its new ID.
    ///
    /// `parent_id` of [`u32::MAX`] appends at the end. A `temporary` rule
    /// applies to the running daemon but is not written to the rules file.
    #[zbus(name = "appendRule")]
    fn append_rule(&self, rule: &str, parent_id: u32, temporary: bool) -> zbus::Result<u32>;

    /// Remove the rule with the given ID.
    #[zbus(name = "removeRule")]
    fn remove_rule(&self, id: u32) -> zbus::Result<()>;
}

/// Subset of `org.freedesktop.PolicyKit1.Authority`.
///
/// Used to ask, without side effects and without raising a dialog, whether
/// this session may perform a given USBGuard action. That question cannot be
/// answered by trying the action: `removeRule` has no dry run, and the only
/// way to discover it is refused would be to attempt a deletion.
#[zbus::proxy(
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_service = "org.freedesktop.PolicyKit1",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
pub trait Polkit {
    /// Whether `subject` is authorised for `action_id`.
    ///
    /// Returns `(authorized, challenge, details)`. `challenge` means the
    /// action is permitted but only after the user authenticates, which is a
    /// materially different answer from a flat refusal.
    #[zbus(name = "CheckAuthorization")]
    fn check_authorization(
        &self,
        subject: &(&str, HashMap<&str, zbus::zvariant::Value<'_>>),
        action_id: &str,
        details: HashMap<&str, &str>,
        flags: u32,
        cancellation_id: &str,
    ) -> zbus::Result<(bool, bool, HashMap<String, String>)>;
}

/// Subset of `org.freedesktop.systemd1.Manager` used for health checks.
///
/// Read-only, and reachable without authentication, so this tells us whether
/// USBGuard is actually running rather than merely installed.
#[zbus::proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait Systemd {
    /// Look up several units at once.
    ///
    /// Each tuple is `(name, description, load_state, active_state, sub_state,
    /// followed, object_path, job_id, job_type, job_path)`.
    #[allow(clippy::type_complexity)]
    fn list_units_by_names(
        &self,
        names: &[&str],
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            u32,
            String,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// Whether a unit is enabled, disabled, masked, static, and so on.
    fn get_unit_file_state(&self, name: &str) -> zbus::Result<String>;
}

/// `org.freedesktop.Notifications` — desktop notifications.
///
/// Used directly rather than through a helper crate so the app has one D-Bus
/// stack and one async runtime rather than two.
#[zbus::proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    /// Post a notification, returning its ID.
    ///
    /// Passing a previously returned ID as `replaces_id` updates that
    /// notification in place instead of stacking a new one.
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    /// Withdraw a notification by ID.
    fn close_notification(&self, id: u32) -> zbus::Result<()>;

    /// The user activated one of a notification's actions.
    #[zbus(signal)]
    fn action_invoked(&self, id: u32, action_key: String) -> zbus::Result<()>;

    /// A notification was dismissed or expired.
    #[zbus(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;
}
