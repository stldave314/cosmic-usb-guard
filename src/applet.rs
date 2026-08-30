// SPDX-License-Identifier: GPL-3.0-or-later

//! The COSMIC panel applet.
//!
//! The panel icon reflects protection status at a glance, and the popup is
//! where a newly connected device is allowed or denied. This is the binary
//! that watches for insertions, so it is the one that raises notifications and
//! opens itself when a decision is waiting.

use std::sync::Arc;

use cosmic::app::{Core, Task};
use cosmic::iced::core::window;
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, Length, Limits, Rectangle, Subscription};
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{button, column, container, divider, row, scrollable, text};
use cosmic::{Apply, Element};

use crate::constants::{
    APPLET_ID, APPLET_MAX_INLINE_PROMPTS, DEVICE_REFRESH_INTERVAL, POPUP_MAX_HEIGHT,
    POPUP_MAX_WIDTH, POPUP_MIN_WIDTH,
};
use crate::notify::{ACTION_ALLOW, ACTION_BLOCK, ACTION_DETAILS, Notifier};
use crate::state::{Effect, State};
use crate::tasks::{self, Refreshed};
use crate::ui::{self, Action};
use crate::usbguard::{Event, Target};
use crate::{debug_log, fl};

/// Messages handled by the applet.
#[derive(Debug, Clone)]
pub enum Message {
    /// The user did something in a shared view.
    Ui(Action),
    /// The daemon reported something.
    ///
    /// Boxed: an `Event` carries a full device list, which would otherwise
    /// make every message in this enum as large as the biggest one.
    Event(Box<Event>),
    /// A refresh finished.
    Refreshed(Arc<Refreshed>),
    /// A decision finished.
    Decided(u32, Result<(), String>),
    /// A revocation finished.
    Revoked(u32, Result<usize, String>),
    /// The notification service connected, or turned out to be unavailable.
    NotifierReady(Option<Notifier>),
    /// A notification was posted for a device.
    Notified(u32, Option<u32>),
    /// The user activated a notification action.
    NotificationAction(u32, String),
    /// Time to re-check health.
    Tick,
    /// The popup was closed by the compositor.
    PopupClosed(Id),
    /// Open the popup with no anchor override, e.g. because a device arrived.
    OpenPopup,
    /// A surface action to hand back to the runtime.
    Surface(cosmic::surface::Action),
    /// Stop showing the current error.
    DismissError,
}

/// The applet.
pub struct Applet {
    core: Core,
    state: State,
    popup: Option<Id>,
    notifier: Option<Notifier>,
}

impl Default for Applet {
    fn default() -> Self {
        Self {
            core: Core::default(),
            state: State::new(),
            popup: None,
            notifier: None,
        }
    }
}

impl Applet {
    fn refresh(&self) -> Task<Message> {
        cosmic::task::future(async { Message::Refreshed(tasks::refresh().await) })
    }

    /// Run the effects a state transition asked for.
    fn run_effects(&mut self, effects: Vec<Effect>) -> Task<Message> {
        let mut tasks = Vec::new();

        for effect in effects {
            match effect {
                Effect::Notify(device_id) => {
                    let (Some(notifier), Some(device)) =
                        (self.notifier.clone(), self.state.device(device_id).cloned())
                    else {
                        continue;
                    };
                    tasks.push(cosmic::task::future(async move {
                        let id = notifier.prompt(&device, 0).await;
                        Message::Notified(device_id, id)
                    }));
                }

                Effect::CloseNotification(id) => {
                    let Some(notifier) = self.notifier.clone() else {
                        continue;
                    };
                    tasks.push(cosmic::task::future(async move {
                        notifier.close(id).await;
                        // Nothing to report; a tick is the cheapest no-op that
                        // keeps the task's message type consistent.
                        Message::Tick
                    }));
                }

                Effect::OpenPopup => {
                    if self.popup.is_none() {
                        tasks.push(cosmic::task::message(Message::OpenPopup));
                    }
                }

                Effect::ReloadHistory => self.state.reload_history(),
            }
        }

        Task::batch(tasks)
    }

    /// Build the popup, optionally anchored to the panel button's bounds.
    fn open_popup(&mut self, anchor: Option<Rectangle>) -> Task<Message> {
        if self.popup.is_some() {
            return Task::none();
        }

        cosmic::task::message(Message::Surface(app_popup::<Applet>(
            |_| Default::default(),
            move |state: &mut Applet| {
                let new_id = Id::unique();
                state.popup = Some(new_id);

                let mut settings = state.core.applet.get_popup_settings(
                    state.core.main_window_id().unwrap_or(Id::NONE),
                    new_id,
                    None,
                    None,
                    None,
                );

                // Anchor to the button that was clicked. Without bounds — when
                // a device arrival opened the popup — the applet window's own
                // default anchor is already correct.
                if let Some(bounds) = anchor {
                    settings.positioner.anchor_rect = Rectangle {
                        x: bounds.x as i32,
                        y: bounds.y as i32,
                        width: bounds.width as i32,
                        height: bounds.height as i32,
                    };
                }

                settings
            },
            Some(Box::new(move |state: &Applet| {
                state.popup_view().map(cosmic::Action::App)
            })),
        )))
    }

