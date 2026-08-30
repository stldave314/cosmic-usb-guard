// SPDX-License-Identifier: GPL-3.0-or-later

//! The settings page.

use cosmic::widget::{column, settings, text};
use cosmic::{Element, theme};

use crate::fl;
use crate::journal;
use crate::state::State;

use super::{Action, SettingChange};

fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

/// A settings row with a title, description and a toggle.
fn toggle<'a>(
    title: String,
    description: String,
    value: bool,
    change: fn(bool) -> SettingChange,
) -> cosmic::widget::list::ListButton<'a, Action> {
    settings::item::builder(title)
        .description(description)
        .toggler(value, move |v| Action::Setting(change(v)))
}

/// The full settings page.
pub fn view<'a>(state: &State) -> Element<'a, Action> {
    let space = spacing();
    let settings = &state.config.settings;

    let behaviour = settings::section()
        .title(fl!("section-behaviour"))
        .add(toggle(
            fl!("setting-prompt-on-insert"),
            fl!("setting-prompt-on-insert-description"),
            settings.prompt_on_insert,
            SettingChange::PromptOnInsert,
        ))
        .add(toggle(
            fl!("setting-notify-on-insert"),
            fl!("setting-notify-on-insert-description"),
            settings.notify_on_insert,
            SettingChange::NotifyOnInsert,
        ))
        .add(toggle(
            fl!("setting-auto-open-popup"),
            fl!("setting-auto-open-popup-description"),
            settings.auto_open_popup,
            SettingChange::AutoOpenPopup,
        ))
        .add(toggle(
            fl!("setting-default-permanent"),
            fl!("setting-default-permanent-description"),
            settings.default_permanent,
            SettingChange::DefaultPermanent,
        ));

    let display = settings::section()
        .title(fl!("section-display"))
        .add(toggle(
            fl!("setting-show-hardwired"),
            fl!("setting-show-hardwired-description"),
            settings.show_hardwired,
            SettingChange::ShowHardwired,
        ))
        .add(toggle(
            fl!("setting-show-root-hubs"),
            fl!("setting-show-root-hubs-description"),
            settings.show_root_hubs,
            SettingChange::ShowRootHubs,
        ))
        .add(toggle(
            fl!("setting-warn-input-capable"),
            fl!("setting-warn-input-capable-description"),
            settings.warn_input_capable,
            SettingChange::WarnInputCapable,
        ))
        .add(toggle(
            fl!("setting-warn-on-health-problems"),
            fl!("setting-warn-on-health-problems-description"),
            settings.warn_on_health_problems,
            SettingChange::WarnOnHealthProblems,
        ));

    let history = settings::section()
        .title(fl!("section-privacy"))
        .add(toggle(
            fl!("setting-journal-enabled"),
            fl!(
                "setting-journal-enabled-description",
                path = journal::path().display().to_string()
            ),
            settings.journal_enabled,
            SettingChange::JournalEnabled,
        ));

    column::with_capacity(4)
        .push(text::title3(fl!("page-settings")))
        .push(behaviour)
        .push(display)
        .push(history)
        .spacing(space.space_m)
        .into()
}
