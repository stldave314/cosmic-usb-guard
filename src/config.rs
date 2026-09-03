// SPDX-License-Identifier: GPL-3.0-or-later

//! User-facing settings, persisted through `cosmic-config`.
//!
//! Only settings a user would reasonably want to change belong here.
//! Implementation tuning values live in [`crate::constants`] as compile-time
//! constants, so there is no second config mechanism to load, parse or fail.

use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};
use serde::{Deserialize, Serialize};

use crate::hooks::Hook;

use crate::constants::APP_ID;
use crate::debug_log;

/// Schema version. Bump when a field is removed or changes meaning.
pub const CONFIG_VERSION: u64 = 1;

/// Persisted user settings.
///
/// Both the applet and the main window read the same config, so a change made
/// in one is picked up by the other through `cosmic-config`'s watcher.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CosmicConfigEntry)]
#[version = 1]
pub struct Settings {
    /// Raise a decision prompt when a device that has no standing rule is
    /// plugged in.
    pub prompt_on_insert: bool,

    /// Also post a desktop notification for that prompt.
    ///
    /// Useful when the panel is hidden or the user is in a full-screen app.
    pub notify_on_insert: bool,

    /// Let the applet open its popup by itself when a device needs a decision.
    pub auto_open_popup: bool,

    /// Whether "remember this decision" starts ticked in the prompt.
    ///
    /// Off by default: writing a permanent rule is the more consequential of
    /// the two options, so it should be a deliberate act.
    pub default_permanent: bool,

    /// Show devices that are soldered in or otherwise not user-pluggable.
    pub show_hardwired: bool,

    /// Show root hubs, which are part of the machine rather than plugged into it.
    pub show_root_hubs: bool,

    /// Show devices the user has marked as internal.
    pub show_internal: bool,

    /// Show devices that have a standing rule but are not plugged in.
    ///
    /// On by default: a permanent decision outlives the device being
    /// connected, so hiding unplugged devices hides the only way to take one
    /// back.
    pub show_disconnected: bool,

    /// Descriptor hashes of devices the user has marked as part of this
    /// machine — an internal card reader, a fingerprint sensor, a soldered-on
    /// Bluetooth radio.
    ///
    /// USBGuard's own `with-connect-type` cannot be relied on for this: a
    /// Goodix fingerprint sensor on an internal header reports `"not used"`,
    /// not `"hardwired"`, so no heuristic short of a hardware database gets it
    /// right. The user knows; this is where they say so.
    ///
    /// Hashes, not USB IDs, so a mark cannot be inherited by a device that
    /// merely claims the same vendor and product.
    pub internal_hashes: Vec<String>,

    /// Call out devices that present a keyboard or pointer interface.
    ///
    /// A device that can type can do anything the user can, which is the basis
    /// of the "BadUSB" class of attack, so it is worth flagging even when the
    /// device also does something innocuous.
    pub warn_input_capable: bool,

    /// Record events to the decision journal.
    pub journal_enabled: bool,

    /// Show the status icon in the system tray.
    ///
    /// On by default, because the icon is the only thing that tells the user
    /// the app is running and watching. Turning it off does not stop it
    /// watching — see [`Settings::start_minimized`], which is what makes a
    /// hidden icon plus a hidden window reachable again only from the
    /// launcher.
    pub show_tray_icon: bool,

    /// Start the app when the session starts.
    ///
    /// Mirrors a freedesktop autostart entry rather than being the source of
    /// truth; [`crate::autostart::is_enabled`] reads the file itself, so an
    /// entry deleted outside the app is noticed.
    pub autostart: bool,

    /// Start with the window hidden, showing only the status icon.
    pub start_minimized: bool,

    /// Keep running when the window is closed.
    ///
    /// On by default. Closing the window otherwise stops the app watching for
    /// devices, which would silently turn off the prompting this exists for.
    pub run_in_background: bool,

    /// Notify when USBGuard blocks a device without asking.
    ///
    /// This is the case a user cannot otherwise see: a device with a standing
    /// block rule is refused silently, and nothing on screen explains why the
    /// drive did not appear.
    pub notify_on_auto_block: bool,

    /// Per-device hook programs, keyed by descriptor hash.
    ///
    /// See [`crate::hooks`] for why a hook only ever runs for a device that is
    /// both configured here and currently authorised.
    pub hooks: Vec<Hook>,

    /// Warn when USBGuard's own configuration would defeat prompting.
    pub warn_on_health_problems: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            prompt_on_insert: true,
            notify_on_insert: true,
            auto_open_popup: true,
            default_permanent: false,
            show_hardwired: false,
            show_root_hubs: false,
            show_internal: false,
            show_disconnected: true,
            internal_hashes: Vec::new(),
            warn_input_capable: true,
            journal_enabled: true,
            warn_on_health_problems: true,
            show_tray_icon: true,
            autostart: false,
            start_minimized: false,
            run_in_background: true,
            notify_on_auto_block: true,
            hooks: Vec::new(),
        }
    }
}

/// A config handle plus the settings last read from it.
#[derive(Debug, Clone, Default)]
pub struct ConfigState {
    /// Handle used to persist changes, or `None` when `cosmic-config` is
    /// unavailable — in which case settings are in-memory only for this run.
    pub handle: Option<Config>,
    /// Current settings.
    pub settings: Settings,
}

