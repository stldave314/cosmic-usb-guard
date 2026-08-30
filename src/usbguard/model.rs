// SPDX-License-Identifier: GPL-3.0-or-later

//! Device and policy types built on top of parsed [`Rule`]s.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::rule::{ParseError, Rule, Target};

/// Why a device is in the state it is in, as far as we can tell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresenceEvent {
    /// Device was already present when we connected.
    Present,
    /// Device was just plugged in.
    Insert,
    /// Device's descriptors changed.
    Update,
    /// Device was unplugged.
    Remove,
    /// An event code this build does not recognise.
    Unknown,
}

impl PresenceEvent {
    /// Convert from the integer used by the `DevicePresenceChanged` signal.
    pub fn from_dbus(value: u32) -> Self {
        match value {
            0 => Self::Present,
            1 => Self::Insert,
            2 => Self::Update,
            3 => Self::Remove,
            _ => Self::Unknown,
        }
    }
}

/// A USB interface descriptor triple: class, subclass, protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interface {
    /// USB class code.
    pub class: u8,
    /// USB subclass code.
    pub subclass: u8,
    /// USB protocol code.
    pub protocol: u8,
}

impl Interface {
    /// Parse the `CC:SS:PP` form used in rule strings.
    ///
    /// USBGuard also permits `*` for a wildcard subclass or protocol; those
    /// parse as `0xFF`, which is the USB "vendor specific" code, so callers
    /// should not read meaning into a wildcard beyond the class.
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split(':');
        let class = u8::from_str_radix(parts.next()?, 16).ok()?;
        let subclass = parts
            .next()
            .and_then(|p| u8::from_str_radix(p, 16).ok())
            .unwrap_or(0);
        let protocol = parts
            .next()
            .and_then(|p| u8::from_str_radix(p, 16).ok())
            .unwrap_or(0);
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            class,
            subclass,
            protocol,
        })
    }

    /// Human-readable name of the USB device class.
    ///
    /// Codes are from the USB-IF "Defined Class Codes" list. This is
    /// deliberately a static table rather than a `usb.ids` lookup: it is small,
    /// stable, and means the app has no runtime data-file dependency.
    pub fn class_name(&self) -> &'static str {
        match self.class {
            0x00 => "Per-interface",
            0x01 => "Audio",
            0x02 => "Communications",
            0x03 => "Human interface device",
            0x05 => "Physical",
            0x06 => "Image",
            0x07 => "Printer",
            0x08 => "Mass storage",
            0x09 => "Hub",
            0x0a => "CDC data",
            0x0b => "Smart card",
            0x0d => "Content security",
            0x0e => "Video",
            0x0f => "Personal healthcare",
            0x10 => "Audio/video",
            0x11 => "Billboard",
            0x12 => "USB-C bridge",
            0x3c => "I3C",
            0xdc => "Diagnostic",
            0xe0 => "Wireless controller",
            0xef => "Miscellaneous",
            0xfe => "Application specific",
            0xff => "Vendor specific",
            _ => "Unknown",
        }
    }

    /// Whether this interface class can act as a keyboard or pointer, and so
    /// could be used to inject keystrokes ("BadUSB").
    ///
    /// HID boot-protocol subclass 1 covers keyboard (protocol 1) and mouse
    /// (protocol 2); other HID interfaces still speak the same bus and are
    /// treated as input-capable.
    pub fn is_input_capable(&self) -> bool {
        self.class == 0x03
    }

    /// Whether this interface class exposes storage.
    pub fn is_storage(&self) -> bool {
        self.class == 0x08
    }

    /// Whether this interface class is a network adapter.
    pub fn is_network(&self) -> bool {
        // Communications, CDC data, and wireless controllers can all present
        // a network interface the host will route through.
        matches!(self.class, 0x02 | 0x0a | 0xe0)
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}",
            self.class, self.subclass, self.protocol
        )
    }
}

/// A USB device as USBGuard sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    /// Daemon-assigned device ID. Stable only while the device stays plugged in.
    pub id: u32,
    /// Current authorisation target.
    pub target: Target,
    /// Vendor ID, as four lowercase hex digits.
    pub vendor_id: String,
    /// Product ID, as four lowercase hex digits.
    pub product_id: String,
    /// Product name reported by the device. May be empty.
    pub name: String,
    /// Serial number reported by the device. May be empty.
    pub serial: String,
    /// Base64 hash of the device descriptors. This is USBGuard's durable
    /// identity for a device and is what permanent rules should key on.
    pub hash: String,
    /// Hash of the parent hub.
    pub parent_hash: String,
    /// Physical port path, e.g. `1-1.2`.
    pub via_port: String,
    /// How the device was connected, e.g. `hotplug` or `hardwired`.
    pub connect_type: String,
    /// Interfaces the device exposes.
    pub interfaces: Vec<Interface>,
    /// The rule string the daemon gave us, kept verbatim.
    pub rule: String,
}

