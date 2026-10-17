// SPDX-License-Identifier: GPL-3.0-or-later

//! Device list, device details, and the decision prompt.

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{
    button, column, container, divider, icon, row, settings, text, text_input, toggler,
};
use cosmic::{Apply, Element, theme};

use crate::fl;
use crate::state::State;
use crate::usbguard::{Device, Target};

use super::format;
use super::icons;
use super::{Action, HookField};

/// Spacing tokens from the active COSMIC theme.
fn spacing() -> cosmic::cosmic_theme::Spacing {
    theme::active().cosmic().spacing
}

/// A short colored word describing the device's authorization.
fn state_badge<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let pending = device.daemon_id().is_some_and(|id| state.is_pending(id));
    let (label, class) = if pending {
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

/// Whether Allow has anything left to do for a device.
///
/// Plugged in, it is offered until the device is allowed by both the live
/// target and the policy — the path back from an accidental "no". Not plugged
/// in, there is no live target, so the standing rule is all a decision can
/// change: once that already says allow, the button would promise something
/// that has already happened.
fn offers_allow(connected: bool, live: Target, standing: Option<Target>) -> bool {
    let allowed_always = standing == Some(Target::Allow);
    if connected {
        live != Target::Allow || !allowed_always
    } else {
        !allowed_always
    }
}

/// The buttons offered for a device, given what state it is in.
///
/// The rule is that every decision this app can make must be reachable in
/// reverse. A device that was permanently blocked or rejected is not
/// necessarily plugged in any more — and a *rejected* device is detached on
/// sight, so waiting for it to come back is not a plan — which is why the
/// controls key off the standing rule as well as the live target.
fn actions<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let key = device.key();
    let busy = state.busy.contains(&key);
    let pending = device.daemon_id().is_some_and(|id| state.is_pending(id));
    let standing = state.standing_target(device);

    // A decision has to be written as a rule in two cases the "remember this"
    // toggle does not cover.
    //
    // Not plugged in: there is nothing to authorize right now, so a rule is
    // the only thing a decision can be.
    //
    // Contradicted by a standing rule: a one-off decision would be undone on
    // the next replug, so the button would not do what its label says. The
    // device already has a permanent rule stating the opposite — replacing it
    // is the least that makes the click mean anything.
    let contradicted = standing.is_some_and(|standing| standing != Target::Allow);
    let permanent = state.permanent || !device.is_connected() || contradicted;
    // Changing a decision keeps the port binding it already had; binding or
    // releasing is the detail switch's job, not a side effect of Allow.
    let bind = state.standing_port(device).is_some() && state.port_mismatch(device).is_none();

    let enabled = |action: Action| (!busy).then_some(action);
    let mut controls = row::with_capacity(3).spacing(space.space_xxs);

    let allowed_now = device.is_connected() && device.target == Target::Allow;
    if offers_allow(device.is_connected(), device.target, standing) {
        controls = controls.push(button::suggested(fl!("allow")).on_press_maybe(enabled(
            Action::Decide {
                key: key.clone(),
                target: Target::Allow,
                permanent,
                bind,
            },
        )));
    }

    if pending {
        controls = controls.push(button::standard(fl!("block")).on_press_maybe(enabled(
            Action::Decide {
                key: key.clone(),
                target: Target::Block,
                permanent,
                bind,
            },
        )));
    }

    // Revoke covers the one case where removing the rule is not enough on its
    // own: the device is authorized *right now*, so it also has to be blocked
    // or it stays usable until it is unplugged.
    if allowed_now {
        controls = controls.push(
            button::destructive(fl!("revoke"))
                .on_press_maybe(enabled(Action::Revoke { key: key.clone() })),
        );
    } else if standing.is_some() {
        controls = controls
            .push(button::standard(fl!("forget")).on_press_maybe(enabled(Action::Forget { key })));
    }

    controls.into()
}

/// A caption naming what the policy will do the next time this device appears.
fn standing_caption<'a>(state: &State, device: &Device) -> Option<Element<'a, Action>> {
    let standing = state.standing_target(device)?;
    let label = match standing {
        Target::Allow => fl!("standing-allow"),
        Target::Block => fl!("standing-block"),
        Target::Reject => fl!("standing-reject"),
        other => fl!("standing-other", target = format::target_label(other)),
    };
    let class = match standing {
        Target::Allow => super::success_text(),
        Target::Block => super::warning_text(),
        Target::Reject => super::danger_text(),
        _ => theme::Text::Default,
    };
    Some(text::caption(label).class(class).into())
}

