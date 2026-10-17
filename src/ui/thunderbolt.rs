// SPDX-License-Identifier: GPL-3.0-or-later

//! The Thunderbolt page: what the controller lets through, and the devices
//! bolt knows about.

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, divider, icon, row, text};
use cosmic::{Apply, Element, theme};

use crate::fl;
use crate::state::State;
use crate::thunderbolt::{Device, Domain, SecurityLevel, Status};
use crate::usbguard::CheckGroup;

use super::Action;
use super::icons;

fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

/// What a security level means for the user, in a sentence.
fn security_label(level: &SecurityLevel) -> String {
    match level {
        SecurityLevel::None => fl!("tb-level-none"),
        SecurityLevel::User => fl!("tb-level-user"),
        SecurityLevel::Secure => fl!("tb-level-secure"),
        SecurityLevel::DpOnly => fl!("tb-level-dponly"),
        SecurityLevel::UsbOnly => fl!("tb-level-usbonly"),
        SecurityLevel::NoPcie => fl!("tb-level-nopcie"),
        SecurityLevel::Unknown(value) => value.clone(),
    }
}

/// A device's state, in words, with the color that goes with it.
fn status_badge<'a>(device: &Device) -> Element<'a, Action> {
    let (label, class) = match &device.status {
        Status::Connected => (fl!("tb-status-waiting"), theme::Text::Accent),
        Status::Authorized => (fl!("tb-status-authorized"), super::success_text()),
        Status::AuthError => (fl!("tb-status-error"), super::danger_text()),
        Status::Connecting | Status::Authorizing => {
            (fl!("tb-status-working"), theme::Text::Default)
        }
        Status::Disconnected => (fl!("state-disconnected"), theme::Text::Default),
        Status::Unknown(value) => (value.clone(), theme::Text::Default),
    };
    text::caption(label).class(class).into()
}

/// One labeled line of the summary card.
fn summary_line<'a>(label: String, value: String) -> Element<'a, Action> {
    let space = spacing();
    row::with_capacity(2)
        .push(text::body(label).width(Length::FillPortion(2)))
        .push(text::body(value).width(Length::FillPortion(3)))
        .spacing(space.space_s)
        .align_y(Alignment::Center)
        .into()
}