impl Device {
    /// Build a device from the `(id, rule)` pair returned by `listDevices`.
    pub fn from_rule(id: u32, rule_text: &str) -> Result<Self, ParseError> {
        let rule = Rule::parse(rule_text)?;

        let (vendor_id, product_id) = rule
            .attribute("id")
            .and_then(|id| id.split_once(':'))
            .map(|(v, p)| (v.to_lowercase(), p.to_lowercase()))
            .unwrap_or_default();

        let interfaces = rule
            .attribute_values("with-interface")
            .into_iter()
            .filter_map(Interface::parse)
            .collect();

        Ok(Self {
            id,
            target: rule.target,
            vendor_id,
            product_id,
            name: rule.attribute("name").unwrap_or_default().to_string(),
            serial: rule.attribute("serial").unwrap_or_default().to_string(),
            hash: rule.attribute("hash").unwrap_or_default().to_string(),
            parent_hash: rule
                .attribute("parent-hash")
                .unwrap_or_default()
                .to_string(),
            via_port: rule.attribute("via-port").unwrap_or_default().to_string(),
            connect_type: rule
                .attribute("with-connect-type")
                .unwrap_or_default()
                .to_string(),
            interfaces,
            rule: rule_text.to_string(),
        })
    }

    /// `vvvv:pppp`, the conventional way to write a USB ID.
    pub fn usb_id(&self) -> String {
        if self.vendor_id.is_empty() && self.product_id.is_empty() {
            String::new()
        } else {
            format!("{}:{}", self.vendor_id, self.product_id)
        }
    }

    /// The best human-readable label available for this device.
    ///
    /// Devices routinely report an empty name, so fall back through the USB ID
    /// to the port path rather than showing a blank row.
    pub fn display_name(&self) -> String {
        if !self.name.trim().is_empty() {
            return self.name.clone();
        }
        let id = self.usb_id();
        if !id.is_empty() {
            return id;
        }
        if !self.via_port.is_empty() {
            return format!("port {}", self.via_port);
        }
        format!("device {}", self.id)
    }

    /// Distinct interface class names, in first-seen order.
    pub fn interface_classes(&self) -> Vec<&'static str> {
        let mut seen = Vec::new();
        for interface in &self.interfaces {
            let name = interface.class_name();
            if !seen.contains(&name) {
                seen.push(name);
            }
        }
        seen
    }

    /// Whether the device claims a human-interface class and could therefore
    /// type on the user's behalf.
    pub fn is_input_capable(&self) -> bool {
        self.interfaces.iter().any(Interface::is_input_capable)
    }

    /// Whether the device exposes storage.
    pub fn is_storage(&self) -> bool {
        self.interfaces.iter().any(Interface::is_storage)
    }

    /// Whether the device presents a network interface.
    pub fn is_network(&self) -> bool {
        self.interfaces.iter().any(Interface::is_network)
    }

    /// Whether this looks like a root hub, which is part of the machine rather
    /// than something the user plugged in.
    ///
    /// Root hubs use the Linux Foundation vendor ID and are hardwired.
    pub fn is_root_hub(&self) -> bool {
        self.vendor_id == "1d6b"
    }

    /// Whether the device is soldered in or otherwise not user-pluggable.
    pub fn is_hardwired(&self) -> bool {
        self.connect_type == "hardwired"
    }

    /// A permanent USBGuard rule that matches *this specific device* by its
    /// descriptor hash.
    ///
    /// Keying on the hash rather than the USB ID matters: a rule written as
    /// `allow id 0781:5567` authorises every device that claims those IDs,
    /// which is exactly the property a spoofing device relies on.
    pub fn permanent_rule(&self, target: Target) -> Option<String> {
        if self.hash.is_empty() {
            return None;
        }
        Some(format!(
            "{} hash {}",
            target.keyword(),
            super::rule::quote(&self.hash)
        ))
    }
}

/// A rule in the daemon's policy, with the ID needed to remove it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    /// Daemon-assigned rule ID.
    pub id: u32,
    /// The parsed rule.
    pub rule: Rule,
}