impl Settings {
    /// The hook configured for a device hash, if any.
    pub fn hook(&self, hash: &str) -> Option<&Hook> {
        if hash.is_empty() {
            return None;
        }
        self.hooks.iter().find(|hook| hook.hash == hash)
    }
}

impl ConfigState {
    /// Load settings, falling back to defaults on any failure.
    ///
    /// A missing or corrupt config must not stop the app from starting: the
    /// defaults are safe, and refusing to run would leave the user with no way
    /// to see or change USB policy at all.
    pub fn load() -> Self {
        let Ok(handle) = Config::new(APP_ID, CONFIG_VERSION) else {
            crate::error_log!(
                crate::debug::CONFIG,
                "cosmic-config unavailable; using defaults for this session"
            );
            return Self::default();
        };

        let settings = match Settings::get_entry(&handle) {
            Ok(settings) => settings,
            Err((errors, fallback)) => {
                for error in errors {
                    debug_log!(crate::debug::CONFIG, "config key error: {error}");
                }
                fallback
            }
        };

        debug_log!(crate::debug::CONFIG, "loaded settings: {settings:?}");
        Self {
            handle: Some(handle),
            settings,
        }
    }

    /// Apply a change and persist it.
    ///
    /// The in-memory value is updated regardless of whether the write
    /// succeeds, so the UI stays consistent with what the user just did.
    pub fn update(&mut self, change: impl FnOnce(&mut Settings)) {
        change(&mut self.settings);
        let Some(handle) = self.handle.as_ref() else {
            return;
        };
        if let Err(e) = self.settings.write_entry(handle) {
            crate::error_log!(crate::debug::CONFIG, "could not save settings: {e}");
        } else {
            debug_log!(crate::debug::CONFIG, "settings saved");
        }
    }

    /// Merge in keys that changed underneath us, e.g. edited by the other
    /// binary or by `cosmic-settings`.
    pub fn reload_keys<T: AsRef<str>>(&mut self, config: &Config, keys: &[T]) {
        let (errors, updated) = self.settings.update_keys(config, keys);
        for error in errors {
            debug_log!(crate::debug::CONFIG, "config reload error: {error}");
        }
        if !updated.is_empty() {
            debug_log!(crate::debug::CONFIG, "reloaded keys: {updated:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_prompt_and_log() {
        // The whole point of the app is to ask before authorising, and to keep
        // a record. Both must be on out of the box.
        let settings = Settings::default();
        assert!(settings.prompt_on_insert);
        assert!(settings.journal_enabled);
        assert!(settings.warn_on_health_problems);
    }

    #[test]
    fn permanent_rules_are_not_the_default() {
        // Writing a standing rule is the more consequential choice; it should
        // never happen because the user clicked through a pre-ticked box.
        assert!(!Settings::default().default_permanent);
    }

    #[test]
    fn a_permanent_decision_is_reversible_out_of_the_box() {
        // The device does not have to be plugged in for its standing rule to
        // exist, so hiding unplugged devices by default would hide the only
        // control that can take a mistaken "block" back.
        assert!(Settings::default().show_disconnected);
    }

    #[test]
    fn nothing_is_internal_until_the_user_says_so() {
        // Guessing would be worse than not guessing: silently treating a
        // device as internal stops the app asking about it.
        assert!(Settings::default().internal_hashes.is_empty());
    }

    #[test]
    fn the_tray_icon_is_on_and_nothing_autostarts_by_default() {
        // The icon is the only sign the app is running; hiding it by default
        // would make a watching process invisible. Autostart is the user's
        // call, so installing the app must not quietly add a session entry.
        let settings = Settings::default();
        assert!(settings.show_tray_icon);
        assert!(!settings.autostart);
        assert!(!settings.start_minimized);
    }

    #[test]
    fn closing_the_window_does_not_stop_watching_by_default() {
        // Otherwise the close button silently turns off USB prompting.
        assert!(Settings::default().run_in_background);
    }

    #[test]
    fn no_hooks_exist_until_the_user_adds_one() {
        // A hook runs a program. There must be no way to end up with one by
        // default, by upgrade, or by a config that failed to parse.
        assert!(Settings::default().hooks.is_empty());
    }

    #[test]
    fn a_hook_is_only_found_by_a_real_hash() {
        let settings = Settings {
            hooks: vec![Hook::new("H1=".to_string())],
            ..Settings::default()
        };
        assert!(settings.hook("H1=").is_some());
        assert!(settings.hook("H2=").is_none());
        // An empty hash must never match: a device that reports no descriptor
        // hash would otherwise pick up the first hook in the list.
        assert!(settings.hook("").is_none());
    }

    #[test]
    fn settings_round_trip_through_ron() {
        let settings = Settings {
            prompt_on_insert: false,
            show_root_hubs: true,
            internal_hashes: vec!["GW6m2e7TcpHpbCWTcSHkQNRWKpjY0S92r2ktsuLQ9Xc=".to_string()],
            hooks: vec![Hook {
                hash: "GW6m2e7TcpHpbCWTcSHkQNRWKpjY0S92r2ktsuLQ9Xc=".to_string(),
                program: std::path::PathBuf::from("/usr/local/bin/backup.sh"),
                args: vec!["--full".to_string()],
                enabled: true,
                label: "Backup".to_string(),
            }],
            ..Settings::default()
        };
        let encoded = ron::ser::to_string(&settings).unwrap();
        let decoded: Settings = ron::from_str(&encoded).unwrap();
        assert_eq!(decoded, settings);
    }
}
