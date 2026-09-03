// SPDX-License-Identifier: GPL-3.0-or-later

//! The status icon, published as a `StatusNotifierItem`.
//!
//! The app used to ship a second binary that the user had to add to the COSMIC
//! panel by hand. This replaces it: the running application registers itself
//! with whatever `StatusNotifierWatcher` owns the session — on COSMIC that is
//! `cosmic-applet-status-area`, which is in the default panel — so the icon
//! appears on its own and the same window serves the popup's old job.
//!
//! The tray runs on its own zbus connection inside `ksni`. It cannot touch
//! application state, so it reports through a channel that the UI drains as an
//! [`crate::subscription`], which keeps every state change on the iced thread.

use tokio::sync::mpsc;

use crate::debug_log;
use ksni::TrayMethods;

use crate::ui::icons;
use crate::usbguard::Severity;

/// What the user chose from the tray.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayEvent {
    /// Show the window, or raise it if it is already open.
    Open,
    /// Re-read devices and health.
    Refresh,
    /// Quit the application.
    Quit,
}

/// What the tray needs to know to draw itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayState {
    /// Whether the daemon is reachable.
    pub connected: bool,
    /// Worst health severity observed.
    pub severity: Severity,
    /// How many devices are waiting for a decision.
    pub pending: usize,
}

impl Default for TrayState {
    fn default() -> Self {
        Self {
            connected: false,
            severity: Severity::Critical,
            pending: 0,
        }
    }
}

impl TrayState {
    /// Read the tray-relevant parts out of the application state.
    pub fn from_state(state: &crate::state::State) -> Self {
        Self {
            connected: state.connected,
            severity: if state.health_checked {
                state.health.worst()
            } else {
                Severity::Warning
            },
            pending: state.pending.len(),
        }
    }

    fn icon(&self) -> &'static str {
        icons::for_severity_status(self.connected, self.severity, self.pending)
    }

    /// One line describing the current state, for the tray tooltip.
    fn tooltip(&self) -> String {
        if !self.connected {
            return crate::fl!("status-disconnected");
        }
        if self.pending > 0 {
            return crate::fl!("devices-pending", count = self.pending);
        }
        match self.severity {
            Severity::Ok => crate::fl!("status-ok"),
            Severity::Warning => crate::fl!("status-warning"),
            Severity::Critical => crate::fl!("status-critical"),
        }
    }
}

/// The process-wide channel the tray reports through.
///
/// A static because the tray is created inside an async task while the
/// subscription that drains it is created separately by the iced runtime, and
/// the two have no other way to meet. Unbounded because the sender is called
/// from `ksni`'s own thread inside a non-async callback, where it cannot wait.
fn channel() -> &'static Channel {
    static CHANNEL: std::sync::OnceLock<Channel> = std::sync::OnceLock::new();
    CHANNEL.get_or_init(|| {
        let (sender, receiver) = mpsc::unbounded_channel();
        Channel {
            sender,
            receiver: tokio::sync::Mutex::new(Some(receiver)),
        }
    })
}

struct Channel {
    sender: mpsc::UnboundedSender<TrayEvent>,
    receiver: tokio::sync::Mutex<Option<mpsc::UnboundedReceiver<TrayEvent>>>,
}

/// A sender the tray can report through.
pub fn sender() -> mpsc::UnboundedSender<TrayEvent> {
    channel().sender.clone()
}

/// Take the receiving end. Yields `None` on any call after the first.
pub async fn take_receiver() -> Option<mpsc::UnboundedReceiver<TrayEvent>> {
    channel().receiver.lock().await.take()
}

/// The `ksni` item. Holds only what it draws.
struct Indicator {
    state: TrayState,
    events: mpsc::UnboundedSender<TrayEvent>,
}

impl ksni::Tray for Indicator {
    fn id(&self) -> String {
        crate::constants::APP_ID.to_string()
    }

    fn title(&self) -> String {
        crate::fl!("app-title")
    }

    fn icon_name(&self) -> String {
        self.state.icon().to_string()
    }

