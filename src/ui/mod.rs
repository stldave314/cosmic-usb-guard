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

use crate::usbguard::{DeviceKey, Target};

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

/// The button style for a clickable list row.
///
/// A button sets the text colour for everything inside it. `Button::Transparent`
/// sounds like "no styling" but its foreground colour is literally
/// `rgba(0, 0, 0, 0)`, so every label inside one renders invisible unless it
/// overrides its own colour — which is not a thing the compiler or a headless
/// test would notice. `Button::ListItem` is the style actually intended for
/// this, and `tests::a_list_row_never_renders_its_text_invisible` pins it.
pub fn list_row_style() -> cosmic::theme::Button {
    cosmic::theme::Button::ListItem(cosmic::theme::active().cosmic().corner_radii.radius_s)
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
        /// Which device.
        key: DeviceKey,
        /// What to do with it.
        target: Target,
        /// Whether to write a standing rule as well.
        ///
        /// Always true for a device that is not plugged in: there is nothing
        /// to authorise now, so a rule is the only thing a decision can be.
        permanent: bool,
    },
    /// Remove the standing rule for a device and block it now.
    Revoke {
        /// Which device.
        key: DeviceKey,
    },
    /// Remove the standing rule for a device, without changing anything else.
    ///
    /// This is how a permanent decision is undone. Removing the rule returns
    /// the device to the implicit policy, which is to ask — so a mistaken
    /// "block, and remember that" becomes "ask me again next time" rather than
    /// forcing the opposite answer to be committed instead.
    Forget {
        /// Which device.
        key: DeviceKey,
    },
    /// Start editing a device's hook program.
    HookBegin {
        /// Descriptor hash of the device.
        hash: String,
    },
    /// Change one field of the hook being edited.
    HookEdit {
        /// Which field.
        field: HookField,
        /// Its new contents.
        value: String,
    },
    /// Turn the hook being edited on or off.
    HookEnabled(bool),
    /// Save the hook being edited.
    HookSave,
    /// Stop editing without saving.
    HookCancel,
    /// Delete a device's hook.
    HookRemove {
        /// Descriptor hash of the device.
        hash: String,
    },
    /// Mark a device as part of this machine, or unmark it.
    SetInternal {
        /// Descriptor hash of the device to mark.
        hash: String,
        /// Whether it is internal.
        internal: bool,
    },
    /// Show or hide a device's details. `None` collapses the current one.
    Select(Option<DeviceKey>),
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
    /// [`crate::config::Settings::show_internal`]
    ShowInternal(bool),
    /// [`crate::config::Settings::show_disconnected`]
    ShowDisconnected(bool),
    /// [`crate::config::Settings::warn_input_capable`]
    WarnInputCapable(bool),
    /// [`crate::config::Settings::journal_enabled`]
    JournalEnabled(bool),
    /// [`crate::config::Settings::warn_on_health_problems`]
    WarnOnHealthProblems(bool),
    /// [`crate::config::Settings::notify_on_auto_block`]
    NotifyOnAutoBlock(bool),
    /// [`crate::config::Settings::show_tray_icon`]
    ShowTrayIcon(bool),
    /// [`crate::config::Settings::autostart`]
    Autostart(bool),
    /// [`crate::config::Settings::start_minimized`]
    StartMinimized(bool),
    /// [`crate::config::Settings::run_in_background`]
    RunInBackground(bool),
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
            Self::ShowInternal(v) => settings.show_internal = v,
            Self::ShowDisconnected(v) => settings.show_disconnected = v,
            Self::WarnInputCapable(v) => settings.warn_input_capable = v,
            Self::JournalEnabled(v) => settings.journal_enabled = v,
            Self::WarnOnHealthProblems(v) => settings.warn_on_health_problems = v,
            Self::NotifyOnAutoBlock(v) => settings.notify_on_auto_block = v,
            Self::ShowTrayIcon(v) => settings.show_tray_icon = v,
            Self::Autostart(v) => settings.autostart = v,
            Self::StartMinimized(v) => settings.start_minimized = v,
            Self::RunInBackground(v) => settings.run_in_background = v,
        }
    }
}

/// A text field of the hook editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookField {
    /// What the user calls the hook.
    Label,
    /// Path to the program.
    Program,
    /// Arguments, one per line.
    Args,
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
    use cosmic::widget::button::Catalog;

    /// Resolve a button style's text colour the way the renderer would.
    fn text_colour(class: &cosmic::theme::Button) -> Option<cosmic::iced::Color> {
        cosmic::Theme::dark().active(false, false, class).text_color
    }

    #[test]
    fn a_list_row_never_renders_its_text_invisible() {
        // Control: prove this check can detect the failure. `Button::Transparent`
        // is the style that shipped first, and it makes every label inside the
        // row fully transparent. If this assertion ever stops holding, the one
        // below has stopped meaning anything.
        let transparent = text_colour(&cosmic::theme::Button::Transparent);
        assert_eq!(
            transparent.map(|c| c.a),
            Some(0.0),
            "Button::Transparent no longer produces invisible text, so this \
             test can no longer tell a visible row from an invisible one"
        );

        // The real assertion: the style the device list actually uses must
        // either inherit the surrounding text colour or set a visible one.
        match text_colour(&list_row_style()) {
            None => {}
            Some(colour) => assert!(
                colour.a > 0.0,
                "the device list row renders its text at alpha {}, which is invisible",
                colour.a
            ),
        }
    }

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
            (SettingChange::ShowInternal(true), |s| s.show_internal),
            (SettingChange::ShowDisconnected(false), |s| {
                s.show_disconnected
            }),
            (SettingChange::WarnInputCapable(false), |s| {
                s.warn_input_capable
            }),
            (SettingChange::JournalEnabled(false), |s| s.journal_enabled),
            (SettingChange::WarnOnHealthProblems(false), |s| {
                s.warn_on_health_problems
            }),
            (SettingChange::NotifyOnAutoBlock(false), |s| {
                s.notify_on_auto_block
            }),
            (SettingChange::ShowTrayIcon(false), |s| s.show_tray_icon),
            (SettingChange::Autostart(true), |s| s.autostart),
            (SettingChange::StartMinimized(true), |s| s.start_minimized),
            (SettingChange::RunInBackground(false), |s| {
                s.run_in_background
            }),
        ] {
            let before = read(&settings);
            change.apply(&mut settings);
            assert_ne!(read(&settings), before, "{change:?} did not take effect");
        }
    }
}