    fn close_popup(&mut self) -> Task<Message> {
        let Some(id) = self.popup.take() else {
            return Task::none();
        };
        cosmic::task::message(Message::Surface(destroy_popup(id)))
    }

    /// Handle a shared UI action.
    fn on_action(&mut self, action: Action) -> Task<Message> {
        match action {
            Action::Decide {
                device_id,
                target,
                permanent,
            } => {
                let Some(device) = self.state.device(device_id).cloned() else {
                    return Task::none();
                };
                self.state.busy.insert(device_id);
                self.state.record_decision(&device, target, permanent);
                cosmic::task::future(async move {
                    let (id, result) = tasks::decide(device_id, target, permanent).await;
                    Message::Decided(id, result)
                })
            }

            Action::Revoke { device_id } => {
                let Some(device) = self.state.device(device_id).cloned() else {
                    return Task::none();
                };
                self.state.busy.insert(device_id);
                let hash = device.hash.clone();
                cosmic::task::future(async move {
                    let (id, result) = tasks::revoke(device_id, hash).await;
                    Message::Revoked(id, result)
                })
            }

            Action::Select(id) => {
                self.state.selected = id;
                Task::none()
            }

            Action::SetPermanent(value) => {
                self.state.permanent = value;
                Task::none()
            }

            Action::DismissPrompt { device_id } => {
                let effects = self.state.resolve_pending(device_id);
                self.run_effects(effects)
            }

            Action::Copy(text) | Action::CopyRemedy(text) => cosmic::iced::clipboard::write(text),

            Action::Refresh => self.refresh(),

            Action::Setting(change) => {
                self.state.apply_setting(change);
                Task::none()
            }

            Action::SetHistoryFilter(filter) => {
                self.state.history_filter = filter;
                Task::none()
            }

            Action::ClearHistory => {
                self.state.clear_history();
                Task::none()
            }

            Action::OpenApp => {
                launch_main_window();
                self.close_popup()
            }

            Action::Quit => cosmic::iced::exit(),
        }
    }

    /// The popup contents.
    fn popup_view(&self) -> Element<'_, Message> {
        let space = cosmic::theme::active().cosmic().spacing;

        let mut content = column::with_capacity(6).spacing(space.space_s);

        if let Some(error) = self.state.error.as_ref() {
            content = content.push(
                cosmic::widget::warning(error.clone())
                    .on_close(Message::DismissError)
                    .into_widget(),
            );
        }

        let mut inner = column::with_capacity(6).spacing(space.space_s);

        if let Some(banner) = ui::health::banner(&self.state) {
            inner = inner.push(banner);
        }

        // Prompts come before the device list: they are the only thing here
        // the user has to act on.
        let pending = self.state.pending_devices();
        for device in pending.iter().take(APPLET_MAX_INLINE_PROMPTS) {
            inner = inner.push(ui::device::prompt(&self.state, device));
        }
        if pending.len() > APPLET_MAX_INLINE_PROMPTS {
            let overflow = pending.len() - APPLET_MAX_INLINE_PROMPTS;
            inner = inner.push(text::caption(fl!("devices-pending", count = overflow)));
        }

        inner = inner.push(divider::horizontal::default());
        inner = inner.push(text::heading(fl!("devices-heading")));
        inner = inner.push(ui::device::list(&self.state));

        content = content.push(
            Element::from(
                inner
                    .apply(scrollable)
                    .height(Length::Shrink)
                    .apply(container)
                    .max_height(POPUP_MAX_HEIGHT),
            )
            .map(Message::Ui),
        );

        content = content.push(divider::horizontal::default());
        content = content.push(
            row::with_capacity(3)
                .push(button::text(fl!("open-app")).on_press(Message::Ui(Action::OpenApp)))
                .push(cosmic::widget::space::horizontal())
                .push(button::text(fl!("refresh")).on_press(Message::Ui(Action::Refresh)))
                .push(button::destructive(fl!("quit")).on_press(Message::Ui(Action::Quit)))
                .align_y(Alignment::Center)
                .spacing(space.space_xxs),
        );

        self.core
            .applet
            .popup_container(content)
            .limits(
                Limits::NONE
                    .min_width(POPUP_MIN_WIDTH)
                    .max_width(POPUP_MAX_WIDTH)
                    .max_height(POPUP_MAX_HEIGHT),
            )
            .into()
    }

    /// Tooltip text for the panel icon.
    fn tooltip(&self) -> String {
        if !self.state.connected {
            return fl!("status-disconnected");
        }
        let pending = self.state.pending.len();
        if pending > 0 {
            return fl!("devices-pending", count = pending);
        }
        if !self.state.health_checked {
            return fl!("status-checking");
        }
        ui::format::status_headline(self.state.health.worst())
    }
}

impl cosmic::Application for Applet {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APPLET_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let applet = Self {
            core,
            ..Default::default()
        };

