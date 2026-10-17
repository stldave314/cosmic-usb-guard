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
use crate::usbguard::{Device, DeviceKey, Event, Health, PolicyRule, PresenceEvent, Target};

/// A hook being edited, before it is saved.
///
/// Kept as strings because that is what the text inputs hold; conversion and
/// validation happen once, on save, so a half-typed path is never written to
/// the config and never runs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HookDraft {
    /// Descriptor hash of the device being configured.
    pub hash: String,
    /// What the user is calling it.
    pub label: String,
    /// Absolute path to the program.
    pub program: String,
    /// Arguments, one per line.
    ///
    /// Newline-separated rather than space-separated because there is no shell
    /// to do the splitting, so a space-separated field would silently make
    /// `--dir /my documents` into three arguments.
    pub args: String,
    /// Whether it is active.
    pub enabled: bool,
}

impl HookDraft {
    /// Start editing a device's hook, or a blank one if it has none.
    pub fn new(hash: String, existing: Option<&crate::hooks::Hook>) -> Self {
        match existing {
            Some(hook) => Self {
                hash,
                label: hook.label.clone(),
                program: hook.program.display().to_string(),
                args: hook.args.join("\n"),
                enabled: hook.enabled,
            },
            None => Self {
                hash,
                enabled: true,
                ..Self::default()
            },
        }
    }

    /// The hook this draft describes.
    pub fn to_hook(&self) -> crate::hooks::Hook {
        crate::hooks::Hook {
            hash: self.hash.clone(),
            program: std::path::PathBuf::from(self.program.trim()),
            args: self
                .args
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect(),
            enabled: self.enabled,
            label: self.label.trim().to_string(),
        }
    }
}

/// A device waiting for the user to decide about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pending {
    /// Daemon-assigned device ID.
    pub device_id: u32,
    /// Descriptor hash of the device the prompt was raised for.
    ///
    /// The daemon reuses IDs. A notification can outlive its device — the
    /// prompt says "Allow" for one thing, the user clicks it minutes later,
    /// and by then the ID may belong to something else. A click only acts if
    /// the device holding the ID still has this hash.
    pub hash: String,
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

/// A Thunderbolt device waiting for the user to allow it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThunderboltPending {
    /// bolt's UID for the device.
    pub uid: String,
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
    /// Tell the user a device was refused without being asked about.
    NotifyAutoBlocked(u32),
    /// Show the window, because a decision is waiting.
    ShowWindow,
    /// Run this device's configured hook program.
    RunHook(u32),
    /// Re-read the journal from disk.
    ReloadHistory,
    /// Post a notification asking about this Thunderbolt device.
    NotifyThunderbolt(String),
    /// Show the window at the Thunderbolt page, because a device is waiting.
    ShowThunderbolt,
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
    /// Devices the daemon currently reports as present.
    pub devices: Vec<Device>,
    /// Devices reconstructed from standing policy rules.
    ///
    /// Kept unfiltered; [`State::remembered_devices`] drops the ones that are
    /// also plugged in, so a device that arrives between a policy read and a
    /// device read cannot appear in both lists at once.
    policy_devices: Vec<Device>,
    /// Installation health.
    pub health: Health,
    /// Whether health has been evaluated at least once.
    ///
    /// Distinguishes "not checked yet" from "checked and broken"; without it
    /// the UI would flash a scary red banner every time it starts.
    pub health_checked: bool,
    /// Devices awaiting a decision.
    pub pending: Vec<Pending>,
    /// Prompts the user dismissed, by daemon ID and descriptor hash, until the
    /// device is unplugged or plugged in again.
    ///
    /// Without this a dismissal lasted only until the next refresh, which
    /// found the device still blocked and asked again.
    dismissed: HashSet<(u32, String)>,
    /// Device whose details are expanded.
    pub selected: Option<DeviceKey>,
    /// Whether the next decision is written as a standing rule.
    pub permanent: bool,
    /// The device whose prompt has "bind to this port" switched on.
    ///
    /// Held per device rather than as a flag, and never persisted: binding is
    /// a deliberate choice about one device. A plain flag stayed on after
    /// that device was decided, and the next device's prompt arrived with the
    /// switch already set.
    pub bind: Option<u32>,
    /// Devices with an in-flight request, so their buttons can be disabled.
    pub busy: HashSet<DeviceKey>,
    /// The last error worth showing the user.
    pub error: Option<String>,
    /// Journal entries for the history view.
    pub history: Vec<Entry>,
    /// Which entries the history view lists.
    pub history_filter: HistoryFilter,
    /// The hook currently being edited, if any.
    pub hook_draft: Option<HookDraft>,
    /// Hashes whose hook has already run since the device was last connected.
    ///
    /// A device is announced by an insert signal and again by whatever policy
    /// signal follows it; without this the hook would run twice for one plug.
    /// Cleared when the device is removed, so replugging runs it again.
    hooks_fired: HashSet<String>,
    /// What the policy says about each device hash it pins a rule to.
    ///
    /// The target matters, not just the presence of a rule: undoing a decision
    /// means noticing that the standing rule contradicts what the user is now
    /// asking for.
    standing: HashMap<String, Target>,
    /// The first rule the policy pins to each device hash.
    ///
    /// The rule the daemon acts on, kept whole so its port binding can be
    /// read and so it can be rewritten with a different one.
    standing_rules: HashMap<String, PolicyRule>,
    /// The whole policy, in the order the daemon evaluates it.
    policy: Vec<PolicyRule>,
    /// Whether `standing` reflects a successful policy read.
    standing_loaded: bool,
    /// Thunderbolt devices and domains, once read. `None` before the first
    /// read and on machines without Thunderbolt or USB4.
    pub thunderbolt: Option<crate::thunderbolt::Snapshot>,
    /// Thunderbolt devices waiting to be allowed.
    pub thunderbolt_pending: Vec<ThunderboltPending>,
    /// Thunderbolt devices with a request in flight, by UID.
    pub thunderbolt_busy: HashSet<String>,
    /// Thunderbolt devices the user dismissed, until they stop waiting.
    ///
    /// Without this a dismissal lasted only until the next refresh, which
    /// found the device still waiting and asked again.
    thunderbolt_dismissed: HashSet<String>,
    /// UIDs that were waiting at the last *successful* bolt read, or `None`
    /// before the first one.
    ///
    /// The baseline for what counts as newly arrived. A failed read does not
    /// replace it: if it did, the first good read after a hiccup would
    /// announce every device that had been waiting all along.
    thunderbolt_baseline: Option<HashSet<String>>,
    /// Whether journal entries are written to disk as well as kept here.
    ///
    /// Off only in tests. A test that exercises journaling must never append
    /// to the real file: every `cargo test` run would otherwise leave a
    /// fabricated event — "the daemon failed" — in the user's history, where
    /// it is indistinguishable from a real one.
    persist_journal: bool,
}

