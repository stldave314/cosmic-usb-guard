// SPDX-License-Identifier: GPL-3.0-or-later

//! The append-only decision journal.
//!
//! USBGuard has its own audit log, but it is root-readable and records the
//! daemon's view rather than the user's. This journal answers the question the
//! app exists to answer: *what was connected, what was decided about it, and
//! who decided?* It is written as JSON Lines so it can be appended to safely,
//! read back incrementally, and inspected with ordinary tools.
//!
//! It lives under the user's data directory and is never a security boundary:
//! it records what happened, it does not enforce anything.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::constants::{JOURNAL_FILE, JOURNAL_MAX_BYTES, JOURNAL_VIEW_LIMIT, PKG_NAME};
use crate::debug_log;
use crate::usbguard::{Device, Target};

/// What kind of thing happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// A device was plugged in.
    Inserted,
    /// A device was unplugged.
    Removed,
    /// A device's descriptors changed while connected.
    Updated,
    /// A device was authorised.
    Allowed,
    /// A device was de-authorised.
    Blocked,
    /// A device was de-authorised and detached.
    Rejected,
    /// A standing rule for a device was removed.
    Revoked,
    /// USBGuard became reachable.
    ServiceUp,
    /// USBGuard became unreachable.
    ServiceDown,
    /// A health check changed to a failing state.
    HealthProblem,
}

impl Kind {
    /// The kind corresponding to a decision target.
    pub fn for_target(target: Target) -> Self {
        match target {
            Target::Allow => Self::Allowed,
            Target::Block => Self::Blocked,
            Target::Reject => Self::Rejected,
            Target::Match | Target::Unknown => Self::Updated,
        }
    }

    /// Whether this kind records an authorisation decision, as opposed to a
    /// device merely coming and going.
    pub fn is_decision(self) -> bool {
        matches!(
            self,
            Self::Allowed | Self::Blocked | Self::Rejected | Self::Revoked
        )
    }
}

/// Who or what made a decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Actor {
    /// The user, through this application.
    User,
    /// USBGuard's own policy, with no human involved.
    Policy,
    /// Something outside this app: another front-end, the CLI, or a script.
    External,
    /// The application itself, for service and health entries.
    System,
}

/// A device as it was at the moment of the entry.
///
/// Denormalised on purpose: the journal has to stay readable years later, when
/// the device is long gone and its daemon-assigned ID has been reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSnapshot {
    /// Best available display name.
    pub name: String,
    /// `vvvv:pppp`.
    #[serde(default)]
    pub usb_id: String,
    /// Serial number, if the device reported one.
    #[serde(default)]
    pub serial: String,
    /// Descriptor hash — the durable identity.
    #[serde(default)]
    pub hash: String,
    /// Physical port path.
    #[serde(default)]
    pub via_port: String,
    /// Interface class names.
    #[serde(default)]
    pub classes: Vec<String>,
}

impl From<&Device> for DeviceSnapshot {
    fn from(device: &Device) -> Self {
        Self {
            name: device.display_name(),
            usb_id: device.usb_id(),
            serial: device.serial.clone(),
            hash: device.hash.clone(),
            via_port: device.via_port.clone(),
            classes: device
                .interface_classes()
                .into_iter()
                .map(str::to_string)
                .collect(),
        }
    }
}

/// One line of the journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// When it happened, in UTC.
    pub timestamp: DateTime<Utc>,
    /// What happened.
    pub kind: Kind,
    /// Who caused it.
    pub actor: Actor,
    /// The device involved, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSnapshot>,
    /// Whether the decision was written into the persistent policy.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub permanent: bool,
    /// Free-text detail: an error message, a health check name, a parameter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub detail: String,
}

impl Entry {
    /// A device-related entry.
    pub fn device(kind: Kind, actor: Actor, device: &Device) -> Self {
        Self {
            timestamp: Utc::now(),
            kind,
            actor,
            device: Some(DeviceSnapshot::from(device)),
            permanent: false,
            detail: String::new(),
        }
    }

