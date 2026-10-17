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
pub mod thunderbolt;

use crate::usbguard::{DeviceKey, Target};

/// Text color for something that succeeded or is permitted.
///
/// `cosmic::theme::Text` only carries `Accent`, `Default` and an explicit
/// color, so the semantic palette entries are resolved from the active theme
/// here. Views are rebuilt when the theme changes, so this stays correct.
pub fn success_text() -> cosmic::theme::Text {
    cosmic::theme::Text::Color(cosmic::theme::active().cosmic().success_text_color().into())
}

/// Text color for a warning.
pub fn warning_text() -> cosmic::theme::Text {
    cosmic::theme::Text::Color(cosmic::theme::active().cosmic().warning_text_color().into())
}

/// The button style for a clickable list row.
///
/// A button sets the text color for everything inside it. `Button::Transparent`
/// sounds like "no styling" but its foreground color is literally
/// `rgba(0, 0, 0, 0)`, so every label inside one renders invisible unless it
/// overrides its own color — which is not a thing the compiler or a headless
/// test would notice. `Button::ListItem` is the style actually intended for
/// this, and `tests::a_list_row_never_renders_its_text_invisible` pins it.
pub fn list_row_style() -> cosmic::theme::Button {
    cosmic::theme::Button::ListItem(cosmic::theme::active().cosmic().corner_radii.radius_s)
}

/// A dismissible message saying that something the user asked for failed.
///
/// A card with the theme's own foreground, not `cosmic::widget::warning`. That
/// widget paints `warning.on` text over the `warning` color, and a theme is
/// free to make those two nearly the same — under one such theme the message
/// rendered as white on pale gray, so the only explanation of a failure was
/// the one thing on the page that could not be read. A card's text color is
/// defined as the one that contrasts with its background, and
/// `tests::an_error_banner_is_readable` pins that.
///
/// Generic over the message type because dismissing is the application's
/// business rather than an [`Action`].
pub fn error_banner<'a, Message: Clone + 'static>(
    message: String,
    on_close: Message,
) -> cosmic::Element<'a, Message> {
    use cosmic::iced::{Alignment, Length};
    use cosmic::widget::{button, container, icon, row, text};

    let space = cosmic::theme::active().cosmic().spacing;

    let content = row::with_capacity(3)
        .push(
            icon::from_name(icons::ERROR)
                .size(20)
                .icon()
                .class(cosmic::theme::Svg::custom(|theme| {
                    cosmic::widget::svg::Style {
                        color: Some(theme.cosmic().destructive_text_color().into()),
                    }
                })),
        )
        .push(text::body(message).width(Length::Fill))
        .push(button::icon(icon::from_name(icons::CLOSE).size(16)).on_press(on_close))
        .align_y(Alignment::Center)
        .spacing(space.space_xs);

    container(content)
        .padding(space.space_xs)
        .class(error_banner_style())
        .width(Length::Fill)
        .into()
}

/// The container style behind [`error_banner`].
fn error_banner_style() -> cosmic::theme::Container<'static> {
    cosmic::theme::Container::Card
}

/// Text color for something blocked, failed, or destructive.
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
    /// Apply an authorization decision to a device.
    Decide {
        /// Which device.
        key: DeviceKey,
        /// What to do with it.
        target: Target,
        /// Whether to write a standing rule as well.
        ///
        /// Always true for a device that is not plugged in: there is nothing
        /// to authorize now, so a rule is the only thing a decision can be.
        permanent: bool,
        /// Whether a permanent rule is bound to the device's current port.
        bind: bool,
    },
    /// Bind a device's standing rule to its current port, or release it.
    SetBound {
        /// Which device.
        key: DeviceKey,
        /// Bound or not.
        bound: bool,
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
    /// Turn "bind to this port" on or off in one device's prompt.
    SetBind {
        /// Daemon-assigned ID of the device the prompt is for.
        device_id: u32,
        /// On or off.
        bind: bool,
    },
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
    /// Allow a Thunderbolt device for this connection only.
    ThunderboltAuthorize {
        /// bolt's UID for the device.
        uid: String,
    },
    /// Allow a Thunderbolt device and have bolt remember it.
    ThunderboltEnroll {
        /// bolt's UID for the device.
        uid: String,
    },
    /// Have bolt forget a Thunderbolt device.
    ThunderboltForget {
        /// bolt's UID for the device.
        uid: String,
    },
    /// Stop asking about a Thunderbolt device without allowing it.
    ThunderboltDismiss {
        /// bolt's UID for the device.
        uid: String,
    },
    /// Go to the Status page, where problems are explained with their fixes.
    ShowStatus,
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
    /// Only entries that record an authorization decision.
    Decisions,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use cosmic::widget::button::Catalog;

    /// Resolve a button style's text color the way the renderer would.
    fn text_color(class: &cosmic::theme::Button) -> Option<cosmic::iced::Color> {
        cosmic::Theme::dark().active(false, false, class).text_color
    }

    /// WCAG relative luminance of an opaque color.
    fn luminance(color: cosmic::iced::Color) -> f32 {
        let channel = |c: f32| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(color.r) + 0.7152 * channel(color.g) + 0.0722 * channel(color.b)
    }

    /// WCAG contrast ratio between two colors, from 1 (identical) to 21.
    fn contrast(a: cosmic::iced::Color, b: cosmic::iced::Color) -> f32 {
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn an_error_banner_is_readable() {
        use cosmic::iced::Background;
        use cosmic::widget::container::Catalog;

        // Control: the arithmetic can tell unreadable from readable.
        let white = cosmic::iced::Color::WHITE;
        let pale = cosmic::iced::Color::from_rgb(0.85, 0.85, 0.85);
        assert!(contrast(white, pale) < 2.0);
        assert!(contrast(white, cosmic::iced::Color::BLACK) > 20.0);

        for (name, theme) in [
            ("dark", cosmic::Theme::dark()),
            ("light", cosmic::Theme::light()),
            ("dark high contrast", cosmic::Theme::dark_hc()),
            ("light high contrast", cosmic::Theme::light_hc()),
        ] {
            let style = theme.style(&error_banner_style());
            let text = style
                .text_color
                .unwrap_or_else(|| theme.cosmic().on_bg_color().into());
            let Some(Background::Color(background)) = style.background else {
                panic!("the error banner has no solid background under the {name} theme");
            };
            // 4.5 is the WCAG AA threshold for body text.
            let ratio = contrast(text, background);
            assert!(
                ratio >= 4.5,
                "error banner text has a contrast ratio of {ratio:.2} under the {name} theme"
            );
        }
    }

    #[test]
    fn a_list_row_never_renders_its_text_invisible() {
        // Control: prove this check can detect the failure. `Button::Transparent`
        // is the style that shipped first, and it makes every label inside the
        // row fully transparent. If this assertion ever stops holding, the one
        // below has stopped meaning anything.
        let transparent = text_color(&cosmic::theme::Button::Transparent);
        assert_eq!(
            transparent.map(|c| c.a),
            Some(0.0),
            "Button::Transparent no longer produces invisible text, so this \
             test can no longer tell a visible row from an invisible one"
        );

        // The real assertion: the style the device list actually uses must
        // either inherit the surrounding text color or set a visible one.
        match text_color(&list_row_style()) {
            None => {}
            Some(color) => assert!(
                color.a > 0.0,
                "the device list row renders its text at alpha {}, which is invisible",
                color.a
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