impl Default for State {
    fn default() -> Self {
        let config = ConfigState::load();
        Self {
            permanent: config.settings.default_permanent,
            bind: None,
            config,
            connected: false,
            disconnect_reason: String::new(),
            devices: Vec::new(),
            policy_devices: Vec::new(),
            health: Health::default(),
            health_checked: false,
            pending: Vec::new(),
            dismissed: HashSet::new(),
            selected: None,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            hook_draft: None,
            hooks_fired: HashSet::new(),
            standing: HashMap::new(),
            standing_rules: HashMap::new(),
            policy: Vec::new(),
            standing_loaded: false,
            thunderbolt: None,
            thunderbolt_pending: Vec::new(),
            thunderbolt_busy: HashSet::new(),
            thunderbolt_dismissed: HashSet::new(),
            thunderbolt_baseline: None,
            persist_journal: true,
        }
    }
}

impl State {
    /// Fresh state with settings loaded from disk.
    pub fn new() -> Self {
        Self::default()
    }

    // -- queries ----------------------------------------------------------

    /// Look up a connected device by daemon ID.
    pub fn device(&self, id: u32) -> Option<&Device> {
        self.devices.iter().find(|d| d.daemon_id() == Some(id))
    }

    /// Look up any device the interface can refer to, connected or remembered.
    pub fn device_by_key(&self, key: &DeviceKey) -> Option<&Device> {
        match key {
            DeviceKey::Connected(id) => self.device(*id),
            DeviceKey::Remembered(hash) => self
                .policy_devices
                .iter()
                .find(|d| &d.hash == hash)
                .filter(|_| !self.is_connected_hash(hash)),
        }
    }

    /// How the interface refers to the device with this hash: the plugged-in
    /// one if there is one, otherwise the remembered one.
    pub fn key_for_hash(&self, hash: &str) -> Option<DeviceKey> {
        if hash.is_empty() {
            return None;
        }
        self.devices
            .iter()
            .chain(self.policy_devices.iter())
            .find(|d| d.hash == hash)
            .map(Device::key)
    }

    /// Whether the user has marked this device as part of the machine.
    ///
    /// Keyed on the descriptor hash, the same identity permanent rules use: a
    /// mark keyed on the USB ID would follow any device that claimed those IDs,
    /// which is the property this app exists to deny.
    pub fn is_internal(&self, device: &Device) -> bool {
        !device.hash.is_empty() && self.config.settings.internal_hashes.contains(&device.hash)
    }

    /// Whether a device with this hash is plugged in right now.
    fn is_connected_hash(&self, hash: &str) -> bool {
        !hash.is_empty() && self.devices.iter().any(|d| d.hash == hash)
    }

