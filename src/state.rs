// SPDX-License-Identifier: GPL-3.0-or-later

//! State shared by the applet and the window application.
//!
//! Both binaries observe the same daemon and answer the same questions, so the
//! model and its transitions live here and each binary supplies only its own
//! iced wiring. Keeping the logic out of the `Application` impls is also what
//! makes it testable without a compositor.

use std::collections::{HashMap, HashSet};

use crate::config::ConfigState;
use crate::debug_log;
use crate::journal::{self, Actor, Entry, Kind};
use crate::ui::{HistoryFilter, SettingChange};
use crate::usbguard::{Device, Event, Health, PresenceEvent, Target};

/// A device waiting for the user to decide about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pending {
    /// Daemon-assigned device ID.
    pub device_id: u32,
    /// Whether the device arrived while we were watching, as opposed to
    /// already being connected when we started.
    ///
    /// Only a live arrival earns a notification or an auto-opened popup;
    /// re-announcing everything that was already blocked at login would train
    /// the user to dismiss prompts without reading them.
    pub live: bool,
    /// ID of the notification raised for it, if any.
    pub notification_id: Option<u32>,
}

/// Something the surrounding binary must do that the state cannot do itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// Post a decision notification for this device.
    Notify(u32),
    /// Withdraw the notification with this ID.
    CloseNotification(u32),
    /// Open the applet popup, because a decision is waiting.
    OpenPopup,
    /// Re-read the journal from disk.
    ReloadHistory,
}

/// Everything the UI renders from.
#[derive(Debug)]
pub struct State {
    /// User settings.
    pub config: ConfigState,
    /// Whether the daemon is currently reachable.
    pub connected: bool,
    /// Why not, when `connected` is false.
    pub disconnect_reason: String,
    /// Devices, as last reported.
    pub devices: Vec<Device>,
    /// Installation health.
    pub health: Health,
    /// Whether health has been evaluated at least once.
    ///
    /// Distinguishes "not checked yet" from "checked and broken"; without it
    /// the UI would flash a scary red banner every time it starts.
    pub health_checked: bool,
    /// Devices awaiting a decision.
    pub pending: Vec<Pending>,
    /// Device whose details are expanded.
    pub selected: Option<u32>,
    /// Whether the next decision is written as a standing rule.
    pub permanent: bool,
    /// Devices with an in-flight request, so their buttons can be disabled.
    pub busy: HashSet<u32>,
    /// The last error worth showing the user.
    pub error: Option<String>,
    /// Journal entries for the history view.
    pub history: Vec<Entry>,
    /// Which entries the history view lists.
    pub history_filter: HistoryFilter,
    /// Descriptor hashes that already have a standing policy rule.
    pub known_hashes: HashSet<String>,
    /// Whether `known_hashes` reflects a successful policy read.
    known_hashes_loaded: bool,
}

impl Default for State {
    fn default() -> Self {
        let config = ConfigState::load();
        Self {
            permanent: config.settings.default_permanent,
            config,
            connected: false,
            disconnect_reason: String::new(),
            devices: Vec::new(),
            health: Health::default(),
            health_checked: false,
            pending: Vec::new(),
            selected: None,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            known_hashes: HashSet::new(),
            known_hashes_loaded: false,
        }
    }
}

impl State {
    /// Fresh state with settings loaded from disk.
    pub fn new() -> Self {
        Self::default()
    }

    // -- queries ----------------------------------------------------------

    /// Look up a device by daemon ID.
    pub fn device(&self, id: u32) -> Option<&Device> {
        self.devices.iter().find(|d| d.id == id)
    }

    /// Whether a device should be shown given the current filters.
    ///
    /// A device awaiting a decision is always shown, whatever the filters say:
    /// hiding something the user must act on would be a bug, not a preference.
    pub fn is_visible(&self, device: &Device) -> bool {
        if self.is_pending(device.id) {
            return true;
        }
        if device.is_root_hub() && !self.config.settings.show_root_hubs {
            return false;
        }
        if device.is_hardwired() && !self.config.settings.show_hardwired {
            return false;
        }
        true
    }