        debug_log!(crate::debug::UI, "applet initialised");
        let task = Task::batch([
            applet.refresh(),
            cosmic::task::future(async {
                // Connecting the notifier up front means the first insertion
                // does not have to wait for a D-Bus round trip before it can
                // be announced.
                Message::NotifierReady(Notifier::connect().await)
            }),
        ]);

        (applet, task)
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            crate::subscription::usbguard_events().map(|e| Message::Event(Box::new(e))),
            crate::subscription::health_ticks().map(|()| Message::Tick),
            cosmic::iced::time::every(DEVICE_REFRESH_INTERVAL).map(|_| Message::Tick),
            crate::subscription::notification_actions()
                .map(|(id, action)| Message::NotificationAction(id, action)),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ui(action) => self.on_action(action),

            Message::Event(event) => {
                let effects = self.state.apply_event(*event);
                let run = self.run_effects(effects);
                Task::batch([run, self.refresh()])
            }

            Message::Refreshed(refreshed) => {
                match &refreshed.devices {
                    Ok(devices) => {
                        let effects = self.state.apply_event(Event::Connected {
                            devices: devices.clone(),
                        });
                        let _ = self.run_effects(effects);
                    }
                    Err(reason) => {
                        let effects = self.state.apply_event(Event::Disconnected {
                            reason: reason.clone(),
                        });
                        let _ = self.run_effects(effects);
                    }
                }
                if let Some(hashes) = refreshed.known_hashes.clone() {
                    self.state.set_known_hashes(hashes);
                }
                self.state.set_health(refreshed.health.clone());
                Task::none()
            }

            Message::Decided(device_id, result) => {
                self.state.busy.remove(&device_id);
                match result {
                    Ok(()) => {
                        let effects = self.state.resolve_pending(device_id);
                        let run = self.run_effects(effects);
                        Task::batch([run, self.refresh()])
                    }
                    Err(message) => {
                        self.state.error = Some(message);
                        Task::none()
                    }
                }
            }

            Message::Revoked(device_id, result) => {
                self.state.busy.remove(&device_id);
                match result {
                    Ok(removed) => {
                        if let Some(device) = self.state.device(device_id).cloned() {
                            self.state.record_revocation(&device, removed);
                        }
                        self.refresh()
                    }
                    Err(message) => {
                        self.state.error = Some(message);
                        Task::none()
                    }
                }
            }

            Message::NotifierReady(notifier) => {
                if notifier.is_none() {
                    debug_log!(
                        crate::debug::NOTIFY,
                        "no notification service; prompts will only appear in the popup"
                    );
                }
                self.notifier = notifier;
                Task::none()
            }

            Message::Notified(device_id, Some(notification_id)) => {
                self.state.set_notification(device_id, notification_id);
                Task::none()
            }
            Message::Notified(_, None) => Task::none(),

            Message::NotificationAction(notification_id, action) => {
                let Some(device_id) = self.state.device_for_notification(notification_id) else {
                    return Task::none();
                };
                match action.as_str() {
                    ACTION_ALLOW => self.on_action(Action::Decide {
                        device_id,
                        target: Target::Allow,
                        permanent: self.state.permanent,
                    }),
                    ACTION_BLOCK => self.on_action(Action::Decide {
                        device_id,
                        target: Target::Block,
                        permanent: self.state.permanent,
                    }),
                    ACTION_DETAILS => {
                        self.state.selected = Some(device_id);
                        self.open_popup(None)
                    }
                    _ => Task::none(),
                }
            }

            Message::Tick => self.refresh(),

            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
                Task::none()
            }

            Message::OpenPopup => self.open_popup(None),

            Message::Surface(action) => {
                cosmic::task::message(cosmic::Action::Cosmic(cosmic::app::Action::Surface(action)))
            }

            Message::DismissError => {
                self.state.error = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let icon_name = ui::icons::for_status(
            self.state.connected,
            &self.state.health,
            self.state.pending.len(),
        );

        let open = self.popup;
        let button = self
            .core
            .applet
            .icon_button(icon_name)
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = open {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<Applet>(
                        |_| Default::default(),
                        move |state: &mut Applet| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let mut settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap_or(Id::NONE),
                                new_id,
                                None,
                                None,
                                None,
                            );
                            settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };
                            settings
                        },
                        Some(Box::new(move |state: &Applet| {
                            state.popup_view().map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        self.core
            .applet
            .applet_tooltip::<Message>(
                button,
                self.tooltip(),
                self.popup.is_some(),
                Message::Surface,
                None,
            )
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        self.popup_view()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

/// Launch the window application.
///
/// Spawned detached so the applet is not the parent of a long-lived window,
/// and failures are reported rather than swallowed — a dead "Open" button with
/// no explanation is worse than an error in the log.
fn launch_main_window() {
    match std::process::Command::new(crate::constants::PKG_NAME).spawn() {
        Ok(_) => debug_log!(crate::debug::UI, "launched the main window"),
        Err(e) => crate::error_log!(crate::debug::UI, "could not launch the main window: {e}"),
    }
}

/// Run the applet.
pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Applet>(())
}
