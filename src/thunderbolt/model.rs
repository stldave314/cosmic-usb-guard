// SPDX-License-Identifier: GPL-3.0-or-later

//! Thunderbolt/USB4 devices and domains as bolt and the kernel describe them.
//!
//! bolt reports everything as strings and D-Bus variants. They are turned
//! into enums here, once, so nothing above this layer matches on prose — and
//! so a value bolt adds in a later release lands in an explicit `Unknown`
//! rather than being mistaken for something it is not.

use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

/// How a Thunderbolt domain decides which devices get PCIe access.
///
/// The kernel's names, from `/sys/bus/thunderbolt/devices/domainN/security`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Every device is connected automatically. No authorization at all.
    None,
    /// A device is connected only once something authorizes it.
    User,
    /// As `User`, plus a key challenge for devices seen before.
    Secure,
    /// Only DisplayPort is tunneled; PCIe never is.
    DpOnly,
    /// Only USB is tunneled; PCIe never is.
    UsbOnly,
    /// PCIe tunneling is disabled outright.
    NoPcie,
    /// A value this build does not know.
    Unknown(String),
}

impl SecurityLevel {
    /// Parse the kernel's or bolt's name for a level.
    pub fn parse(value: &str) -> Self {
        match value.trim() {
            "none" => Self::None,
            "user" => Self::User,
            "secure" => Self::Secure,
            "dponly" => Self::DpOnly,
            "usbonly" => Self::UsbOnly,
            "nopcie" => Self::NoPcie,
            other => Self::Unknown(other.to_string()),
        }
    }

    /// The name as the kernel spells it, for the observed-value line.
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::User => "user",
            Self::Secure => "secure",
            Self::DpOnly => "dponly",
            Self::UsbOnly => "usbonly",
            Self::NoPcie => "nopcie",
            Self::Unknown(value) => value,
        }
    }

    /// Whether a new device can reach PCIe without anyone authorizing it.
    pub fn connects_without_asking(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Whether devices are authorized one by one at this level, which is the
    /// only case where this app has anything to ask about.
    pub fn asks_per_device(&self) -> bool {
        matches!(self, Self::User | Self::Secure)
    }
}

/// Where a device is in bolt's connect-and-authorize cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Known to bolt but not plugged in.
    Disconnected,
    /// Being enumerated.
    Connecting,
    /// Plugged in and waiting for authorization. No PCIe access yet.
    Connected,
    /// Authorization is in progress.
    Authorizing,
    /// Authorization was attempted and failed.
    AuthError,
    /// Plugged in and authorized.
    Authorized,
    /// A value this build does not know.
    Unknown(String),
}

impl Status {
    /// Parse bolt's `Status` property.
    pub fn parse(value: &str) -> Self {
        match value {
            "disconnected" => Self::Disconnected,
            "connecting" => Self::Connecting,
            "connected" => Self::Connected,
            "authorizing" => Self::Authorizing,
            "auth-error" => Self::AuthError,
            "authorized" => Self::Authorized,
            other => Self::Unknown(other.to_string()),
        }
    }

    /// Whether the device is plugged in, whatever its authorization.
    pub fn is_present(&self) -> bool {
        !matches!(self, Self::Disconnected | Self::Unknown(_))
    }
}

/// What kind of Thunderbolt router a device is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The computer's own controller. Never something to authorize.
    Host,
    /// Anything plugged in: a dock, enclosure, display or cable.
    Peripheral,
    /// A value this build does not know.
    Unknown,
}

/// A Thunderbolt/USB4 device known to bolt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    /// bolt's stable identifier for the device.
    pub uid: String,
    /// Product name.
    pub name: String,
    /// Vendor name.
    pub vendor: String,
    /// A name the user gave it, if any.
    pub label: String,
    /// Host or peripheral.
    pub kind: Kind,
    /// Where it is in the authorization cycle.
    pub status: Status,
    /// Whether bolt remembers it, and so authorizes it on its own when its
    /// policy allows.
    pub stored: bool,
    /// bolt's policy for a remembered device: `auto`, `manual`, `iommu`.
    pub policy: String,
    /// UID of the domain it is attached to.
    pub domain: String,
}

impl Device {
    /// Build from the result of `org.freedesktop.DBus.Properties.GetAll`.
    pub fn from_properties(properties: &HashMap<String, OwnedValue>) -> Option<Self> {
        let uid = string(properties, "Uid")?;
        Some(Self {
            uid,
            name: string(properties, "Name").unwrap_or_default(),
            vendor: string(properties, "Vendor").unwrap_or_default(),
            label: string(properties, "Label").unwrap_or_default(),
            kind: match string(properties, "Type").as_deref() {
                Some("host") => Kind::Host,
                Some("peripheral") => Kind::Peripheral,
                _ => Kind::Unknown,
            },
            status: Status::parse(&string(properties, "Status").unwrap_or_default()),
            stored: boolean(properties, "Stored").unwrap_or(false),
            policy: string(properties, "Policy").unwrap_or_default(),
            domain: string(properties, "Domain").unwrap_or_default(),
        })
    }