    /// Devices to render, ordered by how much they want attention.
    pub fn visible_devices(&self) -> Vec<&Device> {
        let mut devices: Vec<&Device> = self
            .devices
            .iter()
            .filter(|device| self.is_visible(device))
            .collect();

        devices.sort_by(|a, b| {
            sort_rank(self, a)
                .cmp(&sort_rank(self, b))
                .then_with(|| {
                    a.display_name()
                        .to_lowercase()
                        .cmp(&b.display_name().to_lowercase())
                })
                .then_with(|| a.id.cmp(&b.id))
        });
        devices
    }

    /// How many devices the current filters are hiding.
    pub fn hidden_count(&self) -> usize {
        self.devices
            .iter()
            .filter(|device| !self.is_visible(device))
            .count()
    }

    /// Whether this device is awaiting a decision.
    pub fn is_pending(&self, device_id: u32) -> bool {
        self.pending.iter().any(|p| p.device_id == device_id)
    }

    /// Devices awaiting a decision, in arrival order.
    pub fn pending_devices(&self) -> Vec<&Device> {
        self.pending
            .iter()
            .filter_map(|p| self.device(p.device_id))
            .collect()
    }

    /// Whether a device has a standing policy rule pinned to its hash.
    ///
    /// Returns `false` when the policy could not be read, which makes the
    /// prompt logic fail towards asking rather than towards silence.
    pub fn has_standing_rule(&self, device: &Device) -> bool {
        self.known_hashes_loaded
            && !device.hash.is_empty()
            && self.known_hashes.contains(&device.hash)
    }

    /// Journal entries matching the current filter, newest first.
    pub fn filtered_history(&self) -> Vec<&Entry> {
        self.history
            .iter()
            .rev()
            .filter(|entry| match self.history_filter {
                HistoryFilter::All => true,
                HistoryFilter::Decisions => entry.kind.is_decision(),
            })
            .collect()
    }

    /// Whether the state is safe to present as "protected".
    ///
    /// Requires a live connection *and* a health evaluation that passed.
    /// Anything less is reported honestly rather than optimistically.
    pub fn is_protected(&self) -> bool {
        self.connected && self.health_checked && self.health.is_healthy()
    }

    // -- transitions ------------------------------------------------------

    /// Fold a daemon event into the state, returning what the binary must do.
    pub fn apply_event(&mut self, event: Event) -> Vec<Effect> {
        match event {
            Event::Connected { devices } => self.on_connected(devices),
            Event::Disconnected { reason } => self.on_disconnected(reason),
            Event::Presence { device, event } => self.on_presence(device, event),
            Event::Policy {
                device, old, new, ..
            } => self.on_policy(device, old, new),
        }
    }

    fn on_connected(&mut self, devices: Vec<Device>) -> Vec<Effect> {
        let was_connected = self.connected;
        self.connected = true;
        self.disconnect_reason.clear();
        self.devices = devices;

        if !was_connected {
            self.record(Entry::system(Kind::ServiceUp, String::new()));
        }

        // Everything blocked and pluggable wants a decision, but nothing here
        // arrived while we were watching, so none of it is `live`.
        let existing: Vec<u32> = self
            .devices
            .iter()
            .filter(|device| self.needs_decision(device))
            .map(|device| device.id)
            .collect();

        let mut effects = Vec::new();

        // Drop prompts for devices that are no longer present or no longer
        // blocked; a stale prompt would apply a decision to whatever device
        // has since inherited that ID.
        effects.extend(self.retain_pending(|state, pending| {
            state
                .device(pending.device_id)
                .is_some_and(|device| state.needs_decision(device))
        }));

        for id in existing {
            if !self.is_pending(id) {
                self.pending.push(Pending {
                    device_id: id,
                    live: false,
                    notification_id: None,
                });
            }
        }

        debug_log!(
            crate::debug::UI,
            "connected: {} device(s), {} pending",
            self.devices.len(),
            self.pending.len()
        );
        effects
    }

    fn on_disconnected(&mut self, reason: String) -> Vec<Effect> {
        if self.connected {
            self.record(Entry::system(Kind::ServiceDown, reason.clone()));
        }
        self.connected = false;
        self.disconnect_reason = reason;

        // The device list is no longer current. Keeping it on screen would
        // present stale authorisations as live ones.
        self.devices.clear();
        self.known_hashes_loaded = false;

        // Withdraw prompts we can no longer act on.
        let effects = self.retain_pending(|_, _| false);
        debug_log!(crate::debug::UI, "disconnected: {}", self.disconnect_reason);
        effects
    }

