// SPDX-License-Identifier: GPL-3.0-or-later

//! The decision history page.

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, divider, icon, row, text};
use cosmic::{Apply, Element, theme};

use crate::fl;
use crate::journal::{Entry, Kind};
use crate::state::State;

use super::format;
use super::icons;
use super::{Action, HistoryFilter};

fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

fn entry_icon(kind: Kind) -> &'static str {
    match kind {
        Kind::Allowed => icons::OK,
        Kind::Blocked | Kind::Rejected | Kind::Revoked => icons::BLOCKED,
        Kind::Inserted | Kind::Updated => icons::DEVICE,
        Kind::Removed => icons::REMOVE,
        Kind::ServiceUp => icons::OK,
        Kind::ServiceDown => icons::ERROR,
        Kind::HealthProblem => icons::WARNING,
    }
}

fn entry_class(kind: Kind) -> theme::Text {
    match kind {
        Kind::Allowed | Kind::ServiceUp => super::success_text(),
        Kind::Rejected | Kind::ServiceDown => super::danger_text(),
        Kind::HealthProblem | Kind::Revoked => super::warning_text(),
        _ => theme::Text::Default,
    }
}

/// One journal entry.
fn entry_row<'a>(entry: &Entry) -> Element<'a, Action> {
    let space = spacing();

    let headline = match entry.device.as_ref() {
        Some(device) => format!("{} — {}", format::kind_label(entry.kind), device.name),
        None => format::kind_label(entry.kind),
    };

    let mut meta = vec![format::entry_time(entry), format::actor_label(entry.actor)];
    if entry.permanent {
        meta.push(fl!("remember-decision"));
    }
    if let Some(device) = entry.device.as_ref()
        && !device.usb_id.is_empty()
    {
        meta.push(device.usb_id.clone());
    }
    if !entry.detail.is_empty() {
        meta.push(entry.detail.clone());
    }

    row::with_capacity(2)
        .push(icon::from_name(entry_icon(entry.kind)).size(18))
        .push(
            column::with_capacity(2)
                .push(text::body(headline).class(entry_class(entry.kind)))
                .push(text::caption(meta.join(" · ")))
                .spacing(space.space_xxxs)
                .width(Length::Fill),
        )
        .align_y(Alignment::Start)
        .spacing(space.space_xs)
        .apply(container)
        .padding(space.space_xs)
        .width(Length::Fill)
        .into()
}

/// The full history page.
pub fn view<'a>(state: &State) -> Element<'a, Action> {
    let space = spacing();
    let entries = state.filtered_history();

    let filter = filter_control(state.history_filter);

    let header = row::with_capacity(3)
        .push(text::title3(fl!("history-heading")).width(Length::Fill))
        .push(text::caption(fl!("history-entries", count = entries.len())))
        .push(
            button::destructive(fl!("history-clear"))
                .on_press_maybe((!state.history.is_empty()).then_some(Action::ClearHistory)),
        )
        .align_y(Alignment::Center)
        .spacing(space.space_s);

    let mut content = column::with_capacity(entries.len() + 3)
        .push(header)
        .push(filter)
        .push(divider::horizontal::default())
        .spacing(space.space_s);

    if entries.is_empty() {
        content = content.push(
            column::with_capacity(2)
                .push(text::body(fl!("history-empty")))
                .push(text::caption(fl!("history-empty-description")))
                .spacing(space.space_xxs)
                .apply(container)
                .padding(space.space_m)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        );
    } else {
        let mut list = column::with_capacity(entries.len());
        for entry in entries {
            list = list.push(entry_row(entry));
        }
        content = content.push(list);
    }

    content.into()
}

/// The two-option filter control.
///
/// Plain buttons rather than a `segmented_button`, whose model would have to
/// outlive the view; keeping it in application state just to render two
/// choices is not worth the coupling.
fn filter_control<'a>(filter: HistoryFilter) -> Element<'a, Action> {
    let space = spacing();

    let option = |label: String, value: HistoryFilter| {
        if filter == value {
            button::suggested(label)
        } else {
            button::standard(label)
        }
        .on_press(Action::SetHistoryFilter(value))
    };

    row::with_capacity(2)
        .push(option(fl!("history-filter-all"), HistoryFilter::All))
        .push(option(
            fl!("history-filter-decisions"),
            HistoryFilter::Decisions,
        ))
        .spacing(space.space_xxs)
        .into()
}