    /// An entry with no device, e.g. the service going down.
    pub fn system(kind: Kind, detail: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            kind,
            actor: Actor::System,
            device: None,
            permanent: false,
            detail: detail.into(),
        }
    }

    /// Mark the entry as having changed the persistent policy.
    pub fn permanent(mut self, permanent: bool) -> Self {
        self.permanent = permanent;
        self
    }

    /// Attach free-text detail.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    /// The timestamp in the user's local time zone, for display.
    pub fn local_time(&self) -> DateTime<Local> {
        self.timestamp.with_timezone(&Local)
    }
}

/// Directory the journal lives in: `$XDG_DATA_HOME/cosmic-usb-guard`.
pub fn data_dir() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir);
    base.join(PKG_NAME)
}

/// Full path to the journal file.
pub fn path() -> PathBuf {
    data_dir().join(JOURNAL_FILE)
}

/// Path the journal is rotated to when it grows too large.
fn rotated_path() -> PathBuf {
    data_dir().join(format!("{JOURNAL_FILE}.1"))
}

/// Append an entry.
///
/// Failures are reported but never propagated to the caller: losing a log line
/// must not stop the user from blocking a device.
pub fn append(entry: &Entry) {
    if let Err(e) = try_append(&path(), entry) {
        crate::error_log!(crate::debug::JOURNAL, "could not write journal entry: {e}");
    }
}

fn try_append(path: &Path, entry: &Entry) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    rotate_if_needed(path)?;

    let mut line = serde_json::to_string(entry).map_err(std::io::Error::other)?;
    line.push('\n');

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(line.as_bytes())?;

    debug_log!(
        crate::debug::JOURNAL,
        "appended {:?} by {:?}",
        entry.kind,
        entry.actor
    );
    Ok(())
}

/// Move the journal aside once it exceeds [`JOURNAL_MAX_BYTES`].
///
/// One generation is kept. Rotation is checked on append rather than on a
/// timer so an idle session never touches the disk.
fn rotate_if_needed(path: &Path) -> std::io::Result<()> {
    let Ok(metadata) = std::fs::metadata(path) else {
        return Ok(());
    };
    if metadata.len() < JOURNAL_MAX_BYTES {
        return Ok(());
    }
    debug_log!(
        crate::debug::JOURNAL,
        "rotating journal at {} bytes",
        metadata.len()
    );
    std::fs::rename(path, rotated_path())
}

/// Read back the most recent entries, oldest first.
///
/// At most [`JOURNAL_VIEW_LIMIT`] are returned. Unparseable lines are skipped:
/// a truncated final line from an interrupted write should not hide the rest
/// of the history.
pub fn read_recent() -> Vec<Entry> {
    read_recent_from(&path(), JOURNAL_VIEW_LIMIT)
}

fn read_recent_from(path: &Path, limit: usize) -> Vec<Entry> {
    let Ok(file) = std::fs::File::open(path) else {
        return Vec::new();
    };

    let mut entries = std::collections::VecDeque::with_capacity(limit.min(1024));
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Entry>(&line) {
            Ok(entry) => {
                if entries.len() == limit {
                    entries.pop_front();
                }
                entries.push_back(entry);
            }
            Err(e) => {
                debug_log!(
                    crate::debug::JOURNAL,
                    "skipping malformed journal line: {e}"
                );
            }
        }
    }

    entries.into()
}