/// One row of the device list.
pub fn row_view<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let key = device.key();
    let expanded = state.selected.as_ref() == Some(&key);

    let mut heading = column::with_capacity(4)
        .push(text::body(device.display_name()))
        .push(text::caption(format::device_summary(device)))
        .spacing(space.space_xxxs)
        .width(Length::Fill);

    // For a device that is not plugged in, the standing rule *is* its status —
    // there is no live authorization to report — so it goes on the row rather
    // than being buried in the detail.
    if !device.is_connected()
        && let Some(caption) = standing_caption(state, device)
    {
        heading = heading.push(caption);
    }

    if state.is_internal(device) {
        heading = heading.push(text::caption(fl!("device-internal")));
    }

    // `Button::ListItem`, not `Button::Transparent`. A button sets the text
    // color for everything inside it, and Transparent's foreground color is
    // literally rgba(0,0,0,0) — so the device name and summary rendered
    // invisible while the status badge, which sets its own color, did not.
    let summary = row::with_capacity(4)
        .push(icon::from_name(icons::for_device(device)).size(24))
        .push(heading)
        .push(if device.is_connected() {
            state_badge(state, device)
        } else {
            text::caption(fl!("state-disconnected"))
                .class(theme::Text::Default)
                .into()
        })
        .align_y(Alignment::Center)
        .spacing(space.space_s)
        .apply(button::custom)
        .class(super::list_row_style())
        .padding(space.space_xs)
        .width(Length::Fill)
        .on_press(Action::Select((!expanded).then(|| key.clone())));

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

/// Warnings to show inline for a device, honoring the user's preference.
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

/// A switch for the end of a [`settings::item`] row.
///
/// The row puts the label on the left at full width and the switch on the
/// right, the same as the Settings page. A toggler's own `.label()` instead
/// sets the label hard against the switch, which looked like one cramped
/// widget rather than a label and its control.
fn switch<'a>(
    value: bool,
    on_toggle: impl Fn(bool) -> Action + 'a,
) -> cosmic::widget::Toggler<'a, Action> {
    toggler(value).width(Length::Shrink).on_toggle(on_toggle)
}

/// Width of the trailing slot every field row reserves for a copy button.
///
/// Reserved whether or not the row has a button. The label and value columns
/// share what is left by proportion, so a button present on some rows and
/// absent on others used to shift those rows' values sideways and make them
/// taller, which is what made the detail look ragged.
const FIELD_ACTION_SLOT: f32 = 32.0;

/// One labeled field of the expanded detail.
fn field<'a>(label: String, value: String, copyable: bool) -> Element<'a, Action> {
    let space = spacing();
    // Shown trimmed (USBGuard reports some names with a leading space, which
    // set them off from the column), copied exactly as reported.
    let shown = format::field_or_placeholder(value.trim());

    let action: Element<'a, Action> = if copyable && !value.trim().is_empty() {
        button::icon(icon::from_name(icons::COPY).size(16))
            .on_press(Action::Copy(value))
            .into()
    } else {
        cosmic::widget::Space::new().into()
    };

    row::with_capacity(3)
        .push(text::body(label).width(Length::FillPortion(2)))
        // Hashes and serials are one long word. Without breaking inside a
        // word when it has to, the descriptor hash ran under the copy button
        // and out of the card.
        .push(
            text::body(shown)
                .wrapping(cosmic::iced::widget::text::Wrapping::WordOrGlyph)
                .width(Length::FillPortion(3)),
        )
        .push(
            container(action)
                .width(Length::Fixed(FIELD_ACTION_SLOT))
                .height(Length::Fixed(FIELD_ACTION_SLOT))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .spacing(space.space_s)
        .align_y(Alignment::Center)
        .into()
}

