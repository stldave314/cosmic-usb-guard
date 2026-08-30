// SPDX-License-Identifier: GPL-3.0-or-later

//! Device list, device details, and the decision prompt.

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, divider, icon, row, text, toggler};
use cosmic::{Apply, Element, theme};

use crate::fl;
use crate::state::State;
use crate::usbguard::{Device, Target};

use super::Action;
use super::format;
use super::icons;

/// Spacing tokens from the active COSMIC theme.
fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

/// A short coloured word describing the device's authorisation.
fn state_badge<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let (label, class) = if state.is_pending(device.id) {
        (fl!("state-pending"), theme::Text::Accent)
    } else {
        match device.target {
            Target::Allow => (format::target_label(Target::Allow), super::success_text()),
            Target::Block => (format::target_label(Target::Block), super::warning_text()),
            Target::Reject => (format::target_label(Target::Reject), super::danger_text()),
            other => (format::target_label(other), theme::Text::Default),
        }
    };

    text::caption(label).class(class).into()
}

/// The buttons offered for a device, given what state it is in.
///
/// A device awaiting a decision gets Allow and Block. An already-allowed
/// device gets Revoke, which removes the standing rule as well as blocking it
/// now — blocking alone would leave a rule that re-authorises it on the next
/// replug, which is exactly the trap this app exists to avoid.
fn actions<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let busy = state.busy.contains(&device.id);
    let permanent = state.permanent;
    let id = device.id;

    let mut controls = row::with_capacity(3).spacing(space.space_xxs);

    if state.is_pending(id) || device.target != Target::Allow {
        let allow =
            button::suggested(fl!("allow")).on_press_maybe((!busy).then_some(Action::Decide {
                device_id: id,
                target: Target::Allow,
                permanent,
            }));
        controls = controls.push(allow);
    }

    if device.target == Target::Allow {
        let revoke = button::destructive(fl!("revoke"))
            .on_press_maybe((!busy).then_some(Action::Revoke { device_id: id }));
        controls = controls.push(revoke);
    } else if state.is_pending(id) {
        let block =
            button::standard(fl!("block")).on_press_maybe((!busy).then_some(Action::Decide {
                device_id: id,
                target: Target::Block,
                permanent,
            }));
        controls = controls.push(block);
    }

    controls.into()
}

/// One row of the device list.
pub fn row_view<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let expanded = state.selected == Some(device.id);

    let heading = column::with_capacity(2)
        .push(text::body(device.display_name()))
        .push(text::caption(format::device_summary(device)))
        .spacing(space.space_xxxs)
        .width(Length::Fill);

    let summary = row::with_capacity(4)
        .push(icon::from_name(icons::for_device(device)).size(24))
        .push(heading)
        .push(state_badge(state, device))
        .align_y(Alignment::Center)
        .spacing(space.space_s)
        .apply(button::custom)
        .class(theme::Button::Transparent)
        .padding(space.space_xs)
        .width(Length::Fill)
        .on_press(Action::Select(if expanded {
            None
        } else {
            Some(device.id)
        }));

    let mut content = column::with_capacity(4)
        .push(summary)
        .spacing(space.space_xxs);

    for warning in warnings_for(state, device) {
        content = content.push(
            row::with_capacity(2)
                .push(icon::from_name(icons::WARNING).size(16))
                .push(text::caption(warning))
                .align_y(Alignment::Center)
                .spacing(space.space_xxs)
                .apply(container)
                .padding([0, space.space_xs]),
        );
    }

    if expanded {
        content = content.push(detail(state, device));
    }

    content = content.push(
        actions(state, device)
            .apply(container)
            .padding([0, space.space_xs])
            .width(Length::Fill)
            .align_x(Alignment::End),
    );

    content.into()
}

/// Warnings to show inline for a device, honouring the user's preference.
fn warnings_for(state: &State, device: &Device) -> Vec<String> {
    if !state.config.settings.warn_input_capable {
        return Vec::new();
    }
    // Only the keystroke-injection warning is worth the vertical space in the
    // list; the rest are shown in the expanded detail.
    if device.is_input_capable() {
        vec![fl!("warning-input-capable")]
    } else {
        Vec::new()
    }
}