    /// Whether a device should be shown given the current filters.
    ///
    /// A device awaiting a decision is always shown, whatever the filters say:
    /// hiding something the user must act on would be a bug, not a preference.
    pub fn is_visible(&self, device: &Device) -> bool {
        if device.daemon_id().is_some_and(|id| self.is_pending(id)) {
            return true;
        }
        if self.is_internal(device) && !self.config.settings.show_internal {
            return false;
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
                .then_with(|| a.daemon_id().cmp(&b.daemon_id()))
        });
        devices
    }

    /// Devices the policy remembers that are not plugged in right now.
    ///
    /// This is how a decision stays reversible after the device is unplugged.
    /// Without it a permanent block or reject can only be undone by plugging
    /// the device back in — and a *rejected* device is detached from the
    /// system on sight, so replugging it does not reliably bring back a row to
    /// click either.
    pub fn remembered_devices(&self) -> Vec<&Device> {
        if !self.config.settings.show_disconnected {
            return Vec::new();
        }
        let mut devices: Vec<&Device> = self
            .policy_devices
            .iter()
            .filter(|device| !self.is_connected_hash(&device.hash))
            .filter(|device| self.is_visible(device))
            .collect();

        devices.sort_by(|a, b| {
            sort_rank(self, a).cmp(&sort_rank(self, b)).then_with(|| {
                a.display_name()
                    .to_lowercase()
                    .cmp(&b.display_name().to_lowercase())
            })
        });
        devices
    }

    /// How many devices the current filters are hiding.
    ///
    /// Counts what the two lists actually drop, which for remembered devices
    /// includes the whole section when `show_disconnected` is off —
    /// [`State::is_visible`] does not know about that filter, so counting
    /// through it alone would under-report and the caption would claim fewer
    /// devices were hidden than the list is hiding.
    pub fn hidden_count(&self) -> usize {
        let connected = self
            .devices
            .iter()
            .filter(|device| !self.is_visible(device))
            .count();

        let remembered = self
            .policy_devices
            .iter()
            .filter(|d| !self.is_connected_hash(&d.hash));

        let remembered = if self.config.settings.show_disconnected {
            remembered.filter(|d| !self.is_visible(d)).count()
        } else {
            remembered.count()
        };

        connected + remembered
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

    /// What the policy will do to this device the next time it is connected.
    ///
    /// Returns `None` when the policy could not be read, which makes the prompt
    /// logic fail towards asking rather than towards silence.
    pub fn standing_target(&self, device: &Device) -> Option<Target> {
        if !self.standing_loaded || device.hash.is_empty() {
            return None;
        }
        self.standing.get(&device.hash).copied()
    }

    /// Whether a device has a standing policy rule pinned to its hash.
    pub fn has_standing_rule(&self, device: &Device) -> bool {
        self.standing_target(device).is_some()
    }

    /// A rule earlier in the policy that could decide about this device before
    /// its own rule is reached.
    ///
    /// The daemon applies the first rule that matches, and the rules this app
    /// writes go at the end. An older, broader rule — `allow id 0781:5567`, or
    /// `block with-interface 08:*:*` — ahead of them can therefore overrule a
    /// decision that reported success. Only devices with a standing rule are
    /// considered: for the rest, whatever matches first *is* their rule.
    pub fn overriding_rule(&self, device: &Device) -> Option<&PolicyRule> {
        if !self.standing_loaded || device.hash.is_empty() {
            return None;
        }
        let own = self
            .policy
            .iter()
            .position(|rule| rule.hash() == Some(device.hash.as_str()))?;
        self.policy[..own]
            .iter()
            .find(|rule| rule.may_match(device))
    }

    /// The port this device's standing rule is bound to, if it is bound.
    pub fn standing_port(&self, device: &Device) -> Option<&str> {
        if !self.standing_loaded || device.hash.is_empty() {
            return None;
        }
        self.standing_rules.get(&device.hash)?.port()
    }

    /// Where a plugged-in device's rule is bound, when it is somewhere else.
    ///
    /// Returns `(bound, now)`. The rule does not match on any other port, so
    /// the daemon has fallen back to blocking the device: from the user's
    /// side a device they allowed for good has stopped working, and this is
    /// the reason.
    pub fn port_mismatch(&self, device: &Device) -> Option<(String, String)> {
        let bound = self.standing_port(device)?;
        (device.is_connected() && bound != device.via_port)
            .then(|| (bound.to_string(), device.via_port.clone()))
    }

    /// Whether the standing rule actually covers this device where it is.
    ///
    /// A rule bound to another port is a standing rule that does not apply,
    /// and treating it as one would leave the device blocked with nobody
    /// asked about it.
    pub fn standing_rule_applies(&self, device: &Device) -> bool {
        self.has_standing_rule(device) && self.port_mismatch(device).is_none()
    }

    /// The standing rule rewritten bound to the device's current port, or
    /// released from any port.
    ///
    /// `None` when there is no standing rule, or when binding is asked for a
    /// device that is not plugged in — it has no current port to bind to.
    pub fn rebound_rule(&self, device: &Device, bind: bool) -> Option<String> {
        if !self.standing_loaded || device.hash.is_empty() {
            return None;
        }
        let rule = self.standing_rules.get(&device.hash)?;
        let port = if bind {
            if !device.can_bind_port() {
                return None;
            }
            Some(device.via_port.as_str())
        } else {
            None
        };
        crate::usbguard::rule::with_via_port(&rule.rule.raw, port)
    }

    /// Whether the standing rule contradicts the device's live authorization.
    ///
    /// This is the state a one-off decision leaves behind: the device is
    /// allowed (or blocked) now, and the rule will undo that on the next
    /// replug. Worth saying out loud rather than letting the user find out.
    pub fn standing_rule_conflicts(&self, device: &Device) -> Option<Target> {
        let standing = self.standing_target(device)?;
        (device.is_connected() && standing != device.target).then_some(standing)
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
        let existing: Vec<(u32, String)> = self
            .devices
            .iter()
            .filter(|device| self.needs_decision(device))
            .filter_map(|device| Some((device.daemon_id()?, device.hash.clone())))
            .collect();

        let mut effects = Vec::new();

        // Drop prompts for devices that are no longer present or no longer
        // blocked; a stale prompt would apply a decision to whatever device
        // has since inherited that ID.
        effects.extend(self.retain_pending(|state, pending| {
            state
                .device(pending.device_id)
                .is_some_and(|device| device.hash == pending.hash && state.needs_decision(device))
        }));

        // Forget dismissals for devices that have gone.
        let present: HashSet<(u32, String)> = self
            .devices
            .iter()
            .filter_map(|d| Some((d.daemon_id()?, d.hash.clone())))
            .collect();
        self.dismissed.retain(|key| present.contains(key));

        for (id, hash) in existing {
            if !self.is_pending(id) && !self.dismissed.contains(&(id, hash.clone())) {
                self.pending.push(Pending {
                    device_id: id,
                    hash,
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
        // present stale authorizations as live ones.
        self.devices.clear();
        self.policy_devices.clear();
        self.standing_loaded = false;

        // Withdraw prompts we can no longer act on.
        let effects = self.retain_pending(|_, _| false);
        debug_log!(crate::debug::UI, "disconnected: {}", self.disconnect_reason);
        effects
    }

    fn on_presence(&mut self, device: Device, event: PresenceEvent) -> Vec<Effect> {
        let mut effects = Vec::new();

        match event {
            PresenceEvent::Remove => {
                let key = device.key();
                self.devices.retain(|d| d.key() != key);
                if self.selected.as_ref() == Some(&key) {
                    self.selected = None;
                }
                self.busy.remove(&key);
                // So the hook runs again the next time it is plugged in.
                self.hooks_fired.remove(&device.hash);
                // And so it is asked about again.
                if let Some(id) = device.daemon_id() {
                    self.dismissed.remove(&(id, device.hash.clone()));
                }
                effects.extend(self.retain_pending(|_, p| Some(p.device_id) != device.daemon_id()));
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

        // A hook belongs to a device that is already authorized, so an insert
        // that arrives allowed — because a standing rule allowed it — is the
        // moment to run it.
        effects.extend(self.hook_effect(&device));

        // A device with a standing block or reject rule is refused without
        // anyone being asked, which from the user's side looks like the device
        // simply not working. Nothing else would ever mention it.
        if event == PresenceEvent::Insert
            && let Some(id) = device.daemon_id()
            && !self.needs_decision(&device)
            && device.target != Target::Allow
            && !self.is_internal(&device)
            && !device.is_root_hub()
            && self.config.settings.notify_on_auto_block
        {
            debug_log!(
                crate::debug::DEVICE,
                "device {id} was refused by policy without a prompt"
            );
            effects.push(Effect::NotifyAutoBlocked(id));
        }

        if let (PresenceEvent::Insert, Some(id)) = (event, device.daemon_id())
            && self.needs_decision(&device)
        {
            if !self.is_pending(id) {
                self.pending.push(Pending {
                    device_id: id,
                    hash: device.hash.clone(),
                    live: true,
                    notification_id: None,
                });
            }

            if self.config.settings.prompt_on_insert {
                if self.config.settings.notify_on_insert {
                    effects.push(Effect::Notify(id));
                }
                if self.config.settings.auto_open_popup {
                    effects.push(Effect::ShowWindow);
                }
            }

            debug_log!(
                crate::debug::DEVICE,
                "device {id} inserted and needs a decision"
            );
        }

        effects
    }

    fn on_policy(&mut self, device: Device, old: Target, new: Target) -> Vec<Effect> {
        self.upsert(device.clone());
        self.busy.remove(&device.key());

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
            effects.extend(self.retain_pending(|_, p| Some(p.device_id) != device.daemon_id()));
            effects.extend(self.hook_effect(&device));
        } else if self.config.settings.notify_on_auto_block
            && !self.is_internal(&device)
            && !device.is_root_hub()
            && let Some(id) = device.daemon_id()
        {
            // Something de-authorized a device that was working. The user did
            // not do it here, or this would have been journalled as their own.
            effects.push(Effect::NotifyAutoBlocked(id));
        }
        effects
    }

    /// Whether `device` should run its hook now, marking it as run if so.
    ///
    /// The authorization requirement is enforced again in [`crate::hooks::run`];
    /// this only decides *when* to ask, and stops one plug from running the
    /// hook twice because both an insert and a policy signal described it.
    fn hook_effect(&mut self, device: &Device) -> Option<Effect> {
        let id = device.daemon_id()?;
        if device.target != Target::Allow || device.hash.is_empty() {
            return None;
        }
        let hook = self.config.settings.hook(&device.hash)?;
        if !hook.enabled || hook.program.as_os_str().is_empty() {
            return None;
        }
        if !self.hooks_fired.insert(device.hash.clone()) {
            return None;
        }
        debug_log!(crate::debug::HOOK, "queueing hook for device {id}");
        Some(Effect::RunHook(id))
    }

    /// Begin editing a device's hook.
    pub fn begin_hook(&mut self, hash: String) {
        let existing = self.config.settings.hook(&hash).cloned();
        self.hook_draft = Some(HookDraft::new(hash, existing.as_ref()));
    }

    /// Save the draft, if there is one.
    pub fn save_hook(&mut self) {
        let Some(draft) = self.hook_draft.take() else {
            return;
        };
        let hook = draft.to_hook();
        self.set_hook(draft.hash, Some(hook));
    }

    /// Replace a device's hook, or remove it when `hook` is `None`.
    pub fn set_hook(&mut self, hash: String, hook: Option<crate::hooks::Hook>) {
        if hash.is_empty() {
            return;
        }
        self.config.update(|settings| {
            settings.hooks.retain(|h| h.hash != hash);
            if let Some(hook) = hook {
                settings.hooks.push(hook);
            }
        });
        // Let a newly saved hook run for a device that is already plugged in,
        // rather than making the user unplug it to test what they just set up.
        self.hooks_fired.remove(&hash);
    }

    /// The hook configured for a device, if any.
    pub fn hook(&self, device: &Device) -> Option<&crate::hooks::Hook> {
        self.config.settings.hook(&device.hash)
    }

    /// Record a decision this app made.
    pub fn record_decision(&mut self, device: &Device, target: Target, permanent: bool) {
        self.record(
            Entry::device(Kind::for_target(target), Actor::User, device).permanent(permanent),
        );
    }

    /// Record a Thunderbolt action this app made.
    pub fn record_thunderbolt(
        &mut self,
        device: &crate::thunderbolt::Device,
        action: crate::tasks::ThunderboltAction,
    ) {
        use crate::tasks::ThunderboltAction;
        let (kind, permanent) = match action {
            ThunderboltAction::Authorize => (Kind::Allowed, false),
            ThunderboltAction::Enroll => (Kind::Allowed, true),
            ThunderboltAction::Forget => (Kind::Revoked, false),
        };
        self.record(
            Entry::thunderbolt(kind, Actor::User, device.display_name()).permanent(permanent),
        );
    }

    /// Record a revocation this app made.
    pub fn record_revocation(&mut self, device: &Device, rules_removed: usize) {
        self.record(
            Entry::device(Kind::Revoked, Actor::User, device)
                .with_detail(format!("{rules_removed} rule(s) removed")),
        );
    }

    /// Stop asking about a device until it is unplugged or plugged in again,
    /// returning any notification to withdraw. The device stays as USBGuard
    /// left it: blocked.
    pub fn dismiss_pending(&mut self, device_id: u32) -> Vec<Effect> {
        if let Some(device) = self.device(device_id) {
            self.dismissed.insert((device_id, device.hash.clone()));
        }
        self.resolve_pending(device_id)
    }

    /// Clear a device's prompt, returning any notification to withdraw.
    pub fn resolve_pending(&mut self, device_id: u32) -> Vec<Effect> {
        self.retain_pending(|_, p| p.device_id != device_id)
    }

    /// Attach a posted notification's ID to its prompt so it can be withdrawn
    /// once the decision is made.
    ///
    /// Returns `false` when the prompt has already gone, so the caller can
    /// withdraw the notification instead of leaving it unanswerable.
    pub fn set_notification(&mut self, device_id: u32, notification_id: u32) -> bool {
        match self.pending.iter_mut().find(|p| p.device_id == device_id) {
            Some(pending) => {
                pending.notification_id = Some(notification_id);
                true
            }
            None => false,
        }
    }

    /// The device a notification belongs to.
    ///
    /// `None` when the device holding that ID is no longer the one the
    /// notification described — see [`Pending::hash`].
    pub fn device_for_notification(&self, notification_id: u32) -> Option<u32> {
        let pending = self
            .pending
            .iter()
            .find(|p| p.notification_id == Some(notification_id))?;
        self.device(pending.device_id)
            .is_some_and(|device| device.hash == pending.hash)
            .then_some(pending.device_id)
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

    /// Replace the Thunderbolt picture, returning what the binary must do.
    ///
    /// A device counts as newly arrived only if it was not already waiting in
    /// the previous snapshot, and nothing counts as arrived on the first read:
    /// the same rule as for USB, so a dock left plugged in overnight is not
    /// announced as new at every login.
    pub fn set_thunderbolt(&mut self, snapshot: crate::thunderbolt::Snapshot) -> Vec<Effect> {
        // A failed read says nothing about the devices. Show the error, and
        // leave the prompts and the arrival baseline exactly as they were.
        if snapshot.error.is_some() {
            self.thunderbolt = Some(snapshot);
            return Vec::new();
        }

        // Only devices the controller would actually authorize one by one. At
        // a level that never tunnels PCIe there is nothing to allow, and an
        // Allow button would only fail.
        let waiting: Vec<String> = snapshot
            .devices
            .iter()
            .filter(|d| d.is_waiting() && snapshot.asks_about(d))
            .map(|d| d.uid.clone())
            .collect();
        self.thunderbolt = Some(snapshot);

        let first = self.thunderbolt_baseline.is_none();
        let previously_waiting = self
            .thunderbolt_baseline
            .replace(waiting.iter().cloned().collect())
            .unwrap_or_default();

        // A dismissal lasts until the device stops waiting: plugged in again,
        // it is asked about again.
        self.thunderbolt_dismissed
            .retain(|uid| waiting.contains(uid));

        // Withdraw prompts for devices that are no longer waiting: allowed
        // elsewhere, or unplugged.
        let (kept, dropped): (Vec<_>, Vec<_>) = std::mem::take(&mut self.thunderbolt_pending)
            .into_iter()
            .partition(|p| waiting.contains(&p.uid));
        self.thunderbolt_pending = kept;
        let mut effects: Vec<Effect> = dropped
            .into_iter()
            .filter_map(|p| p.notification_id.map(Effect::CloseNotification))
            .collect();

        for uid in waiting {
            if self.thunderbolt_dismissed.contains(&uid)
                || self.thunderbolt_pending.iter().any(|p| p.uid == uid)
            {
                continue;
            }
            self.thunderbolt_pending.push(ThunderboltPending {
                uid: uid.clone(),
                notification_id: None,
            });

            let arrived = !first && !previously_waiting.contains(&uid);
            if arrived && self.config.settings.prompt_on_insert {
                debug_log!(crate::debug::THUNDERBOLT, "{uid} is waiting to be allowed");
                if self.config.settings.notify_on_insert {
                    effects.push(Effect::NotifyThunderbolt(uid));
                }
                if self.config.settings.auto_open_popup {
                    effects.push(Effect::ShowThunderbolt);
                }
            }
        }

        effects
    }

    /// Thunderbolt devices waiting to be allowed, in arrival order.
    pub fn thunderbolt_waiting(&self) -> Vec<&crate::thunderbolt::Device> {
        let Some(snapshot) = self.thunderbolt.as_ref() else {
            return Vec::new();
        };
        self.thunderbolt_pending
            .iter()
            .filter_map(|p| snapshot.device(&p.uid))
            .collect()
    }

    /// Stop prompting about a Thunderbolt device until it stops waiting,
    /// returning any notification to withdraw. The device stays exactly as it
    /// was: not authorized.
    pub fn dismiss_thunderbolt(&mut self, uid: &str) -> Vec<Effect> {
        self.thunderbolt_dismissed.insert(uid.to_string());
        self.resolve_thunderbolt(uid)
    }

    /// Clear a Thunderbolt prompt, returning any notification to withdraw.
    pub fn resolve_thunderbolt(&mut self, uid: &str) -> Vec<Effect> {
        let (kept, dropped): (Vec<_>, Vec<_>) = std::mem::take(&mut self.thunderbolt_pending)
            .into_iter()
            .partition(|p| p.uid != uid);
        self.thunderbolt_pending = kept;
        dropped
            .into_iter()
            .filter_map(|p| p.notification_id.map(Effect::CloseNotification))
            .collect()
    }

    /// Attach a posted notification to a Thunderbolt prompt.
    ///
    /// Returns `false` when the prompt has already gone — the device was
    /// allowed or unplugged while the notification was being posted — so the
    /// caller can withdraw it rather than leave a prompt nobody can answer.
    pub fn set_thunderbolt_notification(&mut self, uid: &str, notification_id: u32) -> bool {
        match self.thunderbolt_pending.iter_mut().find(|p| p.uid == uid) {
            Some(pending) => {
                pending.notification_id = Some(notification_id);
                true
            }
            None => false,
        }
    }

    /// The Thunderbolt device a notification belongs to.
    pub fn thunderbolt_for_notification(&self, notification_id: u32) -> Option<String> {
        self.thunderbolt_pending
            .iter()
            .find(|p| p.notification_id == Some(notification_id))
            .map(|p| p.uid.clone())
    }

    /// Everything waiting for a decision, USB and Thunderbolt together.
    pub fn pending_count(&self) -> usize {
        self.pending.len() + self.thunderbolt_pending.len()
    }

    /// Replace everything derived from the daemon's rule set.
    pub fn set_policy(&mut self, rules: &[PolicyRule]) {
        self.standing = standing_targets(rules);
        self.standing_rules = first_rule_per_hash(rules);
        self.policy = rules.to_vec();
        self.standing_loaded = true;
        self.policy_devices = remembered_devices(rules);
        debug_log!(
            crate::debug::POLICY,
            "policy: {} rule(s), {} pinned device(s)",
            rules.len(),
            self.policy_devices.len()
        );
    }

    /// Mark a device as part of this machine, or unmark it.
    ///
    /// Marking is a display and prompting preference, never an authorization:
    /// an internal device stays exactly as USBGuard has it until the user
    /// allows it explicitly. What it does change is that the app stops asking
    /// about it on every boot, which is the whole reason for the mark.
    ///
    /// Returns the effects of withdrawing any prompt already raised for it, so
    /// marking something internal takes its notification down with it.
    pub fn set_internal(&mut self, hash: String, internal: bool) -> Vec<Effect> {
        if hash.is_empty() {
            return Vec::new();
        }

        self.config.update(|settings| {
            settings.internal_hashes.retain(|h| h != &hash);
            if internal {
                settings.internal_hashes.push(hash.clone());
            }
        });

        if !internal {
            return Vec::new();
        }

        let marked: Vec<u32> = self
            .devices
            .iter()
            .filter(|d| d.hash == hash)
            .filter_map(Device::daemon_id)
            .collect();
        self.retain_pending(|_, p| !marked.contains(&p.device_id))
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
            && !self.is_internal(device)
            && !self.standing_rule_applies(device)
    }

    fn upsert(&mut self, device: Device) {
        match self.devices.iter_mut().find(|d| d.key() == device.key()) {
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
        if self.persist_journal {
            journal::append(&entry);
        }
        self.history.push(entry);
    }
}

/// Sort key: prompts first, then blocked devices, then everything else.
fn sort_rank(state: &State, device: &Device) -> u8 {
    if device.daemon_id().is_some_and(|id| state.is_pending(id)) {
        0
    } else if device.target != Target::Allow {
        1
    } else {
        2
    }
}

/// What the policy does to each device hash it pins a rule to.
///
/// The *first* rule wins, not the last. `usbguard-rules.conf(5)`: "the daemon
/// scans the existing rules sequentially. If a matching rule is found, it
/// either authorizes (allows), deauthorizes (blocks) or removes (rejects) the
/// device". Folding later rules over earlier ones would report the opposite of
/// what the daemon will actually do whenever a policy holds two rules for one
/// device.
pub fn standing_targets(rules: &[PolicyRule]) -> HashMap<String, Target> {
    let mut targets = HashMap::new();
    for rule in rules {
        if let Some(hash) = rule.hash() {
            targets.entry(hash.to_string()).or_insert(rule.rule.target);
        }
    }
    targets
}

/// The first rule pinned to each device hash, which is the one the daemon
/// applies — see [`standing_targets`].
pub fn first_rule_per_hash(rules: &[PolicyRule]) -> HashMap<String, PolicyRule> {
    let mut first = HashMap::new();
    for rule in rules {
        if let Some(hash) = rule.hash() {
            first
                .entry(hash.to_string())
                .or_insert_with(|| rule.clone());
        }
    }
    first
}

/// The devices a policy listing pins rules to, one entry per device.
///
/// Only the first rule for a given hash becomes an entry, for the same reason
/// [`standing_targets`] keeps the first: that is the rule the daemon will act
/// on, so it is the one the user needs to see and be able to change.
pub fn remembered_devices(rules: &[PolicyRule]) -> Vec<Device> {
    let mut seen = HashSet::new();
    rules
        .iter()
        .filter_map(PolicyRule::device)
        .filter(|device| seen.insert(device.hash.clone()))
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
            policy_devices: Vec::new(),
            health: Health::default(),
            health_checked: false,
            pending: Vec::new(),
            dismissed: HashSet::new(),
            selected: None,
            permanent: false,
            bind: None,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            hook_draft: None,
            hooks_fired: HashSet::new(),
            standing: HashMap::new(),
            standing_rules: HashMap::new(),
            policy: Vec::new(),
            standing_loaded: false,
            thunderbolt: None,
            thunderbolt_pending: Vec::new(),
            thunderbolt_busy: HashSet::new(),
            thunderbolt_dismissed: HashSet::new(),
            thunderbolt_baseline: None,
            persist_journal: false,
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
        assert!(effects.contains(&Effect::ShowWindow));
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
        assert!(!effects.contains(&Effect::ShowWindow));
        assert!(!state.pending[0].live);
    }

    #[test]
    fn hardwired_and_root_hub_devices_are_never_prompted_about() {
        let mut state = state();
        let root_hub = Device::from_rule(
            1,
            r#"block id 1d6b:0002 hash "R=" via-port "usb1" with-connect-type "hardwired""#,
        )
        .unwrap();
        state.apply_event(Event::Connected {
            devices: vec![root_hub],
        });
        assert!(state.pending.is_empty());
    }

    /// A policy listing built from rule text, the way `listRules` returns it.
    fn policy(rules: &[&str]) -> Vec<PolicyRule> {
        rules
            .iter()
            .enumerate()
            .map(|(i, text)| PolicyRule::from_pair(i as u32, text).unwrap())
            .collect()
    }

    #[test]
    fn a_device_with_a_standing_rule_is_not_prompted_about() {
        let mut state = state();
        state.set_policy(&policy(&[r#"allow hash "H1=""#]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        assert!(state.pending.is_empty());
    }

    #[test]
    fn a_device_on_the_port_its_rule_is_bound_to_is_not_prompted_about() {
        let mut state = state();
        state.set_policy(&policy(&[r#"allow hash "H1=" via-port "1-1""#]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        assert!(state.pending.is_empty());
        assert_eq!(state.port_mismatch(&state.devices[0]), None);
    }

    #[test]
    fn a_bound_device_on_another_port_is_asked_about_and_told_why() {
        // The rule does not match there, so the daemon blocked it. Treating
        // the rule as covering it would leave a working device dead with
        // nobody asked and nothing said.
        let mut state = state();
        state.set_policy(&policy(&[r#"allow hash "H1=" via-port "3-5""#]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        assert!(state.is_pending(1));
        assert_eq!(
            state.port_mismatch(&state.devices[0]),
            Some(("3-5".to_string(), "1-1".to_string()))
        );
    }

    #[test]
    fn a_standing_rule_is_rebound_to_the_current_port_or_released() {
        let mut state = state();
        state.set_policy(&policy(&[r#"allow id 0781:5561 hash "H1=""#]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        let plugged = state.devices[0].clone();

        let bound = state.rebound_rule(&plugged, true).unwrap();
        assert_eq!(bound, r#"allow id 0781:5561 hash "H1=" via-port "1-1""#);

        state.set_policy(&policy(&[&bound]));
        assert_eq!(state.standing_port(&plugged), Some("1-1"));
        assert_eq!(
            state.rebound_rule(&plugged, false).unwrap(),
            r#"allow id 0781:5561 hash "H1=""#
        );
    }

    #[test]
    fn an_unplugged_device_cannot_be_bound_but_can_be_released() {
        let mut state = state();
        state.set_policy(&policy(&[r#"allow hash "H1=" via-port "3-5""#]));
        let remembered = Device::remembered(r#"allow hash "H1=" via-port "3-5""#).unwrap();
        assert_eq!(state.rebound_rule(&remembered, true), None);
        assert_eq!(
            state.rebound_rule(&remembered, false).as_deref(),
            Some(r#"allow hash "H1=""#)
        );
    }

    fn bolt(devices: &[(&str, &str)]) -> crate::thunderbolt::Snapshot {
        crate::thunderbolt::Snapshot {
            devices: devices
                .iter()
                .map(|(uid, status)| crate::thunderbolt::Device {
                    uid: uid.to_string(),
                    name: "Dock".into(),
                    vendor: "Acme".into(),
                    label: String::new(),
                    kind: crate::thunderbolt::model::Kind::Peripheral,
                    status: crate::thunderbolt::Status::parse(status),
                    stored: false,
                    policy: String::new(),
                    domain: String::new(),
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn a_thunderbolt_device_already_waiting_at_startup_is_listed_but_not_announced() {
        let mut state = state();
        let effects = state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert_eq!(state.thunderbolt_pending.len(), 1);
        assert!(effects.is_empty(), "{effects:?}");
    }

    #[test]
    fn a_thunderbolt_device_plugged_in_while_running_is_announced() {
        let mut state = state();
        state.set_thunderbolt(bolt(&[]));
        let effects = state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert!(effects.contains(&Effect::NotifyThunderbolt("dock".into())));
        assert_eq!(state.pending_count(), 1);

        // A second read with nothing new does not announce it again.
        let again = state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert!(again.is_empty(), "{again:?}");
    }

    #[test]
    fn a_thunderbolt_prompt_is_withdrawn_once_the_device_is_allowed() {
        let mut state = state();
        state.set_thunderbolt(bolt(&[]));
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        state.set_thunderbolt_notification("dock", 42);

        let effects = state.set_thunderbolt(bolt(&[("dock", "authorized")]));
        assert!(state.thunderbolt_pending.is_empty());
        assert_eq!(effects, vec![Effect::CloseNotification(42)]);
    }

    #[test]
    fn dismissing_a_thunderbolt_prompt_lasts_until_the_device_stops_waiting() {
        let mut state = state();
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        state.dismiss_thunderbolt("dock");
        assert!(state.thunderbolt_waiting().is_empty());
        // Nothing about the device itself changed.
        assert!(state.thunderbolt.as_ref().unwrap().devices[0].is_waiting());

        // The next refresh used to bring the prompt straight back.
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert!(state.thunderbolt_waiting().is_empty());

        // Unplugged and plugged in again, it is asked about again.
        state.set_thunderbolt(bolt(&[("dock", "disconnected")]));
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert_eq!(state.thunderbolt_waiting().len(), 1);
    }

    #[test]
    fn a_failed_bolt_read_neither_drops_prompts_nor_reannounces_them() {
        let mut state = state();
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert_eq!(state.thunderbolt_pending.len(), 1);

        let failed = crate::thunderbolt::Snapshot {
            error: Some("bolt timed out".into()),
            ..Default::default()
        };
        assert!(state.set_thunderbolt(failed).is_empty());
        assert_eq!(
            state.thunderbolt_pending.len(),
            1,
            "a hiccup dropped the prompt"
        );

        let recovered = state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert!(
            recovered.is_empty(),
            "the dock was announced as new: {recovered:?}"
        );
    }

    #[test]
    fn a_failed_first_read_is_not_the_baseline() {
        // Startup with bolt slow to answer: the dock left plugged in
        // overnight must not be announced once bolt does answer.
        let mut state = state();
        state.set_thunderbolt(crate::thunderbolt::Snapshot {
            error: Some("bolt timed out".into()),
            ..Default::default()
        });
        assert!(
            state
                .set_thunderbolt(bolt(&[("dock", "connected")]))
                .is_empty()
        );
    }

    #[test]
    fn a_notification_for_a_prompt_already_gone_is_reported_for_withdrawal() {
        let mut state = state();
        state.set_thunderbolt(bolt(&[("dock", "connected")]));
        assert!(state.set_thunderbolt_notification("dock", 7));
        assert!(!state.set_thunderbolt_notification("gone", 8));
        assert!(!state.set_notification(99, 9));
    }

    #[test]
    fn a_notification_click_does_not_act_on_a_device_that_inherited_the_id() {
        let mut state = state();
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        state.set_notification(1, 42);
        assert_eq!(state.device_for_notification(42), Some(1));

        // The daemon hands ID 1 to a different device before the click lands.
        let other = Device::from_rule(1, r#"block id 0bad:0bad hash "OTHER=""#).unwrap();
        state.devices = vec![other];
        assert_eq!(state.device_for_notification(42), None);
    }

    #[test]
    fn an_earlier_broad_rule_is_reported_as_overruling_the_devices_own() {
        let mut state = state();
        state.set_policy(&policy(&[
            r#"allow id 0781:5561"#,
            r#"block id 0781:5561 hash "H1=""#,
        ]));
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        let overruling = state.overriding_rule(&state.devices[0]).unwrap();
        assert_eq!(overruling.rule.raw, "allow id 0781:5561");

        // Placed after the device's own rule, it never gets a say.
        state.set_policy(&policy(&[
            r#"block id 0781:5561 hash "H1=""#,
            r#"allow id 0781:5561"#,
        ]));
        assert!(state.overriding_rule(&state.devices[0]).is_none());
    }

    #[test]
    fn a_dismissed_prompt_stays_dismissed_until_the_device_is_replugged() {
        let mut state = state();
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        state.dismiss_pending(1);
        assert!(!state.is_pending(1));

        // The next refresh used to put the prompt straight back.
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        assert!(!state.is_pending(1));

        // Unplugged and plugged in again, it is asked about again.
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Remove,
        });
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        assert!(state.is_pending(1));
    }

    #[test]
    fn an_unreadable_policy_fails_towards_asking() {
        // The policy was never loaded, so nothing counts as known and the
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
                steps: None,
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
            hash: "H9=".into(),
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

        let order: Vec<u32> = state
            .visible_devices()
            .iter()
            .filter_map(|d| d.daemon_id())
            .collect();
        // 1 and 2 are both pending (1 from startup, 2 live); 3 is allowed.
        assert_eq!(order.last(), Some(&3));
        assert!(order[..2].contains(&1) && order[..2].contains(&2));
    }

    #[test]
    fn hidden_count_reflects_the_filters() {
        let mut state = state();
        state.config.settings.show_root_hubs = false;
        let hub = Device::from_rule(1, r#"allow id 1d6b:0002 hash "R=" via-port "usb1""#).unwrap();
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
                steps: None,
            }],
        };
        let bad = Health {
            checks: vec![crate::usbguard::health::Check {
                id: crate::usbguard::health::CheckId::DaemonRunning,
                severity: crate::usbguard::Severity::Critical,
                detail: "failed".into(),
                remedy: None,
                steps: None,
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
    fn tests_never_write_to_the_real_journal() {
        // This test used to append a made-up "DaemonRunning: failed" to the
        // user's real history on every run. Measured against the real file,
        // so it fails if a test state ever persists again.
        let size = || {
            journal::path()
                .and_then(|path| std::fs::metadata(path).ok())
                .map(|m| m.len())
        };
        let before = size();

        let mut state = state();
        state.config.settings.journal_enabled = true;
        state.record(Entry::system(Kind::HealthProblem, "from a test"));

        // Recorded in memory, which is what the journaling tests rely on...
        assert_eq!(state.history.len(), 1);
        // ...and nowhere else.
        assert_eq!(size(), before, "a test wrote to the real journal");
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
    fn a_silently_refused_device_is_announced() {
        // The gap this closes: a device with a standing block rule is never
        // prompted about, so without this nothing on screen explains why the
        // drive did not appear.
        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });
        state.set_policy(&policy(&[r#"block hash "H1=""#]));

        let effects = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });

        assert!(!state.is_pending(1), "a standing rule means no prompt");
        assert!(
            effects.contains(&Effect::NotifyAutoBlocked(1)),
            "the refusal must be reported: {effects:?}"
        );
    }

    #[test]
    fn a_device_awaiting_a_decision_is_not_reported_as_auto_blocked() {
        // It gets a prompt, which says everything the notification would.
        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });
        let effects = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        assert!(state.is_pending(1));
        assert!(
            !effects.contains(&Effect::NotifyAutoBlocked(1)),
            "{effects:?}"
        );
    }

    #[test]
    fn an_internal_device_is_never_announced_as_refused() {
        // The point of marking something internal is to stop hearing about it.
        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });
        state.set_policy(&policy(&[r#"block hash "H1=""#]));
        state.set_internal("H1=".to_string(), true);

        let effects = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        assert!(
            !effects.contains(&Effect::NotifyAutoBlocked(1)),
            "{effects:?}"
        );
    }

    #[test]
    fn a_hook_runs_once_per_plug_and_only_when_allowed() {
        let mut state = state();
        state.set_hook(
            "H1=".to_string(),
            Some(crate::hooks::Hook {
                hash: "H1=".to_string(),
                program: std::path::PathBuf::from("/bin/true"),
                args: Vec::new(),
                enabled: true,
                label: "Backup".to_string(),
            }),
        );
        state.apply_event(Event::Connected { devices: vec![] });

        // Blocked: nothing runs.
        let blocked = state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        assert!(!blocked.contains(&Effect::RunHook(1)), "{blocked:?}");

        // Allowed: runs once.
        let allowed = state.apply_event(Event::Policy {
            device: device(1, "allow", ""),
            old: Target::Block,
            new: Target::Allow,
            rule_id: 0,
        });
        assert!(allowed.contains(&Effect::RunHook(1)), "{allowed:?}");

        // A second signal describing the same device must not run it again.
        let repeat = state.apply_event(Event::Presence {
            device: device(1, "allow", ""),
            event: PresenceEvent::Update,
        });
        assert!(!repeat.contains(&Effect::RunHook(1)), "{repeat:?}");

        // Unplugging resets it, so the next plug runs the hook again.
        state.apply_event(Event::Presence {
            device: device(1, "allow", ""),
            event: PresenceEvent::Remove,
        });
        let replug = state.apply_event(Event::Presence {
            device: device(1, "allow", ""),
            event: PresenceEvent::Insert,
        });
        assert!(replug.contains(&Effect::RunHook(1)), "{replug:?}");
    }

    #[test]
    fn a_device_with_no_hook_never_asks_to_run_one() {
        let mut state = state();
        let effects = state.apply_event(Event::Presence {
            device: device(1, "allow", ""),
            event: PresenceEvent::Insert,
        });
        assert!(
            !effects.iter().any(|e| matches!(e, Effect::RunHook(_))),
            "{effects:?}"
        );
    }

    #[test]
    fn standing_targets_are_collected_from_policy_rules() {
        let rules = policy(&[
            r#"allow hash "AAA=""#,
            r#"block id 1234:5678"#,
            r#"reject hash "BBB=""#,
        ]);
        let targets = standing_targets(&rules);
        // The `id`-only rule is not pinned to a device and has no hash to key
        // on, so it contributes nothing.
        assert_eq!(targets.len(), 2);
        assert_eq!(targets.get("AAA="), Some(&Target::Allow));
        assert_eq!(targets.get("BBB="), Some(&Target::Reject));
    }

    #[test]
    fn the_first_matching_rule_wins_not_the_last() {
        // usbguard-rules.conf(5): the daemon "scans the existing rules
        // sequentially" and acts on the first match. Reporting the last one
        // would tell the user the opposite of what will happen, and would make
        // the Allow button look like it had worked when it had not.
        let rules = policy(&[r#"reject hash "AAA=""#, r#"allow hash "AAA=""#]);
        assert_eq!(standing_targets(&rules).get("AAA="), Some(&Target::Reject));
    }

    #[test]
    fn a_permanently_rejected_device_is_still_reachable_once_unplugged() {
        // The case this was built for: a decision made in the prompt outlives
        // the device being connected, so without a row for it there is no way
        // back. The rule text is a real one, as USBGuard wrote it.
        const SANDISK: &str = concat!(
            r#"reject id 0781:55a9 serial "00002931052124091945" "#,
            r#"name " SanDisk 3.2Gen1" hash "ifdabAPA9pgbVCvjqyUPQSihHiNCta+T9OWu2HOXKJQ=" "#,
            r#"with-interface { 08:06:50 08:06:62 } with-connect-type "hotplug""#
        );

        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });
        state.set_policy(&policy(&[SANDISK]));

        let remembered = state.remembered_devices();
        assert_eq!(
            remembered.len(),
            1,
            "the rejected device must still be listed"
        );

        let device = remembered[0];
        assert_eq!(device.display_name(), " SanDisk 3.2Gen1");
        assert!(!device.is_connected());
        assert_eq!(
            device.daemon_id(),
            None,
            "an unplugged device must not carry an ID that applyDevicePolicy could act on"
        );
        assert_eq!(state.standing_target(device), Some(Target::Reject));

        // And the rule it would be replaced with is still pinned to the hash,
        // so undoing the mistake cannot widen it to every SanDisk.
        let rule = device.retargeted_rule(Target::Allow).unwrap();
        assert!(rule.starts_with("allow "), "{rule}");
        assert!(rule.contains(r#"hash "ifdabAPA9pgbVCvjqyUPQSihHiNCta+T9OWu2HOXKJQ=""#));
        assert!(!rule.contains("reject"));
    }

    #[test]
    fn the_hidden_count_matches_what_the_lists_actually_drop() {
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        state.set_policy(&policy(&[r#"block hash "GONE=""#]));

        // Shown: nothing is hidden.
        assert_eq!(state.remembered_devices().len(), 1);
        assert_eq!(state.hidden_count(), 0);

        // Turned off: the whole section is hidden and has to be counted, or
        // the caption tells the user about fewer devices than it is hiding.
        state.config.settings.show_disconnected = false;
        assert!(state.remembered_devices().is_empty());
        assert_eq!(state.hidden_count(), 1);
    }

    #[test]
    fn a_connected_device_is_not_also_listed_as_disconnected() {
        // The device list and the policy are read separately; a device that
        // arrives between the two reads must not appear twice.
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        state.set_policy(&policy(&[r#"allow hash "H1=""#, r#"block hash "GONE=""#]));

        let remembered = state.remembered_devices();
        assert_eq!(remembered.len(), 1);
        assert_eq!(remembered[0].hash, "GONE=");
    }

    #[test]
    fn marking_a_device_internal_stops_it_being_asked_about_and_listed() {
        let mut state = state();
        state.apply_event(Event::Connected { devices: vec![] });
        state.apply_event(Event::Presence {
            device: device(1, "block", ""),
            event: PresenceEvent::Insert,
        });
        assert!(
            state.is_pending(1),
            "precondition: it should be asked about"
        );

        // Marking withdraws the outstanding question as well as hiding it;
        // leaving a prompt up for something the user just called internal
        // would be the nagging the mark exists to stop.
        state.set_internal("H1=".to_string(), true);
        assert!(!state.is_pending(1));
        assert!(state.is_internal(&device(1, "block", "")));
        assert!(!state.is_visible(&device(1, "block", "")));

        // But it is still only a display and prompting preference: the device
        // is untouched, and turning the setting on shows it again.
        state.config.settings.show_internal = true;
        assert!(state.is_visible(&device(1, "block", "")));
    }

    #[test]
    fn an_internal_mark_does_not_authorize_anything() {
        // The mark must never be a back door to allowing a device. Nothing in
        // `set_internal` may change a target.
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "block", "")],
        });
        state.set_internal("H1=".to_string(), true);
        assert_eq!(state.devices[0].target, Target::Block);
    }

    #[test]
    fn an_internal_mark_is_pinned_to_the_hash_not_the_usb_id() {
        // A mark that followed the USB ID would be inherited by any device
        // claiming the same vendor and product, which is exactly the spoofing
        // this app exists to catch.
        let mut state = state();
        state.set_internal("H1=".to_string(), true);

        let impostor = Device::from_rule(
            9,
            r#"block id 0781:5561 name "Device 1" hash "DIFFERENT=" with-connect-type "hotplug""#,
        )
        .unwrap();
        assert!(!state.is_internal(&impostor));
    }

    #[test]
    fn a_device_with_no_hash_cannot_be_marked_internal() {
        // There is nothing durable to key the mark on, so it must be refused
        // rather than stored against an empty string and matching everything.
        let mut state = state();
        state.set_internal(String::new(), true);
        assert!(state.config.settings.internal_hashes.is_empty());

        let hashless = Device::from_rule(1, r#"block id 1234:5678"#).unwrap();
        assert!(!state.is_internal(&hashless));
    }

    #[test]
    fn a_conflicting_standing_rule_is_reported() {
        // A one-off allow leaves a device working now and blocked on the next
        // replug. The UI has to be able to say so.
        let mut state = state();
        state.apply_event(Event::Connected {
            devices: vec![device(1, "allow", "")],
        });
        state.set_policy(&policy(&[r#"block hash "H1=""#]));

        let device = state.device(1).unwrap();
        assert_eq!(state.standing_rule_conflicts(device), Some(Target::Block));

        // Agreement is not a conflict.
        state.set_policy(&policy(&[r#"allow hash "H1=""#]));
        assert_eq!(
            state.standing_rule_conflicts(state.device(1).unwrap()),
            None
        );
    }

    #[test]
    fn device_keys_from_the_two_sources_never_collide() {
        // Daemon device IDs and rule IDs are both small integers. If these
        // ever compared equal, clicking a remembered device would act on
        // whichever connected device happened to share the number.
        let connected = device(1, "block", "");
        let remembered = Device::remembered(r#"block hash "H1=""#).unwrap();
        assert_ne!(connected.key(), remembered.key());
        assert_eq!(connected.key(), DeviceKey::Connected(1));
        assert_eq!(remembered.key(), DeviceKey::Remembered("H1=".to_string()));
    }
}
