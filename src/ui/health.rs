// SPDX-License-Identifier: GPL-3.0-or-later

//! The status page: whether USBGuard is actually protecting this machine.

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, divider, icon, row, text};
use cosmic::{Apply, Element, theme};

use crate::fl;
use crate::state::State;
use crate::usbguard::{Check, Severity};

use super::Action;
use super::format;
use super::icons;

fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

fn severity_class(severity: Severity) -> theme::Text {
    match severity {
        Severity::Ok => super::success_text(),
        Severity::Warning => super::warning_text(),
        Severity::Critical => super::danger_text(),
    }
}

/// A one-line banner summarising protection status.
///
/// Shown at the top of both the applet popup and the main window, because a
/// device list is meaningless if the daemon behind it is not running.
pub fn banner<'a>(state: &State) -> Option<Element<'a, Action>> {
    let space = spacing();

    // Not connected is the loudest possible statement and outranks any stale
    // health result.
    if !state.connected {
        return Some(alert(
            Severity::Critical,
            fl!("status-disconnected"),
            if state.disconnect_reason.is_empty() {
                None
            } else {
                Some(fl!(
                    "status-disconnected-description",
                    reason = state.disconnect_reason.clone()
                ))
            },
            None,
        ));
    }

    if !state.health_checked {
        return Some(
            row::with_capacity(2)
                .push(icon::from_name(icons::REFRESH).size(16))
                .push(text::caption(fl!("status-checking")))
                .align_y(Alignment::Center)
                .spacing(space.space_xxs)
                .apply(container)
                .padding(space.space_xs)
                .into(),
        );
    }

    if !state.config.settings.warn_on_health_problems || state.health.is_healthy() {
        return None;
    }

    let worst = state.health.worst();
    let problem = state.health.problems().first().map(|c| (*c).clone());
    Some(alert(
        worst,
        format::status_headline(worst),
        problem.as_ref().map(|check| format::check_label(check.id)),
        problem.as_ref().and_then(|check| check.remedy.clone()),
    ))
}

fn alert<'a>(
    severity: Severity,
    heading: String,
    detail: Option<String>,
    remedy: Option<String>,
) -> Element<'a, Action> {
    let space = spacing();

    let mut body = column::with_capacity(3)
        .push(text::body(heading).class(severity_class(severity)))
        .spacing(space.space_xxxs);

    if let Some(detail) = detail {
        body = body.push(text::caption(detail));
    }

    let mut content = row::with_capacity(3)
        .push(icon::from_name(icons::for_severity(severity)).size(20))
        .push(body.width(Length::Fill))
        .align_y(Alignment::Center)
        .spacing(space.space_xs);

    if let Some(remedy) = remedy {
        content = content.push(
            button::standard(fl!("copy"))
                .on_press(Action::CopyRemedy(remedy))
                .padding(space.space_xxs),
        );
    }

    container(content)
        .padding(space.space_xs)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// One check, with its observed value and suggested fix.
fn check_row<'a>(check: &Check) -> Element<'a, Action> {
    let space = spacing();

    let mut body = column::with_capacity(3)
        .push(text::body(format::check_label(check.id)))
        .spacing(space.space_xxxs);

    if !check.detail.is_empty() {
        body = body.push(text::caption(fl!(
            "check-observed",
            value = check.detail.clone()
        )));
    }

    if let Some(remedy) = check.remedy.as_ref() {
        body = body.push(text::caption(fl!("remedy-heading")));
        body = body.push(
            row::with_capacity(2)
                .push(text::monotext(remedy.clone()).width(Length::Fill))
                .push(
                    button::icon(icon::from_name(icons::COPY).size(14))
                        .on_press(Action::CopyRemedy(remedy.clone())),
                )
                .align_y(Alignment::Center)
                .spacing(space.space_xxs),
        );
    }

    row::with_capacity(2)
        .push(icon::from_name(icons::for_severity(check.severity)).size(18))
        .push(body.width(Length::Fill))
        .spacing(space.space_xs)
        .apply(container)
        .padding(space.space_xs)
        .width(Length::Fill)
        .into()
}

/// The full status page.
pub fn view<'a>(state: &State) -> Element<'a, Action> {
    let space = spacing();

    let headline = if !state.connected {
        fl!("status-disconnected")
    } else if !state.health_checked {
        fl!("status-checking")
    } else {
        format::status_headline(state.health.worst())
    };

    let severity = if state.is_protected() {
        Severity::Ok
    } else if state.connected && state.health_checked {
        state.health.worst()
    } else {
        Severity::Critical
    };

    let mut content = column::with_capacity(state.health.checks.len() + 3)
        .push(
            // `title4`, not `title3`. Unlike the other pages' headings this
            // one is a whole sentence, and at `title3` next to a fixed-width
            // Refresh button it wrapped to three oversized lines in the
            // remaining column.
            row::with_capacity(3)
                .push(icon::from_name(icons::for_severity(severity)).size(32))
                .push(text::title4(headline).width(Length::Fill))
                .push(button::standard(fl!("refresh")).on_press(Action::Refresh))
                .align_y(Alignment::Center)
                .spacing(space.space_s),
        )
        .push(divider::horizontal::default())
        .spacing(space.space_s);

    if state.health.checks.is_empty() {
        content = content.push(text::body(fl!("status-checking")));
    } else {
        let mut checks = column::with_capacity(state.health.checks.len());
        for check in &state.health.checks {
            checks = checks.push(check_row(check));
        }
        content = content.push(checks);
    }

    content.into()
}