    /// The name to show: the user's label, else vendor and product.
    pub fn display_name(&self) -> String {
        if !self.label.trim().is_empty() {
            return self.label.trim().to_string();
        }
        let name = self.name.trim();
        let vendor = self.vendor.trim();
        match (vendor.is_empty(), name.is_empty()) {
            (false, false) if !name.starts_with(vendor) => format!("{vendor} {name}"),
            (_, false) => name.to_string(),
            (false, true) => vendor.to_string(),
            (true, true) => self.uid.clone(),
        }
    }

    /// Whether this is something the user plugged in, as opposed to the
    /// computer's own controller.
    pub fn is_peripheral(&self) -> bool {
        self.kind != Kind::Host
    }

    /// Whether the device is plugged in and has no PCIe access yet.
    pub fn is_waiting(&self) -> bool {
        self.is_peripheral() && matches!(self.status, Status::Connected | Status::AuthError)
    }

    /// Whether "allow once" applies: plugged in and not yet authorized.
    pub fn can_authorize(&self) -> bool {
        self.is_waiting()
    }

    /// Whether "always allow" applies: plugged in and not yet remembered.
    ///
    /// bolt enrolls only a present device, since enrolling authorizes it too.
    pub fn can_enroll(&self) -> bool {
        self.is_peripheral() && !self.stored && self.status.is_present()
    }

    /// Whether "forget" applies: bolt remembers it.
    pub fn can_forget(&self) -> bool {
        self.is_peripheral() && self.stored
    }
}

/// A Thunderbolt domain: one host controller and what hangs off it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    /// bolt's identifier for the domain.
    pub uid: String,
    /// Kernel name, e.g. `domain0`.
    pub id: String,
    /// Its security level.
    pub security: SecurityLevel,
    /// Whether the IOMMU protects memory from its devices.
    pub iommu: bool,
}

impl Domain {
    /// Build from the result of `org.freedesktop.DBus.Properties.GetAll`.
    pub fn from_properties(properties: &HashMap<String, OwnedValue>) -> Option<Self> {
        Some(Self {
            uid: string(properties, "Uid")?,
            id: string(properties, "Id").unwrap_or_default(),
            security: SecurityLevel::parse(
                &string(properties, "SecurityLevel").unwrap_or_default(),
            ),
            iommu: boolean(properties, "IOMMU").unwrap_or(false),
        })
    }
}

/// bolt's daemon-wide settings that change what the user sees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manager {
    /// Whether bolt authorizes remembered devices by itself. `enabled` or
    /// `disabled`, possibly with flags after a `|`.
    pub auth_mode: String,
}

impl Manager {
    /// Build from the result of `org.freedesktop.DBus.Properties.GetAll`.
    pub fn from_properties(properties: &HashMap<String, OwnedValue>) -> Self {
        Self {
            auth_mode: string(properties, "AuthMode").unwrap_or_default(),
        }
    }

    /// Whether remembered devices are authorized without being asked.
    ///
    /// When this is off, "always allow" still records the device, but it
    /// waits like a new one every time it is connected.
    pub fn authorizes_automatically(&self) -> bool {
        !self
            .auth_mode
            .split('|')
            .any(|flag| flag.trim() == "disabled")
    }
}

/// Everything known about Thunderbolt at one moment.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// bolt's own settings, when bolt answered.
    pub manager: Option<Manager>,
    /// Domains as bolt reports them.
    pub domains: Vec<Domain>,
    /// Every device bolt knows, connected or remembered.
    pub devices: Vec<Device>,
    /// Why bolt could not be read, when it could not.
    pub error: Option<String>,
}

impl Snapshot {
    /// Devices worth showing: everything except the host controllers.
    pub fn peripherals(&self) -> impl Iterator<Item = &Device> {
        self.devices.iter().filter(|d| d.is_peripheral())
    }

    /// Whether the controller a device hangs off authorizes devices one by
    /// one, so that allowing this one means anything.
    ///
    /// Unknown domain: yes, failing towards asking.
    pub fn asks_about(&self, device: &Device) -> bool {
        self.domains
            .iter()
            .find(|domain| domain.uid == device.domain)
            .is_none_or(|domain| domain.security.asks_per_device())
    }

    /// Look a device up by bolt UID.
    pub fn device(&self, uid: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.uid == uid)
    }
}