/// The expanded detail for a device.
pub fn detail<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();

    // An unplugged device has no live authorization. Its rule target is what
    // the parsed rule says, and showing "Allowed" for something that is not
    // attached reads as if it were in use; the standing rule is reported
    // separately below.
    let status = if device.is_connected() {
        format::target_label(device.target)
    } else {
        fl!("state-disconnected")
    };

    let fields = column::with_capacity(8)
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
        .push(field(fl!("field-status"), status, false))
        .spacing(space.space_xxs);

    // What the policy says and anything worth warning about, as a block of
    // its own rather than more rows that look like fields.
    let mut notes = column::with_capacity(5).spacing(space.space_xxs);
    let mut has_notes = false;

    if let Some(caption) = standing_caption(state, device) {
        notes = notes.push(caption);
        has_notes = true;
    }

    // A one-off decision leaves the device in a state the policy will undo on
    // the next replug. Silence here is how someone ends up sure they fixed
    // something that comes back.
    if let Some(standing) = state.standing_rule_conflicts(device) {
        notes = notes.push(
            text::caption(fl!(
                "warning-standing-conflict",
                target = format::target_label(standing)
            ))
            .class(super::warning_text()),
        );
        has_notes = true;
    }

    if let Some(rule) = state.overriding_rule(device) {
        notes = notes.push(
            text::caption(fl!("warning-overriding-rule", rule = rule.rule.raw.clone()))
                .class(super::warning_text()),
        );
        has_notes = true;
    }

    for warning in format::device_warnings(device) {
        notes = notes.push(text::caption(warning).class(theme::Text::Accent));
        has_notes = true;
    }

    if device.hash.is_empty() {
        notes = notes.push(text::caption(fl!("warning-no-hash")).class(theme::Text::Accent));
        has_notes = true;
    }

    // Marking is keyed on the descriptor hash, so a device that reports none
    // cannot be marked at all. Saying that is better than a toggle that
    // silently does nothing.
    let internal: Element<'a, Action> = if device.hash.is_empty() {
        text::caption(fl!("device-internal-no-hash")).into()
    } else {
        let hash = device.hash.clone();
        settings::item::builder(fl!("device-internal-toggle"))
            .description(fl!("device-internal-description"))
            .control(switch(state.is_internal(device), move |internal| {
                Action::SetInternal {
                    hash: hash.clone(),
                    internal,
                }
            }))
            .into()
    };

    let mut card = column::with_capacity(7).push(fields).spacing(space.space_s);
    if has_notes {
        card = card.push(divider::horizontal::default()).push(notes);
    }
    if let Some(binding) = binding_switch(state, device) {
        card = card.push(divider::horizontal::default()).push(binding);
    }
    card = card.push(divider::horizontal::default()).push(internal);

    let mut sections = column::with_capacity(2)
        .push(
            container(card)
                .padding(space.space_s)
                .class(theme::Container::Card)
                .width(Length::Fill),
        )
        .spacing(space.space_xs);

    // A hook can only be pinned to a device that has a durable identity, for
    // the same reason a permanent rule can only be.
    if !device.hash.is_empty() {
        sections = sections.push(hook_view(state, device));
    }

    sections.into()
}

/// The "bound to USB port" switch for a device with a standing rule.
///
/// `None` when there is no standing rule to bind. For a device that is not
/// plugged in and not bound, the switch is shown disabled with the reason:
/// there is no current port to bind it to, but the option should not seem to
/// have vanished.
fn binding_switch<'a>(state: &State, device: &Device) -> Option<Element<'a, Action>> {
    if !state.has_standing_rule(device) {
        return None;
    }
    let busy = state.busy.contains(&device.key());
    let bound = state.standing_port(device);

    let description = match bound {
        Some(port) => fl!("bound-port-description", port = port.to_string()),
        None if device.can_bind_port() => fl!("bound-port-description-unbound"),
        None => fl!("bound-port-unplugged"),
    };

    // Binding needs a current port; releasing never does.
    let can_change = !busy && (bound.is_some() || device.can_bind_port());
    let key = device.key();

    Some(
        settings::item::builder(fl!("bound-port-toggle"))
            .description(description)
            .control(
                toggler(bound.is_some())
                    .width(Length::Shrink)
                    .on_toggle_maybe(can_change.then_some(move |bound| Action::SetBound {
                        key: key.clone(),
                        bound,
                    })),
            )
            .into(),
    )
}

/// A short description of what is wrong with a hook's program.
fn hook_problem(problem: crate::hooks::Problem) -> String {
    use crate::hooks::Problem;
    match problem {
        Problem::NotSet => fl!("hook-problem-not-set"),
        Problem::NotAbsolute => fl!("hook-problem-not-absolute"),
        Problem::Missing => fl!("hook-problem-missing"),
        Problem::NotExecutable => fl!("hook-problem-not-executable"),
    }
}

