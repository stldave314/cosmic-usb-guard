// SPDX-License-Identifier: GPL-3.0-or-later

//! iced subscriptions wrapping the async plumbing.
//!
//! Each one is a long-lived stream the runtime keeps alive for as long as the
//! application asks for it, so reconnection and back-off live inside the
//! stream rather than in the application's update loop.

use cosmic::iced::{Subscription, stream};

use crate::constants::REFRESH_INTERVAL;
use crate::tray::TrayEvent;
use crate::usbguard::Event;

/// A self-healing stream of USBGuard events.
///
/// Emits [`Event::Connected`] with a full device list on every (re)connect, so
/// a consumer can replace its state wholesale and never has to reason about
/// what it might have missed while the daemon was away.
pub fn usbguard_events() -> Subscription<Event> {
    Subscription::run(|| {
        stream::channel(64, |mut output| async move {
            crate::usbguard::events::pump(&mut output).await;
        })
    })
}

/// Ticks on which to re-read devices, policy and health.
///
/// Neither systemd unit state nor daemon parameters signal a change, so they
/// have to be polled; the device list rides along as a safety net for a
/// missed signal.
pub fn refresh_ticks() -> Subscription<()> {
    cosmic::iced::time::every(REFRESH_INTERVAL).map(|_| ())
}

/// Actions the user activated on one of our desktop notifications.
///
/// Yields `(notification_id, action_key)`; see [`crate::notify`] for the keys.
pub fn notification_actions() -> Subscription<(u32, String)> {
    Subscription::run(|| {
        stream::channel(
            16,
            |mut output: futures::channel::mpsc::Sender<(u32, String)>| async move {
                use futures::{SinkExt, StreamExt};

                loop {
                    let stream = async {
                        let notifier = crate::notify::Notifier::connect().await?;
                        notifier.actions().await
                    }
                    .await;

                    if let Some(mut stream) = stream {
                        while let Some(action) = stream.next().await {
                            if output.send(action).await.is_err() {
                                return;
                            }
                        }
                    }

                    // No notification daemon, or it went away. Retry rather than
                    // ending the subscription, since one can appear later in the
                    // session.
                    tokio::time::sleep(crate::constants::RECONNECT_DELAY).await;
                }
            },
        )
    })
}

/// Choices the user made in the status icon.
///
/// The tray runs on its own connection inside `ksni` and cannot touch
/// application state, so it reports here and the update loop applies the
/// change like any other message.
pub fn tray_events() -> Subscription<TrayEvent> {
    Subscription::run(|| {
        stream::channel(
            8,
            |mut output: futures::channel::mpsc::Sender<TrayEvent>| async move {
                use futures::SinkExt;

                let Some(mut receiver) = crate::tray::take_receiver().await else {
                    // Already taken: the runtime restarted this subscription.
                    // Ending is correct — a second drain would steal events
                    // from the first.
                    std::future::pending::<()>().await;
                    return;
                };

                while let Some(event) = receiver.recv().await {
                    if output.send(event).await.is_err() {
                        return;
                    }
                }
            },
        )
    })
}