    fn on_presence(&mut self, device: Device, event: PresenceEvent) -> Vec<Effect> {
        let mut effects = Vec::new();

        match event {
            PresenceEvent::Remove => {
                self.devices.retain(|d| d.id != device.id);
                if self.selected == Some(device.id) {
                    self.selected = None;
                }
                self.busy.remove(&device.id);
                effects.extend(self.retain_pending(|_, p| p.device_id != device.id));
                self.record(Entry::device(Kind::Removed, Actor::Policy, &device));
                return effects;
            }
            PresenceEvent::Insert | PresenceEvent::Present | PresenceEvent::Update => {
                self.upsert(device.clone());
            }
            PresenceEvent::Unknown => {
                self.upsert(device.clone());
                return effects;
            }
        }

        let kind = match event {
            PresenceEvent::Update => Kind::Updated,
            _ => Kind::Inserted,
        };
        self.record(Entry::device(kind, Actor::Policy, &device));

        if event == PresenceEvent::Insert && self.needs_decision(&device) {
            if !self.is_pending(device.id) {
                self.pending.push(Pending {
                    device_id: device.id,
                    live: true,
                    notification_id: None,
                });
            }

            if self.config.settings.prompt_on_insert {
                if self.config.settings.notify_on_insert {
                    effects.push(Effect::Notify(device.id));
                }
                if self.config.settings.auto_open_popup {
                    effects.push(Effect::OpenPopup);
                }
            }

            debug_log!(
                crate::debug::DEVICE,
                "device {} inserted and needs a decision",
                device.id
            );
        }

        effects
    }

    fn on_policy(&mut self, device: Device, old: Target, new: Target) -> Vec<Effect> {
        self.upsert(device.clone());
        self.busy.remove(&device.id);

        if old == new {
            return Vec::new();
        }

        // A decision made in this app is journalled where it is issued, with
        // the actor set correctly. Anything reaching us only as a signal came
        // from somewhere else — another front-end, the CLI, or the daemon's
        // own policy.
        self.record(Entry::device(
            Kind::for_target(new),
            Actor::External,
            &device,
        ));

        let mut effects = Vec::new();
        if new == Target::Allow {
            effects.extend(self.retain_pending(|_, p| p.device_id != device.id));
        }
        effects
    }

    /// Record a decision this app made.
    pub fn record_decision(&mut self, device: &Device, target: Target, permanent: bool) {
        self.record(
            Entry::device(Kind::for_target(target), Actor::User, device).permanent(permanent),
        );
    }

    /// Record a revocation this app made.
    pub fn record_revocation(&mut self, device: &Device, rules_removed: usize) {
        self.record(
            Entry::device(Kind::Revoked, Actor::User, device)
                .with_detail(format!("{rules_removed} rule(s) removed")),
        );
    }

    /// Clear a device's prompt, returning any notification to withdraw.
    pub fn resolve_pending(&mut self, device_id: u32) -> Vec<Effect> {
        self.retain_pending(|_, p| p.device_id != device_id)
    }

    /// Attach a posted notification's ID to its prompt so it can be withdrawn
    /// once the decision is made.
    pub fn set_notification(&mut self, device_id: u32, notification_id: u32) {
        if let Some(pending) = self.pending.iter_mut().find(|p| p.device_id == device_id) {
            pending.notification_id = Some(notification_id);
        }
    }

    /// The device a notification belongs to.
    pub fn device_for_notification(&self, notification_id: u32) -> Option<u32> {
        self.pending
            .iter()
            .find(|p| p.notification_id == Some(notification_id))
            .map(|p| p.device_id)
    }

    /// Replace the health report.
    pub fn set_health(&mut self, health: Health) {
        let was_healthy = self.health_checked && self.health.is_healthy();
        let now_healthy = health.is_healthy();

        // Journal the transition, not every poll, or the history fills with
        // duplicates of a problem the user has not fixed yet.
        if self.health_checked && was_healthy && !now_healthy {
            let detail = health
                .problems()
                .first()
                .map(|c| format!("{:?}: {}", c.id, c.detail))
                .unwrap_or_default();
            self.record(Entry::system(Kind::HealthProblem, detail));
        }

        self.health = health;
        self.health_checked = true;
    }

    /// Replace the set of hashes that have standing policy rules.
    pub fn set_known_hashes(&mut self, hashes: HashSet<String>) {
        self.known_hashes = hashes;
        self.known_hashes_loaded = true;
    }