fn string(properties: &HashMap<String, OwnedValue>, name: &str) -> Option<String> {
    let value = properties.get(name)?;
    <&str>::try_from(value).ok().map(str::to_string)
}

fn boolean(properties: &HashMap<String, OwnedValue>, name: &str) -> Option<bool> {
    bool::try_from(properties.get(name)?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::Value;

    fn props(pairs: &[(&str, Value<'_>)]) -> HashMap<String, OwnedValue> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.try_to_owned().unwrap()))
            .collect()
    }

    fn dock(status: &str, stored: bool) -> Device {
        Device::from_properties(&props(&[
            ("Uid", Value::from("d0c4-uid")),
            ("Name", Value::from("Thunderbolt Dock")),
            ("Vendor", Value::from("Acme")),
            ("Type", Value::from("peripheral")),
            ("Status", Value::from(status)),
            ("Stored", Value::from(stored)),
            ("Policy", Value::from("auto")),
        ]))
        .unwrap()
    }

    #[test]
    fn parses_this_machines_host_controller_as_bolt_reports_it() {
        // Values captured from bolt on a ThinkPad with a Lunar Lake TB4 host.
        let host = Device::from_properties(&props(&[
            ("Uid", Value::from("e6888780-3014-0f2c-ffff-ffffffffffff")),
            ("Name", Value::from("Gen16")),
            ("Vendor", Value::from("Intel")),
            ("Label", Value::from("Intel Gen16")),
            ("Type", Value::from("host")),
            ("Status", Value::from("authorized")),
            ("Stored", Value::from(false)),
            ("Policy", Value::from("default")),
            (
                "Domain",
                Value::from("e6888780-3014-0f2c-ffff-ffffffffffff"),
            ),
        ]))
        .unwrap();

        assert_eq!(host.kind, Kind::Host);
        assert_eq!(host.status, Status::Authorized);
        assert_eq!(host.display_name(), "Intel Gen16");
        // The computer's own controller is never offered as a device.
        assert!(!host.is_peripheral());
        assert!(!host.can_authorize() && !host.can_enroll() && !host.can_forget());
    }

    #[test]
    fn a_new_device_waiting_for_authorization_offers_both_kinds_of_allow() {
        let waiting = dock("connected", false);
        assert!(waiting.is_waiting());
        assert!(waiting.can_authorize());
        assert!(waiting.can_enroll());
        assert!(!waiting.can_forget());
    }

    #[test]
    fn a_remembered_device_can_be_forgotten_even_when_unplugged() {
        let away = dock("disconnected", true);
        assert!(!away.is_waiting());
        assert!(!away.can_authorize());
        // Enrolling needs the device present; bolt enrolls by authorizing.
        assert!(!away.can_enroll());
        assert!(away.can_forget());
    }

    #[test]
    fn an_authorized_device_is_not_asked_about() {
        let working = dock("authorized", true);
        assert!(!working.is_waiting());
        assert!(!working.can_authorize());
    }

    #[test]
    fn a_failed_authorization_can_be_retried() {
        assert!(dock("auth-error", false).can_authorize());
    }

    #[test]
    fn display_name_avoids_repeating_the_vendor() {
        let mut device = dock("connected", false);
        assert_eq!(device.display_name(), "Acme Thunderbolt Dock");
        device.name = "Acme Thunderbolt Dock".into();
        assert_eq!(device.display_name(), "Acme Thunderbolt Dock");
        device.name.clear();
        device.vendor.clear();
        assert_eq!(device.display_name(), "d0c4-uid");
    }

    #[test]
    fn security_levels_are_classified_by_what_they_let_through() {
        assert!(SecurityLevel::parse("none").connects_without_asking());
        for level in ["user", "secure", "dponly", "usbonly", "nopcie"] {
            assert!(
                !SecurityLevel::parse(level).connects_without_asking(),
                "{level}"
            );
        }
        assert!(SecurityLevel::parse("user").asks_per_device());
        assert!(!SecurityLevel::parse("dponly").asks_per_device());
        assert_eq!(
            SecurityLevel::parse("future"),
            SecurityLevel::Unknown("future".into())
        );
        // Sysfs values end in a newline.
        assert_eq!(SecurityLevel::parse("user\n"), SecurityLevel::User);
    }

    #[test]
    fn automatic_authorization_is_off_only_when_bolt_says_disabled() {
        let mode = |m: &str| Manager {
            auth_mode: m.into(),
        };
        assert!(mode("enabled").authorizes_automatically());
        assert!(!mode("disabled").authorizes_automatically());
        assert!(!mode("enabled|disabled").authorizes_automatically());
    }
}