impl PolicyRule {
    /// Build from the `(id, rule)` pair returned by `listRules`.
    pub fn from_pair(id: u32, rule_text: &str) -> Result<Self, ParseError> {
        Ok(Self {
            id,
            rule: Rule::parse(rule_text)?,
        })
    }

    /// The device hash this rule pins to, if it is a hash rule.
    pub fn hash(&self) -> Option<&str> {
        self.rule.attribute("hash")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEYBOARD: &str = concat!(
        r#"block id 046d:c31c serial "" name "USB Keyboard" "#,
        r#"hash "AbCdEf0123456789+/=" parent-hash "ZZZ=" via-port "1-4" "#,
        r#"with-interface { 03:01:01 03:00:00 } with-connect-type "hotplug""#
    );

    #[test]
    fn builds_a_device_from_a_rule() {
        let device = Device::from_rule(7, KEYBOARD).unwrap();
        assert_eq!(device.id, 7);
        assert_eq!(device.target, Target::Block);
        assert_eq!(device.vendor_id, "046d");
        assert_eq!(device.product_id, "c31c");
        assert_eq!(device.usb_id(), "046d:c31c");
        assert_eq!(device.name, "USB Keyboard");
        assert_eq!(device.via_port, "1-4");
        assert_eq!(device.connect_type, "hotplug");
        assert_eq!(device.interfaces.len(), 2);
        assert!(device.is_input_capable());
        assert!(!device.is_storage());
        assert!(!device.is_root_hub());
        assert!(!device.is_hardwired());
    }

    #[test]
    fn permanent_rule_pins_to_the_hash_not_the_usb_id() {
        let device = Device::from_rule(7, KEYBOARD).unwrap();
        let rule = device.permanent_rule(Target::Allow).unwrap();
        assert_eq!(rule, r#"allow hash "AbCdEf0123456789+/=""#);
        // A device that spoofs the USB ID must not be matched by it.
        assert!(!rule.contains("046d"));
        // And the generated rule must itself be parseable by the daemon.
        let parsed = Rule::parse(&rule).unwrap();
        assert_eq!(parsed.target, Target::Allow);
        assert_eq!(parsed.attribute("hash"), Some("AbCdEf0123456789+/="));
    }

    #[test]
    fn no_permanent_rule_without_a_hash() {
        let device = Device::from_rule(1, r#"allow id 1234:5678 name "No Hash""#).unwrap();
        assert_eq!(device.permanent_rule(Target::Allow), None);
    }

    #[test]
    fn display_name_falls_back_when_the_device_reports_none() {
        let unnamed = Device::from_rule(3, r#"allow id 1234:5678 name """#).unwrap();
        assert_eq!(unnamed.display_name(), "1234:5678");

        let anonymous = Device::from_rule(4, r#"allow via-port "2-1""#).unwrap();
        assert_eq!(anonymous.display_name(), "port 2-1");

        let nothing = Device::from_rule(5, "allow").unwrap();
        assert_eq!(nothing.display_name(), "device 5");
    }

    #[test]
    fn detects_root_hubs() {
        let hub = Device::from_rule(
            1,
            r#"allow id 1d6b:0002 with-interface 09:00:00 with-connect-type "hardwired""#,
        )
        .unwrap();
        assert!(hub.is_root_hub());
        assert!(hub.is_hardwired());
    }

    #[test]
    fn classifies_interfaces() {
        let storage = Interface::parse("08:06:50").unwrap();
        assert_eq!(storage.class_name(), "Mass storage");
        assert!(storage.is_storage());

        let hid = Interface::parse("03:01:01").unwrap();
        assert!(hid.is_input_capable());

        let net = Interface::parse("02:06:00").unwrap();
        assert!(net.is_network());

        assert_eq!(Interface::parse("nope"), None);
        assert_eq!(Interface::parse("08:06:50:00"), None);
    }

    #[test]
    fn interface_display_round_trips() {
        let interface = Interface::parse("08:06:50").unwrap();
        assert_eq!(interface.to_string(), "08:06:50");
    }

    #[test]
    fn presence_events_map_from_dbus() {
        assert_eq!(PresenceEvent::from_dbus(0), PresenceEvent::Present);
        assert_eq!(PresenceEvent::from_dbus(1), PresenceEvent::Insert);
        assert_eq!(PresenceEvent::from_dbus(3), PresenceEvent::Remove);
        assert_eq!(PresenceEvent::from_dbus(42), PresenceEvent::Unknown);
    }
}