    /// Apply and persist a settings change.
    pub fn apply_setting(&mut self, change: SettingChange) {
        self.config.update(|settings| change.apply(settings));
        if let SettingChange::DefaultPermanent(value) = change {
            self.permanent = value;
        }
    }

    /// Reload the journal from disk.
    pub fn reload_history(&mut self) {
        self.history = journal::read_recent();
    }

    /// Delete the journal and the in-memory copy of it.
    pub fn clear_history(&mut self) {
        if let Err(e) = journal::clear() {
            self.error = Some(e.to_string());
            return;
        }
        self.history.clear();
    }

    // -- internals --------------------------------------------------------

    /// Whether a device is one the user should be asked about.
    ///
    /// Root hubs and soldered-in devices are excluded: they cannot be
    /// unplugged, were not introduced by anyone, and prompting about them
    /// would bury the prompts that matter.
    fn needs_decision(&self, device: &Device) -> bool {
        device.target != Target::Allow
            && !device.is_root_hub()
            && !device.is_hardwired()
            && !self.has_standing_rule(device)
    }

    fn upsert(&mut self, device: Device) {
        match self.devices.iter_mut().find(|d| d.id == device.id) {
            Some(existing) => *existing = device,
            None => self.devices.push(device),
        }
    }

    /// Keep only the prompts for which `keep` holds, emitting a
    /// [`Effect::CloseNotification`] for each one dropped.
    fn retain_pending(&mut self, keep: impl Fn(&Self, &Pending) -> bool) -> Vec<Effect> {
        let (kept, dropped): (Vec<Pending>, Vec<Pending>) = std::mem::take(&mut self.pending)
            .into_iter()
            .partition(|pending| keep(self, pending));

        self.pending = kept;
        dropped
            .into_iter()
            .filter_map(|p| p.notification_id.map(Effect::CloseNotification))
            .collect()
    }

    fn record(&mut self, entry: Entry) {
        if !self.config.settings.journal_enabled {
            return;
        }
        journal::append(&entry);
        self.history.push(entry);
    }
}

/// Sort key: prompts first, then blocked devices, then everything else.
fn sort_rank(state: &State, device: &Device) -> u8 {
    if state.is_pending(device.id) {
        0
    } else if device.target != Target::Allow {
        1
    } else {
        2
    }
}

/// Collect the hashes of every rule in a policy listing.
pub fn hashes_from_rules(rules: &[crate::usbguard::PolicyRule]) -> HashSet<String> {
    rules
        .iter()
        .filter_map(|rule| rule.hash())
        .map(str::to_string)
        .collect()
}