    fn status(&self) -> ksni::Status {
        // A waiting decision is asking the user for something, so the icon
        // must not be collapsed into the overflow menu.
        if self.state.pending > 0 || self.state.severity != Severity::Ok {
            ksni::Status::Active
        } else {
            ksni::Status::Passive
        }
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        ksni::ToolTip {
            icon_name: self.state.icon().to_string(),
            title: crate::fl!("app-title"),
            description: self.state.tooltip(),
            icon_pixmap: Vec::new(),
        }
    }

    /// Left-click opens the window, which is where every action now lives.
    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.events.send(TrayEvent::Open);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::{MenuItem, StandardItem};

        vec![
            StandardItem {
                label: crate::fl!("open-app"),
                icon_name: icons::PANEL_OK.to_string(),
                activate: Box::new(|this: &mut Self| {
                    let _ = this.events.send(TrayEvent::Open);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: crate::fl!("refresh"),
                icon_name: icons::REFRESH.to_string(),
                activate: Box::new(|this: &mut Self| {
                    let _ = this.events.send(TrayEvent::Refresh);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: crate::fl!("quit"),
                icon_name: icons::QUIT.to_string(),
                activate: Box::new(|this: &mut Self| {
                    let _ = this.events.send(TrayEvent::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

/// A running tray icon.
///
/// Dropping this withdraws the icon, which is how the "hide the tray icon"
/// setting is honoured without restarting the app.
pub struct Tray {
    handle: ksni::Handle<Indicator>,
}

impl std::fmt::Debug for Tray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tray").finish_non_exhaustive()
    }
}

impl Tray {
    /// Publish the icon and start serving it.
    ///
    /// Returns `None` when no `StatusNotifierWatcher` owns the session bus —
    /// a desktop with no tray at all. That is not an error: the window still
    /// works, and the app says so rather than failing to start.
    pub async fn spawn(state: TrayState, events: mpsc::UnboundedSender<TrayEvent>) -> Option<Self> {
        match (Indicator { state, events }).spawn().await {
            Ok(handle) => {
                debug_log!(crate::debug::TRAY, "status icon registered");
                Some(Self { handle })
            }
            Err(e) => {
                debug_log!(crate::debug::TRAY, "no status notifier host: {e}");
                None
            }
        }
    }

    /// Redraw the icon after the application state changed.
    ///
    /// A no-op once the service has shut down, which is what happens when the
    /// user turns the icon off.
    pub async fn update(&self, state: TrayState) {
        self.handle
            .update(move |item: &mut Indicator| {
                item.state = state;
            })
            .await;
    }

    /// Withdraw the icon.
    ///
    /// The returned awaiter is dropped rather than awaited: the request is
    /// already queued by the time `shutdown` returns, and the two callers —
    /// turning the icon off, and quitting — have nothing useful to do while
    /// the withdrawal completes.
    pub fn shutdown(&self) {
        debug_log!(crate::debug::TRAY, "withdrawing status icon");
        drop(self.handle.shutdown());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ksni::Tray as _;

    fn tray(connected: bool, severity: Severity, pending: usize) -> TrayState {
        TrayState {
            connected,
            severity,
            pending,
        }
    }

    #[test]
    fn a_waiting_decision_is_never_shown_as_passive() {
        // A passive item can be folded into an overflow menu. Something asking
        // the user a question must stay visible.
        let indicator = |state| Indicator {
            state,
            events: mpsc::unbounded_channel().0,
        };
        assert_eq!(
            indicator(tray(true, Severity::Ok, 1)).status(),
            ksni::Status::Active
        );
        assert_eq!(
            indicator(tray(true, Severity::Ok, 0)).status(),
            ksni::Status::Passive
        );
        assert_eq!(
            indicator(tray(true, Severity::Warning, 0)).status(),
            ksni::Status::Active
        );
    }

    #[test]
    fn the_tooltip_never_claims_protection_while_disconnected() {
        let disconnected = tray(false, Severity::Ok, 0).tooltip();
        assert_ne!(disconnected, crate::fl!("status-ok"));
    }

    #[test]
    fn a_pending_decision_outranks_a_healthy_tooltip() {
        let waiting = tray(true, Severity::Ok, 2).tooltip();
        assert_ne!(waiting, crate::fl!("status-ok"));
        assert!(waiting.contains('2'), "{waiting}");
    }
}