/// The hook editor, or a summary of the hook already configured.
fn hook_view<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let hash = device.hash.clone();
    let editing = state
        .hook_draft
        .as_ref()
        .filter(|draft| draft.hash == device.hash);

    let mut body = column::with_capacity(8)
        .push(text::heading(fl!("hook-heading")))
        // Says the security rule out loud, next to the control that sets it
        // up, rather than only in the manual.
        .push(text::caption(fl!("hook-description")))
        .spacing(space.space_xs);

    match editing {
        Some(draft) => {
            body = body
                .push(
                    text_input(fl!("hook-label-placeholder"), draft.label.clone())
                        .label(fl!("hook-label"))
                        .on_input(|value| Action::HookEdit {
                            field: HookField::Label,
                            value,
                        }),
                )
                .push(
                    text_input(fl!("hook-program-placeholder"), draft.program.clone())
                        .label(fl!("hook-program"))
                        .on_input(|value| Action::HookEdit {
                            field: HookField::Program,
                            value,
                        }),
                )
                .push(
                    text_input(fl!("hook-arguments-placeholder"), draft.args.clone())
                        .label(fl!("hook-arguments"))
                        .on_input(|value| Action::HookEdit {
                            field: HookField::Args,
                            value,
                        }),
                )
                .push(
                    settings::item::builder(fl!("hook-enabled"))
                        .control(switch(draft.enabled, Action::HookEnabled)),
                );

            // Validate before saving, not after the drive goes in at 3 a.m.
            if let Some(problem) = draft.to_hook().problem() {
                body = body.push(text::caption(hook_problem(problem)).class(super::warning_text()));
            }

            body = body.push(
                row::with_capacity(3)
                    .push(button::text(fl!("dismiss")).on_press(Action::HookCancel))
                    .push(cosmic::widget::space::horizontal())
                    .push(button::suggested(fl!("hook-save")).on_press(Action::HookSave))
                    .spacing(space.space_xxs)
                    .align_y(Alignment::Center),
            );
        }

        None => match state.hook(device) {
            Some(hook) => {
                let name = if hook.label.is_empty() {
                    hook.program.display().to_string()
                } else {
                    hook.label.clone()
                };
                body = body.push(
                    row::with_capacity(3)
                        .push(icon::from_name(icons::HOOK).size(16))
                        .push(text::body(name).width(Length::Fill))
                        .push(text::caption(if hook.enabled {
                            fl!("hook-enabled")
                        } else {
                            fl!("state-unknown")
                        }))
                        .align_y(Alignment::Center)
                        .spacing(space.space_xs),
                );
                body = body.push(text::caption(hook.program.display().to_string()));

                if let Some(problem) = hook.problem() {
                    body = body
                        .push(text::caption(hook_problem(problem)).class(super::warning_text()));
                }

                body = body.push(
                    row::with_capacity(3)
                        .push(
                            button::standard(fl!("details"))
                                .on_press(Action::HookBegin { hash: hash.clone() }),
                        )
                        .push(cosmic::widget::space::horizontal())
                        .push(
                            button::destructive(fl!("hook-remove"))
                                .on_press(Action::HookRemove { hash }),
                        )
                        .spacing(space.space_xxs)
                        .align_y(Alignment::Center),
                );
            }
            None => {
                body = body.push(text::caption(fl!("hook-none"))).push(
                    row::with_capacity(2)
                        .push(cosmic::widget::space::horizontal())
                        .push(
                            button::standard(fl!("hook-heading"))
                                .on_press(Action::HookBegin { hash }),
                        )
                        .align_y(Alignment::Center),
                );
            }
        },
    }

    body = body.push(text::caption(fl!(
        "hook-variables",
        names = crate::hooks::ENV_ALL.join(", ")
    )));

    container(body)
        .padding(space.space_s)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// The decision card shown for a device awaiting an answer.
