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

use crate::constants::{APP_ID, REPOSITORY, VERSION, WINDOW_DEFAULT_SIZE, WINDOW_MIN_SIZE};
use crate::notify::{ACTION_ALLOW, ACTION_BLOCK, ACTION_DETAILS, ACTION_MANAGE, Notifier};
use crate::state::{Effect, State};
use crate::tasks::{self, Refreshed, Trigger};
use crate::tray::{Tray, TrayEvent, TrayState};
use crate::ui::{self, Action};
use crate::usbguard::{DeviceKey, Event};
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

impl Page {
    /// Parse the name used by the `--page` option.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "devices" => Some(Self::Devices),
            "history" => Some(Self::History),
            "status" => Some(Self::Status),
            "settings" => Some(Self::Settings),
            _ => None,
        }
    }

    /// Every page, in the order the navigation lists them.
    pub const ALL: [Self; 4] = [Self::Devices, Self::History, Self::Status, Self::Settings];
}

/// The page named by `--page`, if one was given and recognised.
///
/// Lets the app be opened straight at a page — useful for a desktop action, a
/// support instruction ("run it with `--page status`"), and for producing the
/// documentation screenshots reproducibly. An unrecognised name is ignored
/// rather than fatal: failing to start a security tool over a typo in an
/// optional argument would be the wrong trade.
fn requested_page() -> Option<Page> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let name = match arg.strip_prefix("--page=") {
            Some(name) => Some(name.to_string()),
            None if arg == "--page" => args.next(),
            None => None,
        };
        if let Some(name) = name {
            return Page::from_name(&name);
        }
    }
    None
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
    Decided(DeviceKey, Result<(), String>),
    /// A revocation finished.
    Revoked(DeviceKey, Result<usize, String>),
    /// Time to re-check health.
    Tick,
    /// Stop showing the current error.
    DismissError,
    /// The status icon finished starting, or failed to.
    TrayReady(Option<Arc<Tray>>),
    /// The user chose something from the status icon.
    Tray(TrayEvent),
    /// The notification service finished connecting.
    NotifierReady(Option<Notifier>),
    /// A prompt notification was posted, with the ID needed to withdraw it.
    Notified(u32, Option<u32>),
    /// The user activated an action on a notification.
    NotificationAction(u32, String),
    /// Show the window, raising it if it is already open.
    ShowWindow,
    /// Hide the window but keep watching.
    HideWindow,
    /// Nothing to report; lets a background task end without a message of
    /// its own.
    Noop,
}

/// The window application.
pub struct App {
    core: Core,
    state: State,
    nav: nav_bar::Model,
    /// The status icon, once it has registered.
    ///
    /// `Arc` because it is created on a background task and handed back
    /// through a message, and messages must be `Clone`.
    tray: Option<Arc<Tray>>,
    /// Connection to the notification service, if there is one.
    notifier: Option<Notifier>,
    /// Whether the window is currently hidden to the tray.
    hidden: bool,
}

