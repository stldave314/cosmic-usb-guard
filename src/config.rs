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

    /// Call out devices that present a keyboard or pointer interface.
    ///
    /// A device that can type can do anything the user can, which is the basis
    /// of the "BadUSB" class of attack, so it is worth flagging even when the
    /// device also does something innocuous.
    pub warn_input_capable: bool,

    /// Record events to the decision journal.
    pub journal_enabled: bool,

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
            warn_input_capable: true,
            journal_enabled: true,
            warn_on_health_problems: true,
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
    fn settings_round_trip_through_ron() {
        let settings = Settings {
            prompt_on_insert: false,
            show_root_hubs: true,
            ..Settings::default()
        };
        let encoded = ron::ser::to_string(&settings).unwrap();
        let decoded: Settings = ron::from_str(&encoded).unwrap();
        assert_eq!(decoded, settings);
    }
}