pub fn prompt<'a>(state: &State, device: &Device) -> Element<'a, Action> {
    let space = spacing();
    let key = device.key();
    let busy = state.busy.contains(&key);
    // A prompt is only ever raised for a device the daemon currently has, so
    // there is always an ID to dismiss against.
    let id = device.daemon_id().unwrap_or_default();

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

    // A device whose rule is bound to another port is here because of that,
    // and would otherwise look like a device the user had never seen.
    if let Some((bound, now)) = state.port_mismatch(device) {
        body = body.push(
            row::with_capacity(2)
                .push(icon::from_name(icons::WARNING).size(16))
                .push(text::caption(fl!(
                    "prompt-wrong-port",
                    bound = bound,
                    port = now
                )))
                .align_y(Alignment::Center)
                .spacing(space.space_xxs),
        );
    }

    let remember = settings::item::builder(fl!("remember-decision"))
        .control(switch(state.permanent, Action::SetPermanent));

    // Only offered where there is a port to bind to. Disabled rather than
    // hidden while "remember" is off, so it is clear binding exists and what
    // it depends on: a one-off decision writes no rule to bind.
    let bound_here = state.bind == Some(id);
    let bind = device.can_bind_port().then(|| {
        settings::item::builder(fl!("bind-to-port", port = device.via_port.clone()))
            .description(fl!("bind-to-port-description"))
            .control(
                toggler(state.permanent && bound_here)
                    .width(Length::Shrink)
                    .on_toggle_maybe(state.permanent.then_some(move |bind| Action::SetBind {
                        device_id: id,
                        bind,
                    })),
            )
    });

    let buttons = row::with_capacity(3)
        .push(button::text(fl!("dismiss")).on_press(Action::DismissPrompt { device_id: id }))
        .push(cosmic::widget::space::horizontal())
        .push(
            button::standard(fl!("block")).on_press_maybe((!busy).then_some(Action::Decide {
                key: key.clone(),
                target: Target::Block,
                permanent: state.permanent,
                bind: bound_here,
            })),
        )
        .push(
            button::suggested(fl!("allow")).on_press_maybe((!busy).then_some(Action::Decide {
                key,
                target: Target::Allow,
                permanent: state.permanent,
                bind: bound_here,
            })),
        )
        .spacing(space.space_xxs)
        .align_y(Alignment::Center);

    let mut options = column::with_capacity(2)
        .push(remember)
        .spacing(space.space_xs);
    if let Some(bind) = bind {
        options = options.push(bind);
    }

    column::with_capacity(4)
        .push(body)
        .push(options)
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
    let remembered = state.remembered_devices();

    if devices.is_empty() && remembered.is_empty() {
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

    let mut list =
        column::with_capacity((devices.len() + remembered.len()) * 2 + 3).spacing(space.space_xxs);

    for (index, device) in devices.iter().enumerate() {
        if index > 0 {
            list = list.push(divider::horizontal::default());
        }
        list = list.push(row_view(state, device));
    }

    // Devices the policy remembers but that are not plugged in, in their own
    // section so nothing here can be mistaken for something currently attached.
    if !remembered.is_empty() {
        list = list.push(divider::horizontal::default()).push(
            column::with_capacity(2)
                .push(text::heading(fl!("devices-remembered")))
                .push(text::caption(fl!("devices-remembered-description")))
                .spacing(space.space_xxxs)
                .apply(container)
                .padding([space.space_xs, space.space_xs]),
        );

        for (index, device) in remembered.iter().enumerate() {
            if index > 0 {
                list = list.push(divider::horizontal::default());
            }
            list = list.push(row_view(state, device));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_is_not_offered_for_an_unplugged_device_that_is_already_always_allowed() {
        // The button used to appear here, and pressing it could only remove
        // the rule and write it back.
        assert!(!offers_allow(false, Target::Allow, Some(Target::Allow)));

        // Still the way back from a remembered "no", and still offered when
        // the policy could not be read.
        assert!(offers_allow(false, Target::Block, Some(Target::Block)));
        assert!(offers_allow(false, Target::Block, Some(Target::Reject)));
        assert!(offers_allow(false, Target::Block, None));
    }

    #[test]
    fn allow_is_offered_for_a_plugged_in_device_until_both_halves_agree() {
        assert!(offers_allow(true, Target::Block, Some(Target::Allow)));
        assert!(offers_allow(true, Target::Allow, None));
        assert!(offers_allow(true, Target::Allow, Some(Target::Block)));
        assert!(!offers_allow(true, Target::Allow, Some(Target::Allow)));
    }
}