/// Devices grouped by the standing-rule status of their hash, for tests and
/// for the policy view.
pub fn devices_by_hash(devices: &[Device]) -> HashMap<&str, &Device> {
    devices
        .iter()
        .filter(|d| !d.hash.is_empty())
        .map(|d| (d.hash.as_str(), d))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(id: u32, target: &str, extra: &str) -> Device {
        Device::from_rule(
            id,
            &format!(
                r#"{target} id 0781:556{id} name "Device {id}" hash "H{id}=" via-port "1-{id}" with-connect-type "hotplug" {extra}"#
            ),
        )
        .unwrap()
    }

    /// A state that does not touch the user's real journal or config.
    fn state() -> State {
        let mut state = State {
            config: ConfigState::default(),
            connected: false,
            disconnect_reason: String::new(),
            devices: Vec::new(),
            health: Health::default(),
            health_checked: false,
            pending: Vec::new(),
            selected: None,
            permanent: false,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            known_hashes: HashSet::new(),
            known_hashes_loaded: false,
        };
        // Keep tests off the real journal file.
        state.config.settings.journal_enabled = false;
        state
    }

    #[test]
    fn a_blocked_insert_raises_a_prompt() {
        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });

        let effects = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });

        assert!(state.is_pending(1));
        assert!(effects.contains(&Effect::Notify(1)));
        assert!(effects.contains(&Effect::OpenPopup));
    }

    #[test]
    fn an_allowed_insert_raises_no_prompt() {
        let mut state = state();
        let effects = state.apply_event(Event::Presence {
            device: device(2, "allow", ""),
            event: PresenceEvent::Insert,
        });
        assert!(!state.is_pending(2));
        assert!(effects.is_empty());
    }

    #[test]
    fn devices_present_at_startup_are_pending_but_not_announced() {
        // Re-notifying for everything blocked at login teaches the user to
        // dismiss prompts reflexively.
        let mut state = state();
        let effects = state.apply_event(Event::Connected {
            devices: vec![device(1, "block", ""), device(2, "allow", "")],
        });

        assert!(state.is_pending(1));
        assert!(!state.is_pending(2));
        assert!(!effects.iter().any(|e| matches!(e, Effect::Notify(_))));
        assert!(!effects.contains(&Effect::OpenPopup));
        assert!(!state.pending[0].live);
    }

    #[test]
    fn hardwired_and_root_hub_devices_are_never_prompted_about() {
        let mut state = state();
        let root_hub = Device::from_rule(
            1,
            r#"block id 1d6b:0002 hash "R=" with-connect-type "hardwired""#,
        )
        .unwrap();
        state.apply_event(Event::Connected {
            devices: vec![root_hub],
        });
        assert!(state.pending.is_empty());
    }

    #[test]
    fn a_device_with_a_standing_rule_is_not_prompted_about() {
        let mut state = state();
        state.set_known_hashes(HashSet::from(["H1=".to_string()]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        assert!(state.pending.is_empty());
    }

    #[test]
    fn an_unreadable_policy_fails_towards_asking() {
        // known_hashes was never loaded, so nothing counts as known and the
        // user still gets asked.
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        assert!(state.is_pending(1));
    }

    #[test]
    fn allowing_a_device_clears_its_prompt_and_notification() {
        let mut state = state();
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        state.set_notification(1, 42);

        let effects = state.apply_event(Event::Policy {
            device: device(1, "allow", ""),
            old: Target::Block,
            new: Target::Allow,
            rule_id: 0,
        });

        assert!(!state.is_pending(1));
        assert!(effects.contains(&Effect::CloseNotification(42)));
    }

    #[test]
    fn unplugging_clears_the_prompt_and_the_device() {
        let mut state = state();
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        state.set_notification(1, 7);

        let effects = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Remove,
        });

        assert!(state.devices.is_empty());
        assert!(!state.is_pending(1));
        // The prompt must not outlive the device: the daemon reuses IDs, so
        // acting on it later could hit an entirely different device.
        assert!(effects.contains(&Effect::CloseNotification(7)));
    }

    #[test]
    fn disconnecting_drops_the_device_list_rather_than_showing_it_stale() {
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", ""), device(2, "block", "")],
        });
        assert_eq!(state.devices.len(), 2);

        let effects = state.apply_event(Event::Disconnected {
            reason: "stopped".into(),
        });

        assert!(state.devices.is_empty());
        assert!(state.pending.is_empty());
        assert!(!state.connected);
        assert_eq!(state.disconnect_reason, "stopped");
        assert!(!effects.iter().any(|e| matches!(e, Effect::Notify(_))));
    }

    #[test]
    fn is_protected_requires_a_connection_and_a_passed_check() {
        let mut state = state();
        assert!(
            !state.is_protected(),
            "unchecked state must not read as protected"
        );

        state.apply_event(Event::Connected { devices: vec![] });
        assert!(!state.is_protected(), "still unchecked");

        state.set_health(Health {
            checks: vec![crate::usbguard::health::Check {
                id: crate::usbguard::health::CheckId::DaemonRunning,
                severity: crate::usbguard::Severity::Ok,
                detail: String::new(),
                remedy: None,
            }],
        });
        assert!(state.is_protected());

        state.apply_event(Event::Disconnected {
            reason: "gone".into(),
        });
        assert!(!state.is_protected(), "a lost connection must clear it");
    }

    #[test]
    fn pending_devices_are_visible_even_when_filters_would_hide_them() {
        let mut state = state();
        state.config.settings.show_hardwired = false;

        // A hardwired device would normally be hidden, but if something put it
        // in the pending list the user must still be able to act on it.
        let dev = Device::from_rule(
            9,
            r#"block id 1234:5678 hash "H9=" with-connect-type "hardwired""#,
        )
        .unwrap();
        state.devices.push(dev.clone());
        state.pending.push(Pending {
            device_id: 9,
            live: true,
            notification_id: None,
        });

        assert!(state.is_visible(&dev));
        assert_eq!(state.visible_devices().len(), 1);
    }

    #[test]
    fn visible_devices_put_prompts_first_then_blocked_then_allowed() {
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(3, "allow", ""), device(1, "block", "")],
        });
        state.apply_event(Event::Presence {
            device: device(2, "block", ""),
            event: PresenceEvent::Insert,
        });

        let order: Vec<u32> = state.visible_devices().iter().map(|d| d.id).collect();
        // 1 and 2 are both pending (1 from startup, 2 live); 3 is allowed.
        assert_eq!(order.last(), Some(&3));
        assert!(order[..2].contains(&1) && order[..2].contains(&2));
    }

    #[test]
    fn hidden_count_reflects_the_filters() {
        let mut state = state();
        state.config.settings.show_root_hubs = false;
        let hub = Device::from_rule(1, r#"allow id 1d6b:0002 hash "R=""#).unwrap();
        state.apply_event(Event::Connected {
            devices: vec![hub, device(2, "allow", "")],
        });

        assert_eq!(state.hidden_count(), 1);
        assert_eq!(state.visible_devices().len(), 1);
    }

    #[test]
    fn history_filter_selects_decisions_only() {
        let mut state = state();
        let dev = device(1, "block", "");
        state.history = vec![
            Entry::device(Kind::Inserted, Actor::Policy, &dev),
            Entry::device(Kind::Allowed, Actor::User, &dev),
            Entry::system(Kind::ServiceUp, ""),
        ];

        assert_eq!(state.filtered_history().len(), 3);
        state.history_filter = HistoryFilter::Decisions;
        let filtered = state.filtered_history();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].kind, Kind::Allowed);
    }

    #[test]
    fn history_is_newest_first() {
        let mut state = state();
        let dev = device(1, "block", "");
        state.history = vec![
            Entry::device(Kind::Inserted, Actor::Policy, &dev),
            Entry::device(Kind::Blocked, Actor::User, &dev),
        ];
        assert_eq!(state.filtered_history()[0].kind, Kind::Blocked);
    }

    #[test]
    fn health_problems_are_journalled_once_not_every_poll() {
        let mut state = state();
        state.config.settings.journal_enabled = true;

        let ok = Health {
            checks: vec![crate::usbguard::health::Check {
                id: crate::usbguard::health::CheckId::DaemonRunning,
                severity: crate::usbguard::Severity::Ok,
                detail: String::new(),
                remedy: None,
            }],
        };
        let bad = Health {
            checks: vec![crate::usbguard::health::Check {
                id: crate::usbguard::health::CheckId::DaemonRunning,
                severity: crate::usbguard::Severity::Critical,
                detail: "failed".into(),
                remedy: None,
            }],
        };

        state.set_health(ok);
        let before = state.history.len();
        state.set_health(bad.clone());
        let after_first = state.history.len();
        state.set_health(bad);
        let after_second = state.history.len();

        assert_eq!(after_first, before + 1, "the transition should be recorded");
        assert_eq!(after_second, after_first, "repeats should not be");
    }

    #[test]
    fn settings_changes_are_reflected_immediately() {
        let mut state = state();
        assert!(!state.permanent);
        state.apply_setting(SettingChange::DefaultPermanent(true));
        assert!(state.permanent);
        assert!(state.config.settings.default_permanent);
    }

    #[test]
    fn notification_ids_map_back_to_their_device() {
        let mut state = state();
        state.apply_event(Event::Presence {
            device: device(5, "block", ""),
            event: PresenceEvent::Insert,
        });
        state.set_notification(5, 99);
        assert_eq!(state.device_for_notification(99), Some(5));
        assert_eq!(state.device_for_notification(1), None);
    }

    #[test]
    fn hashes_are_collected_from_policy_rules() {
        let rules = vec![
            crate::usbguard::PolicyRule::from_pair(0, r#"allow hash "AAA=""#).unwrap(),
            crate::usbguard::PolicyRule::from_pair(1, r#"block id 1234:5678"#).unwrap(),
            crate::usbguard::PolicyRule::from_pair(2, r#"reject hash "BBB=""#).unwrap(),
        ];
        let hashes = hashes_from_rules(&rules);
        assert_eq!(hashes.len(), 2);
        assert!(hashes.contains("AAA="));
        assert!(hashes.contains("BBB="));
    }
}
