// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared user interface.
//!
//! The applet and the window application render the same information at
//! different sizes, so the views live here and emit a common [`Action`]. Each
//! binary maps that into its own message type with `Element::map`, which keeps
//! the widgets free of any knowledge of either application.

pub mod device;
pub mod format;
pub mod health;
pub mod history;
pub mod icons;
pub mod settings;

use crate::usbguard::Target;

/// Text colour for something that succeeded or is permitted.
///
/// `cosmic::theme::Text` only carries `Accent`, `Default` and an explicit
/// colour, so the semantic palette entries are resolved from the active theme
/// here. Views are rebuilt when the theme changes, so this stays correct.
pub fn success_text() -> cosmic::theme::Text {
    cosmic::theme::Text::Color(cosmic::theme::active().cosmic().success_text_color().into())
}

/// Text colour for a warning.
pub fn warning_text() -> cosmic::theme::Text {
    cosmic::theme::Text::Color(cosmic::theme::active().cosmic().warning_text_color().into())
}

/// Text colour for something blocked, failed, or destructive.
pub fn danger_text() -> cosmic::theme::Text {
    cosmic::theme::Text::Color(
        cosmic::theme::active()
            .cosmic()
            .destructive_text_color()
            .into(),
    )
}

/// Something the user asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Apply an authorisation decision to a device.
    Decide {
        /// Daemon-assigned device ID.
        device_id: u32,
        /// What to do with it.
        target: Target,
        /// Whether to write a standing rule as well.
        permanent: bool,
    },
    /// Remove the standing rule for a device and block it now.
    Revoke {
        /// Daemon-assigned device ID.
        device_id: u32,
    },
    /// Show or hide a device's details. `None` collapses the current one.
    Select(Option<u32>),
    /// Change whether the next decision is written as a standing rule.
    SetPermanent(bool),
    /// Stop asking about a device without deciding anything.
    ///
    /// The device stays exactly as USBGuard left it — blocked — so this
    /// dismisses the prompt, not the protection.
    DismissPrompt {
        /// Daemon-assigned device ID.
        device_id: u32,
    },
    /// Copy text to the clipboard.
    Copy(String),
    /// Re-read devices and health from the daemon.
    Refresh,
    /// Change a setting.
    Setting(SettingChange),
    /// Change which journal entries are listed.
    SetHistoryFilter(HistoryFilter),
    /// Delete the decision history.
    ClearHistory,
    /// Copy a suggested fix-it command to the clipboard.
    CopyRemedy(String),
    /// Launch the main window from the applet.
    OpenApp,
    /// Quit.
    Quit,
}

/// A single settings toggle.
///
/// One variant per field rather than a closure so the action stays `Clone`,
/// `Debug` and comparable, which iced messages need to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingChange {
    /// [`crate::config::Settings::prompt_on_insert`]
    PromptOnInsert(bool),
    /// [`crate::config::Settings::notify_on_insert`]
    NotifyOnInsert(bool),
    /// [`crate::config::Settings::auto_open_popup`]
    AutoOpenPopup(bool),
    /// [`crate::config::Settings::default_permanent`]
    DefaultPermanent(bool),
    /// [`crate::config::Settings::show_hardwired`]
    ShowHardwired(bool),
    /// [`crate::config::Settings::show_root_hubs`]
    ShowRootHubs(bool),
    /// [`crate::config::Settings::warn_input_capable`]
    WarnInputCapable(bool),
    /// [`crate::config::Settings::journal_enabled`]
    JournalEnabled(bool),
    /// [`crate::config::Settings::warn_on_health_problems`]
    WarnOnHealthProblems(bool),
}

impl SettingChange {
    /// Apply the change to a settings struct.
    pub fn apply(self, settings: &mut crate::config::Settings) {
        match self {
            Self::PromptOnInsert(v) => settings.prompt_on_insert = v,
            Self::NotifyOnInsert(v) => settings.notify_on_insert = v,
            Self::AutoOpenPopup(v) => settings.auto_open_popup = v,
            Self::DefaultPermanent(v) => settings.default_permanent = v,
            Self::ShowHardwired(v) => settings.show_hardwired = v,
            Self::ShowRootHubs(v) => settings.show_root_hubs = v,
            Self::WarnInputCapable(v) => settings.warn_input_capable = v,
            Self::JournalEnabled(v) => settings.journal_enabled = v,
            Self::WarnOnHealthProblems(v) => settings.warn_on_health_problems = v,
        }
    }
}

/// Which journal entries the history view lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HistoryFilter {
    /// Everything, including devices coming and going.
    #[default]
    All,
    /// Only entries that record an authorisation decision.
    Decisions,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;

    #[test]
    fn every_setting_change_actually_changes_its_field() {
        // A copy-paste slip here would silently make one toggle a no-op.
        let mut settings = Settings::default();

        for (change, read) in [
            (
                SettingChange::PromptOnInsert(false),
                (|s: &Settings| s.prompt_on_insert) as fn(&Settings) -> bool,
            ),
            (SettingChange::NotifyOnInsert(false), |s| s.notify_on_insert),
            (SettingChange::AutoOpenPopup(false), |s| s.auto_open_popup),
            (SettingChange::DefaultPermanent(true), |s| {
                s.default_permanent
            }),
            (SettingChange::ShowHardwired(true), |s| s.show_hardwired),
            (SettingChange::ShowRootHubs(true), |s| s.show_root_hubs),
            (SettingChange::WarnInputCapable(false), |s| {
                s.warn_input_capable
            }),
            (SettingChange::JournalEnabled(false), |s| s.journal_enabled),
            (SettingChange::WarnOnHealthProblems(false), |s| {
                s.warn_on_health_problems
            }),
        ] {
            let before = read(&settings);
            change.apply(&mut settings);
            assert_ne!(read(&settings), before, "{change:?} did not take effect");
        }
    }
}