impl App {
    /// Kick off a refresh of devices, policy and health.
    ///
    /// `Trigger::Manual` also clears the latch that stops a refused,
    /// Polkit-mediated call from being retried on the timer — so an
    /// authentication dialog can only ever be raised by something the user
    /// just did.
    fn refresh(&self, trigger: Trigger) -> Task<Message> {
        cosmic::task::future(async move { Message::Refreshed(tasks::refresh(trigger).await) })
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

                Effect::NotifyAutoBlocked(device_id) => {
                    let (Some(notifier), Some(device)) =
                        (self.notifier.clone(), self.state.device(device_id).cloned())
                    else {
                        continue;
                    };
                    tasks.push(cosmic::task::future(async move {
                        let id = notifier.auto_blocked(&device).await;
                        Message::Notified(device_id, id)
                    }));
                }

                Effect::CloseNotification(id) => {
                    let Some(notifier) = self.notifier.clone() else {
                        continue;
                    };
                    tasks.push(cosmic::task::future(async move {
                        notifier.close(id).await;
                        Message::Noop
                    }));
                }

                Effect::ShowWindow => tasks.push(cosmic::task::message(Message::ShowWindow)),

                Effect::RunHook(device_id) => {
                    let Some(device) = self.state.device(device_id).cloned() else {
                        continue;
                    };
                    let Some(hook) = self.state.hook(&device).cloned() else {
                        continue;
                    };
                    tasks.push(cosmic::task::future(async move {
                        // `hooks::run` re-checks that the device is authorised;
                        // this path must never be the only thing standing
                        // between an untrusted device and a program.
                        crate::hooks::run(&hook, &device).await;
                        Message::Noop
                    }));
                }

                Effect::ReloadHistory => self.state.reload_history(),
            }
        }

        Task::batch(tasks)
    }

    /// Push the current state to the status icon.
    ///
    /// Also creates or withdraws it, so the "show the status icon" setting
    /// takes effect immediately rather than at the next launch.
    fn sync_tray(&mut self) -> Task<Message> {
        let wanted = self.state.config.settings.show_tray_icon;

        match (wanted, self.tray.clone()) {
            (false, Some(tray)) => {
                tray.shutdown();
                self.tray = None;
                Task::none()
            }
            (true, None) => {
                let state = TrayState::from_state(&self.state);
                cosmic::task::future(async move {
                    Message::TrayReady(
                        Tray::spawn(state, crate::tray::sender())
                            .await
                            .map(Arc::new),
                    )
                })
            }
            (true, Some(tray)) => {
                let state = TrayState::from_state(&self.state);
                cosmic::task::future(async move {
                    tray.update(state).await;
                    Message::Noop
                })
            }
            (false, None) => Task::none(),
        }
    }

    /// Show the window, or bring it forward if it is already open.
    fn show_window(&mut self) -> Task<Message> {
        self.hidden = false;
        match self.core.main_window_id() {
            Some(id) => Task::batch([
                cosmic::iced::window::set_mode(id, cosmic::iced::window::Mode::Windowed),
                cosmic::iced::window::gain_focus(id),
            ]),
            None => Task::none(),
        }
    }

    /// Hide the window, leaving the process running.
    fn hide_window(&mut self) -> Task<Message> {
        self.hidden = true;
        match self.core.main_window_id() {
            Some(id) => cosmic::iced::window::set_mode(id, cosmic::iced::window::Mode::Hidden),
            None => Task::none(),
        }
    }

    /// Shut the status icon down and leave.
    fn quit(&mut self) -> Task<Message> {
        if let Some(tray) = self.tray.take() {
            tray.shutdown();
        }
        cosmic::iced::exit()
    }

    /// Handle a shared UI action.
    fn on_action(&mut self, action: Action) -> Task<Message> {
        match action {
            Action::Decide {
                key,
                target,
                permanent,
            } => {
                let Some(device) = self.state.device_by_key(&key).cloned() else {
                    return Task::none();
                };
                self.state.busy.insert(key);
                self.state.record_decision(&device, target, permanent);
                let decision = tasks::Decision::new(&device, target);
                cosmic::task::future(async move {
                    let (key, result) = tasks::decide(decision, target, permanent).await;
                    Message::Decided(key, result)
                })
            }

            Action::Revoke { key } => {
                let Some(device) = self.state.device_by_key(&key).cloned() else {
                    return Task::none();
                };
                // Revoke also blocks the device where it stands, which needs a
                // live device; without one there is nothing to block and
                // `Forget` is the action the view offers instead.
                let Some(daemon_id) = device.daemon_id() else {
                    return Task::none();
                };
                self.state.busy.insert(key.clone());
                let hash = device.hash.clone();
                cosmic::task::future(async move {
                    let (key, result) = tasks::revoke(key, daemon_id, hash).await;
                    Message::Revoked(key, result)
                })
            }

            Action::Forget { key } => {
                let Some(device) = self.state.device_by_key(&key).cloned() else {
                    return Task::none();
                };
                self.state.busy.insert(key.clone());
                let hash = device.hash.clone();
                cosmic::task::future(async move {
                    let (key, result) = tasks::forget(key, hash).await;
                    Message::Revoked(key, result)
                })
            }

            Action::HookBegin { hash } => {
                self.state.begin_hook(hash);
                Task::none()
            }

            Action::HookEdit { field, value } => {
                if let Some(draft) = self.state.hook_draft.as_mut() {
                    match field {
                        ui::HookField::Label => draft.label = value,
                        ui::HookField::Program => draft.program = value,
                        ui::HookField::Args => draft.args = value,
                    }
                }
                Task::none()
            }

            Action::HookEnabled(enabled) => {
                if let Some(draft) = self.state.hook_draft.as_mut() {
                    draft.enabled = enabled;
                }
                Task::none()
            }

            Action::HookSave => {
                self.state.save_hook();
                Task::none()
            }

            Action::HookCancel => {
                self.state.hook_draft = None;
                Task::none()
            }

            Action::HookRemove { hash } => {
                self.state.set_hook(hash, None);
                self.state.hook_draft = None;
                Task::none()
            }

            Action::SetInternal { hash, internal } => {
                let effects = self.state.set_internal(hash, internal);
                self.run_effects(effects)
            }

            Action::Select(key) => {
                self.state.selected = key;
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

            Action::Refresh => self.refresh(Trigger::Manual),

            Action::Setting(change) => {
                self.state.apply_setting(change);

                // Two settings mean more than a stored flag: one writes a file
                // outside the app's config, the other creates or withdraws the
                // status icon. Both have to be applied now, or the toggle
                // would appear to work and do nothing until the next launch.
                let settings = &self.state.config.settings;
                if matches!(
                    change,
                    ui::SettingChange::Autostart(_) | ui::SettingChange::StartMinimized(_)
                ) && let Err(e) =
                    crate::autostart::set(settings.autostart, settings.start_minimized)
                {
                    self.state.error = Some(fl!("error-autostart", message = e.to_string()));
                }

                if matches!(change, ui::SettingChange::ShowTrayIcon(_)) {
                    return self.sync_tray();
                }
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
            // Copy rather than open: the applet and window both run
            // un-sandboxed with access to the system bus, and handing a URL to
            // whatever handler the session has registered is a wider action
            // than this button needs to be.
            .push(
                row::with_capacity(2)
                    .push(
                        widget::button::link(fl!("repository"))
                            .on_press(Action::Copy(REPOSITORY.to_string())),
                    )
                    .push(
                        widget::button::link(fl!("support"))
                            .on_press(Action::Copy(crate::constants::issues_url())),
                    )
                    .spacing(space.space_xxs),
            )
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
        let wanted = requested_page().unwrap_or(Page::Devices);
        let mut nav = nav_bar::Model::default();
        for page in Page::ALL {
            let (label, icon_name) = match page {
                Page::Devices => (fl!("page-devices"), ui::icons::DEVICE),
                Page::History => (fl!("page-history"), ui::icons::HISTORY),
                Page::Status => (fl!("page-status"), ui::icons::PANEL_OK),
                Page::Settings => (fl!("page-settings"), ui::icons::SETTINGS),
            };
            let id = nav
                .insert()
                .text(label)
                .icon(icon::from_name(icon_name))
                .data(page)
                .id();
            if page == wanted {
                nav.activate(id);
            }
        }

        let mut state = State::new();
        state.reload_history();

        // Hiding the window with no status icon would strand the user with a
        // running process they cannot see or reach, so the icon setting is a
        // precondition rather than an independent choice. If the icon then
        // fails to register, `Message::TrayReady` shows the window again.
        let minimized = (crate::autostart::started_minimized()
            || state.config.settings.start_minimized)
            && state.config.settings.show_tray_icon;

        let mut app = Self {
            core,
            state,
            nav,
            tray: None,
            notifier: None,
            hidden: false,
        };

        // Without this the window and its header render untitled, which in the
        // window list is indistinguishable from any other untitled window.
        let title = fl!("app-title");
        app.set_header_title(title.clone());
        let title_task = match app.core.main_window_id() {
            Some(id) => app.set_window_title(title, id),
            None => Task::none(),
        };

        // Connect the notifier up front, so the first insertion does not wait
        // on a D-Bus round trip before it can be announced.
        let notifier =
            cosmic::task::future(async { Message::NotifierReady(Notifier::connect().await) });

        let tray = app.sync_tray();

        let hide = if minimized {
            debug_log!(crate::debug::UI, "starting minimised to the status icon");
            app.hide_window()
        } else {
            Task::none()
        };

        let task = Task::batch([
            title_task,
            app.refresh(Trigger::Manual),
            notifier,
            tray,
            hide,
        ]);

        debug_log!(crate::debug::UI, "window application initialised");
        (app, task)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Closing the window hides it rather than quitting, so USB prompting
    /// keeps working.
    ///
    /// Without this the close button silently turns off the thing the app is
    /// for: the daemon still blocks devices, but nothing asks the user about
    /// them and no hook ever runs. Quit is on the status icon's menu, and the
    /// behaviour is a setting for anyone who wants close to mean close.
    fn on_close_requested(&self, _id: cosmic::iced::window::Id) -> Option<Message> {
        // With no status icon there would be no way back to the window, so
        // hiding would strand the user with an invisible running process.
        let reachable = self.state.config.settings.show_tray_icon && self.tray.is_some();
        if self.state.config.settings.run_in_background && reachable {
            Some(Message::HideWindow)
        } else {
            Some(Message::Tray(TrayEvent::Quit))
        }
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
            crate::subscription::refresh_ticks().map(|()| Message::Tick),
            crate::subscription::tray_events().map(Message::Tray),
            crate::subscription::notification_actions()
                .map(|(id, action)| Message::NotificationAction(id, action)),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ui(action) => self.on_action(action),

            Message::Event(event) => {
                let effects = self.state.apply_event(*event);
                let reload = self.run_effects(effects);
                let tray = self.sync_tray();
                // A policy change may have added or removed a standing rule.
                Task::batch([reload, tray, self.refresh(Trigger::Manual)])
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
                if let Some(rules) = refreshed.rules.as_deref() {
                    self.state.set_policy(rules);
                }
                self.state.set_health(refreshed.health.clone());
                self.sync_tray()
            }

            Message::Decided(key, result) => {
                self.state.busy.remove(&key);
                match result {
                    Ok(()) => {
                        // Only a connected device can have had a prompt raised
                        // for it, so there is nothing to resolve otherwise.
                        let effects = match &key {
                            DeviceKey::Connected(id) => self.state.resolve_pending(*id),
                            DeviceKey::Remembered(_) => Vec::new(),
                        };
                        let reload = self.run_effects(effects);
                        Task::batch([reload, self.refresh(Trigger::Manual)])
                    }
                    Err(message) => {
                        self.state.error = Some(message);
                        Task::none()
                    }
                }
            }

            Message::Revoked(key, result) => {
                self.state.busy.remove(&key);
                match result {
                    Ok(removed) => {
                        if let Some(device) = self.state.device_by_key(&key).cloned() {
                            self.state.record_revocation(&device, removed);
                        }
                        self.refresh(Trigger::Manual)
                    }
                    Err(message) => {
                        self.state.error = Some(message);
                        Task::none()
                    }
                }
            }

            Message::Tick => self.refresh(Trigger::Automatic),

            Message::DismissError => {
                self.state.error = None;
                Task::none()
            }

            Message::TrayReady(tray) => {
                let failed = tray.is_none();
                self.tray = tray;

                if failed {
                    debug_log!(
                        crate::debug::TRAY,
                        "no status notifier host; the window is the only way in"
                    );
                    // Started hidden, expecting an icon that never appeared.
                    // Staying hidden would leave a running process with no
                    // window and nothing in the panel to bring it back.
                    if self.hidden {
                        self.state.error = Some(fl!("error-no-tray"));
                        return self.show_window();
                    }
                }
                Task::none()
            }

            Message::Tray(event) => match event {
                TrayEvent::Open => self.show_window(),
                TrayEvent::Refresh => self.refresh(Trigger::Manual),
                TrayEvent::Quit => self.quit(),
            },

            Message::NotifierReady(notifier) => {
                if notifier.is_none() {
                    debug_log!(
                        crate::debug::NOTIFY,
                        "no notification service; prompts will only appear in the window"
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
                    // A refusal notice is not tied to a pending decision, so
                    // there is nothing to look up; opening the window is still
                    // the right answer.
                    return if action == ACTION_MANAGE {
                        self.show_window()
                    } else {
                        Task::none()
                    };
                };

                let key = DeviceKey::Connected(device_id);
                match action.as_str() {
                    ACTION_ALLOW => self.on_action(Action::Decide {
                        key,
                        target: crate::usbguard::Target::Allow,
                        permanent: self.state.permanent,
                    }),
                    ACTION_BLOCK => self.on_action(Action::Decide {
                        key,
                        target: crate::usbguard::Target::Block,
                        permanent: self.state.permanent,
                    }),
                    ACTION_DETAILS | ACTION_MANAGE => {
                        self.state.selected = Some(key);
                        self.show_window()
                    }
                    _ => Task::none(),
                }
            }

            Message::ShowWindow => self.show_window(),

            Message::HideWindow => self.hide_window(),

            Message::Noop => Task::none(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_page_has_a_name_and_survives_a_round_trip() {
        // A page missing from `from_name` would make `--page` silently open
        // the wrong one, and a page missing from `ALL` would vanish from the
        // navigation entirely.
        assert_eq!(Page::ALL.len(), 4);
        for page in Page::ALL {
            let name = match page {
                Page::Devices => "devices",
                Page::History => "history",
                Page::Status => "status",
                Page::Settings => "settings",
            };
            assert_eq!(Page::from_name(name), Some(page), "{name}");
        }
    }

    #[test]
    fn an_unknown_page_name_is_ignored_rather_than_fatal() {
        // Refusing to start a security tool over a typo in an optional
        // argument would leave the user with no USB policy front-end at all.
        assert_eq!(Page::from_name("nonsense"), None);
        assert_eq!(Page::from_name(""), None);
    }
}
