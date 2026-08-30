// SPDX-License-Identifier: GPL-3.0-or-later

//! The main window application.
//!
//! Four pages over the same [`State`] the applet uses: the device list, the
//! decision history, the installation status, and settings.

use std::sync::Arc;

use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Length, Limits, Subscription};
use cosmic::widget::{self, column, container, icon, nav_bar, row, scrollable, text};
use cosmic::{ApplicationExt, Apply, Element};

use crate::constants::{
    APP_ID, DEVICE_REFRESH_INTERVAL, REPOSITORY, VERSION, WINDOW_DEFAULT_SIZE, WINDOW_MIN_SIZE,
};
use crate::state::{Effect, State};
use crate::tasks::{self, Refreshed};
use crate::ui::{self, Action};
use crate::usbguard::Event;
use crate::{debug_log, fl};

/// One page of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    /// Connected devices and their authorisation.
    Devices,
    /// The decision journal.
    History,
    /// Installation health.
    Status,
    /// User settings.
    Settings,
}

/// Messages handled by the window application.
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
    /// Time to re-check health.
    Tick,
    /// Stop showing the current error.
    DismissError,
}

/// The window application.
pub struct App {
    core: Core,
    state: State,
    nav: nav_bar::Model,
}

impl App {
    /// Kick off a refresh of devices, policy and health.
    fn refresh(&self) -> Task<Message> {
        cosmic::task::future(async { Message::Refreshed(tasks::refresh().await) })
    }

    /// Run the effects a state transition asked for.
    ///
    /// The window has no panel popup to open and posts no notifications of its
    /// own — the applet owns both — so only the history reload applies here.
    fn run_effects(&mut self, effects: Vec<Effect>) -> Task<Message> {
        for effect in effects {
            if effect == Effect::ReloadHistory {
                self.state.reload_history();
            }
        }
        Task::none()
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

            // The window is the main app; there is nothing to open.
            Action::OpenApp => Task::none(),

            Action::Quit => cosmic::iced::exit(),
        }
    }

    /// The currently selected page.
    fn page(&self) -> Page {
        self.nav
            .active_data::<Page>()
            .copied()
            .unwrap_or(Page::Devices)
    }

    /// The device page: prompts first, then the full list.
    fn devices_view(&self) -> Element<'_, Action> {
        let space = cosmic::theme::active().cosmic().spacing;

        let mut content = column::with_capacity(4).spacing(space.space_m);

        if let Some(banner) = ui::health::banner(&self.state) {
            content = content.push(banner);
        }

        let pending = self.state.pending_devices();
        if !pending.is_empty() {
            let mut prompts = column::with_capacity(pending.len()).spacing(space.space_s);
            for device in pending {
                prompts = prompts.push(ui::device::prompt(&self.state, device));
            }
            content = content.push(prompts);
        }

        content = content.push(
            row::with_capacity(2)
                .push(text::title3(fl!("devices-heading")).width(Length::Fill))
                .push(widget::button::standard(fl!("refresh")).on_press(Action::Refresh))
                .align_y(Alignment::Center)
                .spacing(space.space_s),
        );

        content = content.push(ui::device::list(&self.state));
        content.into()
    }

    /// The about page content, shown in the settings page footer.
    fn about_view(&self) -> Element<'_, Action> {
        let space = cosmic::theme::active().cosmic().spacing;

        column::with_capacity(4)
            .push(text::title4(fl!("app-title")))
            .push(text::caption(fl!("app-description")))
            .push(text::caption(fl!("version", version = VERSION)))
            .push(widget::button::link(REPOSITORY).on_press(Action::Copy(REPOSITORY.to_string())))
            .spacing(space.space_xxs)
            .into()
    }
}

impl cosmic::Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let mut nav = nav_bar::Model::default();
        nav.insert()
            .text(fl!("page-devices"))
            .icon(icon::from_name(ui::icons::DEVICE))
            .data(Page::Devices)
            .activate();
        nav.insert()
            .text(fl!("page-history"))
            .icon(icon::from_name("document-open-recent-symbolic"))
            .data(Page::History);
        nav.insert()
            .text(fl!("page-status"))
            .icon(icon::from_name(ui::icons::PANEL_OK))
            .data(Page::Status);
        nav.insert()
            .text(fl!("page-settings"))
            .icon(icon::from_name("preferences-system-symbolic"))
            .data(Page::Settings);

        let mut state = State::new();
        state.reload_history();

        let mut app = Self { core, state, nav };

        // Without this the window and its header render untitled, which in the
        // window list is indistinguishable from any other untitled window.
        let title = fl!("app-title");
        app.set_header_title(title.clone());
        let title_task = match app.core.main_window_id() {
            Some(id) => app.set_window_title(title, id),
            None => Task::none(),
        };

        let task = Task::batch([title_task, app.refresh()]);

        debug_log!(crate::debug::UI, "window application initialised");
        (app, task)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Message> {
        self.nav.activate(id);
        // The journal is appended to by both binaries, so re-read it when the
        // page is opened rather than trusting the in-memory copy.
        if self.page() == Page::History {
            self.state.reload_history();
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            crate::subscription::usbguard_events().map(|e| Message::Event(Box::new(e))),
            crate::subscription::health_ticks().map(|()| Message::Tick),
            cosmic::iced::time::every(DEVICE_REFRESH_INTERVAL).map(|_| Message::Tick),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ui(action) => self.on_action(action),

            Message::Event(event) => {
                let effects = self.state.apply_event(*event);
                let reload = self.run_effects(effects);
                // A policy change may have added or removed a standing rule.
                Task::batch([reload, self.refresh()])
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
                        let reload = self.run_effects(effects);
                        Task::batch([reload, self.refresh()])
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

            Message::Tick => self.refresh(),

            Message::DismissError => {
                self.state.error = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let space = cosmic::theme::active().cosmic().spacing;

        let page: Element<'_, Action> = match self.page() {
            Page::Devices => self.devices_view(),
            Page::History => ui::history::view(&self.state),
            Page::Status => ui::health::view(&self.state),
            Page::Settings => column::with_capacity(2)
                .push(ui::settings::view(&self.state))
                .push(self.about_view())
                .spacing(space.space_l)
                .into(),
        };

        let mut content = column::with_capacity(2).spacing(space.space_s);

        if let Some(error) = self.state.error.as_ref() {
            content = content.push(
                widget::warning(error.clone())
                    .on_close(Message::DismissError)
                    .into_widget(),
            );
        }

        content = content.push(page.map(Message::Ui));

        content
            .apply(container)
            .padding(space.space_m)
            .width(Length::Fill)
            .apply(scrollable)
            .height(Length::Fill)
            .into()
    }
}

/// Run the window application.
pub fn run() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default()
        .size(cosmic::iced::Size::new(
            WINDOW_DEFAULT_SIZE.0,
            WINDOW_DEFAULT_SIZE.1,
        ))
        .size_limits(
            Limits::NONE
                .min_width(WINDOW_MIN_SIZE.0)
                .min_height(WINDOW_MIN_SIZE.1),
        );

    cosmic::app::run::<App>(settings, ())
}
