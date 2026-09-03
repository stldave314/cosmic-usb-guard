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
    /// Tell the user a device was refused without being asked about.
    NotifyAutoBlocked(u32),
    /// Show the window, because a decision is waiting.
    ShowWindow,
    /// Run this device's configured hook program.
    RunHook(u32),
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
    /// Device whose details are expanded.
    pub selected: Option<DeviceKey>,
    /// Whether the next decision is written as a standing rule.
    pub permanent: bool,
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
    /// Whether `standing` reflects a successful policy read.
    standing_loaded: bool,
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
            policy_devices: Vec::new(),
            health: Health::default(),
            health_checked: false,
            pending: Vec::new(),
            selected: None,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            hook_draft: None,
            hooks_fired: HashSet::new(),
            standing: HashMap::new(),
            standing_loaded: false,
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

    /// Whether the standing rule contradicts the device's live authorisation.
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
        let existing: Vec<u32> = self
            .devices
            .iter()
            .filter(|device| self.needs_decision(device))
            .filter_map(Device::daemon_id)
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

        // A hook belongs to a device that is already authorised, so an insert
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
            // Something de-authorised a device that was working. The user did
            // not do it here, or this would have been journalled as their own.
            effects.push(Effect::NotifyAutoBlocked(id));
        }
        effects
    }

    /// Whether `device` should run its hook now, marking it as run if so.
    ///
    /// The authorisation requirement is enforced again in [`crate::hooks::run`];
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

    /// Replace everything derived from the daemon's rule set.
    pub fn set_policy(&mut self, rules: &[PolicyRule]) {
        self.standing = standing_targets(rules);
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
    /// Marking is a display and prompting preference, never an authorisation:
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
            && !self.has_standing_rule(device)
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
        journal::append(&entry);
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
            selected: None,
            permanent: false,
            busy: HashSet::new(),
            error: None,
            history: Vec::new(),
            history_filter: HistoryFilter::default(),
            hook_draft: None,
            hooks_fired: HashSet::new(),
            standing: HashMap::new(),
            standing_loaded: false,
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
            r#"block id 1d6b:0002 hash "R=" with-connect-type "hardwired""#,
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
    fn an_internal_mark_does_not_authorise_anything() {
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