/// The controller's settings, and what they mean for this page.
fn summary<'a>(state: &State, domains: &[Domain]) -> Element<'a, Action> {
    let space = spacing();
    let mut body = column::with_capacity(6).spacing(space.space_xs);

    for domain in domains {
        let heading = if domains.len() > 1 {
            format!("{} — {}", fl!("tb-security"), domain.id)
        } else {
            fl!("tb-security")
        };
        body = body
            .push(summary_line(heading, security_label(&domain.security)))
            .push(summary_line(
                fl!("tb-dma"),
                if domain.iommu {
                    fl!("tb-dma-on")
                } else {
                    fl!("tb-dma-off")
                },
            ));
    }

    let manager = state.thunderbolt.as_ref().and_then(|s| s.manager.as_ref());
    if manager.is_some_and(|m| !m.authorizes_automatically()) {
        body = body.push(text::caption(fl!("tb-auto-off")));
    }

    if !domains.iter().any(|d| d.security.asks_per_device()) && !domains.is_empty() {
        body = body.push(text::caption(fl!("tb-never-asks")));
    }

    // Anything wrong with the setup is explained, with its fix, on the Status
    // page; this is the way there.
    let problems = state
        .health
        .problems()
        .iter()
        .any(|c| c.id.group() == CheckGroup::Thunderbolt);
    if problems {
        body = body.push(
            row::with_capacity(3)
                .push(icon::from_name(icons::WARNING).size(16))
                .push(text::caption(fl!("tb-has-problems")).width(Length::Fill))
                .push(button::standard(fl!("tb-open-status")).on_press(Action::ShowStatus))
                .align_y(Alignment::Center)
                .spacing(space.space_xs),
        );
    }

    container(body)
        .padding(space.space_s)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// The decision card for a device waiting to be allowed.
pub fn prompt<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let busy = state.thunderbolt_busy.contains(&device.uid);
    let uid = device.uid.clone();

    let body = column::with_capacity(3)
        .push(text::title4(fl!("tb-prompt-heading")))
        .push(text::body(fl!(
            "tb-prompt-description",
            name = device.display_name()
        )))
        .spacing(space.space_xxs);

    let mut buttons = row::with_capacity(4)
        .push(
            button::text(fl!("dismiss")).on_press(Action::ThunderboltDismiss { uid: uid.clone() }),
        )
        .push(cosmic::widget::space::horizontal())
        .push(
            button::standard(fl!("allow-once")).on_press_maybe(
                (!busy && device.can_authorize())
                    .then(|| Action::ThunderboltAuthorize { uid: uid.clone() }),
            ),
        )
        .spacing(space.space_xxs)
        .align_y(Alignment::Center);
    if device.can_enroll() {
        buttons = buttons.push(
            button::suggested(fl!("always-allow"))
                .on_press_maybe((!busy).then_some(Action::ThunderboltEnroll { uid })),
        );
    }

    column::with_capacity(2)
        .push(body)
        .push(buttons)
        .spacing(space.space_s)
        .apply(container)
        .padding(space.space_s)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// One device row, with whatever can be done to it.
fn device_row<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let busy = state.thunderbolt_busy.contains(&device.uid);

    let mut caption = Vec::new();
    if !device.vendor.trim().is_empty() && !device.display_name().starts_with(device.vendor.trim())
    {
        caption.push(device.vendor.trim().to_string());
    }
    caption.push(if device.stored {
        fl!("tb-remembered")
    } else {
        fl!("tb-not-remembered")
    });

    let summary = row::with_capacity(3)
        .push(icon::icon(icons::thunderbolt()).size(24))
        .push(
            column::with_capacity(2)
                .push(text::body(device.display_name()))
                .push(text::caption(caption.join(" · ")))
                .spacing(space.space_xxxs)
                .width(Length::Fill),
        )
        .push(status_badge(device))
        .align_y(Alignment::Center)
        .spacing(space.space_s)
        .apply(container)
        .padding(space.space_xs)
        .width(Length::Fill);

    let mut actions = row::with_capacity(3).spacing(space.space_xxs);
    let mut any = false;
    if device.can_authorize() {
        actions = actions.push(
            button::standard(fl!("allow-once")).on_press_maybe((!busy).then(|| {
                Action::ThunderboltAuthorize {
                    uid: device.uid.clone(),
                }
            })),
        );
        any = true;
    }
    if device.can_enroll() {
        actions = actions.push(button::suggested(fl!("always-allow")).on_press_maybe(
            (!busy).then(|| Action::ThunderboltEnroll {
                uid: device.uid.clone(),
            }),
        ));
        any = true;
    }
    if device.can_forget() {
        actions = actions.push(
            button::standard(fl!("tb-forget")).on_press_maybe((!busy).then(|| {
                Action::ThunderboltForget {
                    uid: device.uid.clone(),
                }
            })),
        );
        any = true;
    }

    let mut content = column::with_capacity(2).push(summary);
    if any {
        content = content.push(
            actions
                .apply(container)
                .padding([0, space.space_xs])
                .width(Length::Fill)
                .align_x(Alignment::End),
        );
    }
    content.spacing(space.space_xxs).into()
}

/// The full page.
pub fn view<'a>(state: &State) -> Element<'a, Action> {
    let space = spacing();

    let mut content = column::with_capacity(6).spacing(space.space_m).push(
        row::with_capacity(2)
            .push(text::title3(fl!("tb-heading")).width(Length::Fill))
            .push(button::standard(fl!("refresh")).on_press(Action::Refresh))
            .align_y(Alignment::Center)
            .spacing(space.space_s),
    );

    let Some(snapshot) = state.thunderbolt.as_ref() else {
        return content.push(text::body(fl!("status-checking"))).into();
    };

    if let Some(error) = snapshot.error.as_ref() {
        content = content.push(
            column::with_capacity(2)
                .push(text::body(error.clone()))
                .push(button::standard(fl!("tb-open-status")).on_press(Action::ShowStatus))
                .spacing(space.space_xs)
                .apply(container)
                .padding(space.space_s)
                .class(theme::Container::Card)
                .width(Length::Fill),
        );
        return content.into();
    }

    content = content.push(summary(state, &snapshot.domains));

    let waiting = state.thunderbolt_waiting();
    if !waiting.is_empty() {
        let mut prompts = column::with_capacity(waiting.len()).spacing(space.space_s);
        for device in waiting {
            prompts = prompts.push(prompt(state, device));
        }
        content = content.push(prompts);
    }

    let devices: Vec<&Device> = snapshot.peripherals().collect();
    let mut list = column::with_capacity(devices.len() * 2 + 2).spacing(space.space_xxs);
    list = list.push(text::heading(fl!("tb-devices")));
    if devices.is_empty() {
        list = list.push(text::caption(fl!("tb-no-devices")));
    }
    for (index, device) in devices.iter().enumerate() {
        if index > 0 {
            list = list.push(divider::horizontal::default());
        }
        list = list.push(device_row(state, device));
    }
    content = content.push(list);

    // Said once, on the page, rather than discovered after pressing Forget on
    // a dock that keeps working.
    content = content.push(text::caption(fl!("tb-forget-note")));

    content.into()
}
