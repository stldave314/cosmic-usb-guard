// SPDX-License-Identifier: GPL-3.0-or-later

//! Raw zbus proxies for bolt, the Thunderbolt device manager.
//!
//! Interface names, method names and signatures were taken from bolt's
//! introspection data (bolt 0.9, D-Bus API version 1). Names are spelled out
//! rather than derived, the same as for USBGuard, so a rename in the Rust
//! cannot silently turn into an `UnknownMethod` at runtime.
//!
//! Device and domain properties are read with a single
//! `org.freedesktop.DBus.Properties.GetAll` per object rather than through
//! generated property accessors: one round trip per object, and no property
//! cache that could hand back a status from before the last plug.

// See `usbguard::proxy`: the generated items carry no docs of their own.
#![allow(missing_docs)]

/// `org.freedesktop.bolt1.Manager` — the device list and enrollment.
#[zbus::proxy(
    interface = "org.freedesktop.bolt1.Manager",
    default_service = "org.freedesktop.bolt",
    default_path = "/org/freedesktop/bolt"
)]
pub trait Manager {
    /// Object paths of every device bolt knows, connected or stored.
    #[zbus(name = "ListDevices")]
    fn list_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Object paths of every domain.
    #[zbus(name = "ListDomains")]
    fn list_domains(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// The object path of the device with this UID.
    #[zbus(name = "DeviceByUid")]
    fn device_by_uid(&self, uid: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Authorize a device and remember it, with the given policy.
    #[zbus(name = "EnrollDevice")]
    fn enroll_device(
        &self,
        uid: &str,
        policy: &str,
        flags: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Stop remembering a device.
    #[zbus(name = "ForgetDevice")]
    fn forget_device(&self, uid: &str) -> zbus::Result<()>;

    /// A device became known.
    #[zbus(signal, name = "DeviceAdded")]
    fn device_added(&self, device: zbus::zvariant::OwnedObjectPath) -> zbus::Result<()>;

    /// A device stopped being known.
    #[zbus(signal, name = "DeviceRemoved")]
    fn device_removed(&self, device: zbus::zvariant::OwnedObjectPath) -> zbus::Result<()>;
}