/// Delete the journal and its rotated generation.
pub fn clear() -> std::io::Result<()> {
    for path in [path(), rotated_path()] {
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    debug_log!(crate::debug::JOURNAL, "journal cleared");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_device() -> Device {
        Device::from_rule(
            4,
            concat!(
                r#"block id 0781:5567 serial "AA123" name "Cruzer Blade" "#,
                r#"hash "H4sh=" parent-hash "P4r3nt=" via-port "1-2" "#,
                r#"with-interface 08:06:50 with-connect-type "hotplug""#
            ),
        )
        .unwrap()
    }

    fn temp_path(tag: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "cosmic-usb-guard-test-{tag}-{}-{}.jsonl",
            std::process::id(),
            // Distinguish concurrent tests within a process.
            tag.len()
        ));
        let _ = std::fs::remove_file(&path);
        path
    }

    #[test]
    fn entries_round_trip_through_json() {
        let entry = Entry::device(Kind::Allowed, Actor::User, &sample_device()).permanent(true);
        let json = serde_json::to_string(&entry).unwrap();
        let parsed: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, entry);
        assert!(parsed.permanent);
        assert_eq!(parsed.device.unwrap().hash, "H4sh=");
    }

    #[test]
    fn snapshot_captures_identity_not_just_the_name() {
        let snapshot = DeviceSnapshot::from(&sample_device());
        assert_eq!(snapshot.name, "Cruzer Blade");
        assert_eq!(snapshot.usb_id, "0781:5567");
        assert_eq!(snapshot.serial, "AA123");
        assert_eq!(snapshot.hash, "H4sh=");
        assert_eq!(snapshot.classes, vec!["Mass storage"]);
    }

    #[test]
    fn append_then_read_preserves_order() {
        let path = temp_path("order");
        let device = sample_device();

        try_append(
            &path,
            &Entry::device(Kind::Inserted, Actor::Policy, &device),
        )
        .unwrap();
        try_append(&path, &Entry::device(Kind::Allowed, Actor::User, &device)).unwrap();
        try_append(&path, &Entry::device(Kind::Removed, Actor::Policy, &device)).unwrap();

        let entries = read_recent_from(&path, 100);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].kind, Kind::Inserted);
        assert_eq!(entries[1].kind, Kind::Allowed);
        assert_eq!(entries[2].kind, Kind::Removed);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn reading_keeps_the_newest_entries_when_over_the_limit() {
        let path = temp_path("limit");
        let device = sample_device();
        for _ in 0..5 {
            try_append(
                &path,
                &Entry::device(Kind::Inserted, Actor::Policy, &device),
            )
            .unwrap();
        }
        try_append(&path, &Entry::device(Kind::Allowed, Actor::User, &device)).unwrap();

        let entries = read_recent_from(&path, 2);
        assert_eq!(entries.len(), 2);
        // The most recent entry must survive truncation.
        assert_eq!(entries[1].kind, Kind::Allowed);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_truncated_final_line_does_not_hide_earlier_history() {
        let path = temp_path("truncated");
        try_append(
            &path,
            &Entry::device(Kind::Allowed, Actor::User, &sample_device()),
        )
        .unwrap();

        // Simulate a write interrupted mid-line.
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        file.write_all(br#"{"timestamp":"2026-01-01T00:00:00Z","kin"#)
            .unwrap();
        drop(file);

        let entries = read_recent_from(&path, 100);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].kind, Kind::Allowed);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn reading_a_missing_journal_is_empty_not_an_error() {
        let path = temp_path("missing");
        assert!(read_recent_from(&path, 10).is_empty());
    }

    #[test]
    fn decision_kinds_are_distinguished_from_presence_kinds() {
        assert!(Kind::Allowed.is_decision());
        assert!(Kind::Blocked.is_decision());
        assert!(Kind::Rejected.is_decision());
        assert!(Kind::Revoked.is_decision());
        assert!(!Kind::Inserted.is_decision());
        assert!(!Kind::Removed.is_decision());
        assert!(!Kind::ServiceDown.is_decision());
    }

    #[test]
    fn targets_map_to_kinds() {
        assert_eq!(Kind::for_target(Target::Allow), Kind::Allowed);
        assert_eq!(Kind::for_target(Target::Block), Kind::Blocked);
        assert_eq!(Kind::for_target(Target::Reject), Kind::Rejected);
    }

    #[test]
    fn data_dir_is_under_the_users_data_home() {
        // Must not fall back to a world-writable location when HOME is set.
        let dir = data_dir();
        assert!(dir.ends_with(PKG_NAME), "unexpected data dir: {dir:?}");
        assert!(dir.is_absolute(), "data dir must be absolute: {dir:?}");
    }
}