/// One labelled field of the expanded detail.
fn field<'a>(label: String, value: String, copyable: bool) -> Element<'a, Action> {
    let space = spacing();
    let shown = format::field_or_placeholder(&value);

    let mut line = row::with_capacity(3)
        .push(text::caption(label).width(Length::FillPortion(2)))
        .push(text::caption(shown).width(Length::FillPortion(3)))
        .spacing(space.space_xs)
        .align_y(Alignment::Center);

    if copyable && !value.trim().is_empty() {
        line = line.push(
            button::icon(icon::from_name(icons::COPY).size(14))
                .on_press(Action::Copy(value))
                .padding(space.space_xxxs),
        );
    }

    line.into()
}

/// The expanded detail for a device.
pub fn detail<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();

    let mut fields = column::with_capacity(8)
        .push(field(fl!("field-name"), device.name.clone(), false))
        .push(field(fl!("field-usb-id"), device.usb_id(), true))
        .push(field(fl!("field-serial"), device.serial.clone(), true))
        .push(field(fl!("field-port"), device.via_port.clone(), false))
        .push(field(
            fl!("field-connection"),
            device.connect_type.clone(),
            false,
        ))
        .push(field(
            fl!("field-interfaces"),
            device
                .interfaces
                .iter()
                .map(|i| format!("{} ({i})", i.class_name()))
                .collect::<Vec<_>>()
                .join(", "),
            false,
        ))
        // The hash is the only identifier that a device cannot forge by
        // claiming someone else's vendor and product IDs, so it is shown and
        // made copyable rather than hidden as an implementation detail.
        .push(field(fl!("field-hash"), device.hash.clone(), true))
        .push(field(
            fl!("field-status"),
            format::target_label(device.target),
            false,
        ))
        .spacing(space.space_xxs);

    for warning in format::device_warnings(device) {
        fields = fields.push(text::caption(warning).class(theme::Text::Accent));
    }

    if device.hash.is_empty() {
        fields = fields.push(text::caption(fl!("warning-no-hash")).class(theme::Text::Accent));
    }

    let _ = state;

    container(fields)
        .padding(space.space_xs)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// The decision card shown for a device awaiting an answer.
pub fn prompt<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let busy = state.busy.contains(&device.id);
    let id = device.id;

    let mut body = column::with_capacity(6)
        .push(text::title4(fl!("prompt-heading")))
        .push(text::body(fl!(
            "prompt-description",
            name = device.display_name()
        )))
        .push(text::caption(format::device_summary(device)))
        .spacing(space.space_xxs);

    for warning in format::device_warnings(device) {
        body = body.push(
            row::with_capacity(2)
                .push(icon::from_name(icons::WARNING).size(16))
                .push(text::caption(warning))
                .align_y(Alignment::Center)
                .spacing(space.space_xxs),
        );
    }

    let remember = toggler(state.permanent)
        .label(fl!("remember-decision"))
        .on_toggle(Action::SetPermanent);

    let buttons = row::with_capacity(3)
        .push(button::text(fl!("dismiss")).on_press(Action::DismissPrompt { device_id: id }))
        .push(cosmic::widget::space::horizontal())
        .push(
            button::standard(fl!("block")).on_press_maybe((!busy).then_some(Action::Decide {
                device_id: id,
                target: Target::Block,
                permanent: state.permanent,
            })),
        )
        .push(
            button::suggested(fl!("allow")).on_press_maybe((!busy).then_some(Action::Decide {
                device_id: id,
                target: Target::Allow,
                permanent: state.permanent,
            })),
        )
        .spacing(space.space_xxs)
        .align_y(Alignment::Center);

    column::with_capacity(4)
        .push(body)
        .push(remember)
        .push(buttons)
        .spacing(space.space_s)
        .apply(container)
        .padding(space.space_s)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// The full device list, with an empty state.
pub fn list<'a>(state: &State) -> Element<'a, Action> {
    let space = spacing();
    let devices = state.visible_devices();

    if devices.is_empty() {
        return column::with_capacity(2)
            .push(text::body(fl!("devices-none")))
            .push(text::caption(fl!("devices-none-description")))
            .spacing(space.space_xxs)
            .apply(container)
            .padding(space.space_m)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into();
    }

    let mut list = column::with_capacity(devices.len() * 2).spacing(space.space_xxs);
    for (index, device) in devices.iter().enumerate() {
        if index > 0 {
            list = list.push(divider::horizontal::default());
        }
        list = list.push(row_view(state, device));
    }

    let hidden = state.hidden_count();
    if hidden > 0 {
        list = list.push(
            text::caption(fl!("devices-hidden", count = hidden))
                .apply(container)
                .padding(space.space_xs),
        );
    }

    list.into()
}
