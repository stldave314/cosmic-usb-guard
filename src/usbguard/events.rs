// SPDX-License-Identifier: GPL-3.0-or-later

//! A single, self-healing stream of everything USBGuard tells us.
//!
//! The daemon can stop, be restarted, or never have been running when the UI
//! started, so this owns the whole connection lifecycle: it connects, replays
//! the current device list, forwards signals, and reconnects with back-off
//! when the service goes away. Consumers only ever see [`Event`]s.

use futures::{SinkExt, StreamExt, stream};

use crate::constants::{RECONNECT_DELAY, RECONNECT_DELAY_MAX};
use crate::debug_log;

use super::client::Client;
use super::model::{Device, PresenceEvent};
use super::rule::Target;

/// Something that happened, from the UI's point of view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Connected to the daemon; `devices` is the full current list.
    ///
    /// Emitted on first connect and after every reconnect, so the consumer can
    /// replace its state wholesale rather than trying to reconcile a gap.
    Connected {
        /// Every device the daemon currently knows about.
        devices: Vec<Device>,
    },
    /// Lost the daemon, or never reached it. The UI must stop presenting its
    /// device list as current.
    Disconnected {
        /// Human-readable reason.
        reason: String,
    },
    /// A device appeared, changed or went away.
    Presence {
        /// The device the event concerns.
        device: Device,
        /// What happened to it.
        event: PresenceEvent,
    },
    /// A device's authorisation changed, whoever changed it.
    ///
    /// This fires for decisions made in this app, in another USBGuard
    /// front-end, on the command line, or automatically by the daemon's own
    /// policy — which is what makes the journal a complete record.
    Policy {
        /// The device the decision concerns.
        device: Device,
        /// Authorisation before the change.
        old: Target,
        /// Authorisation after the change.
        new: Target,
        /// ID of the policy rule that caused it, if any.
        rule_id: u32,
    },
}

/// Internal union of the raw signal streams before they become [`Event`]s.
enum Raw {
    Presence(u32, u32, u32, String),
    Policy(u32, u32, u32, String, u32),
    /// The `org.usbguard1` bus name changed owner. `false` means it went away.
    Owner(bool),
}

/// Drive the event stream forever, sending into `tx`.
///
/// Never returns. Intended to be spawned inside an iced subscription; see
/// [`crate::subscription`].
pub async fn pump(tx: &mut futures::channel::mpsc::Sender<Event>) {
    let mut backoff = RECONNECT_DELAY;

    loop {
        match session(tx).await {
            Ok(()) => {
                // A clean end means the daemon went away; retry promptly.
                backoff = RECONNECT_DELAY;
            }
            Err(reason) => {
                debug_log!(crate::debug::DBUS, "session ended: {reason}");
                let _ = tx.send(Event::Disconnected { reason }).await;
            }
        }

        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(RECONNECT_DELAY_MAX);
    }
}

/// One connected session: connect, replay, forward until the daemon goes away.
///
/// Returns `Ok(())` when the daemon disappeared cleanly, `Err` with a reason
/// when the connection could not be established or broke unexpectedly.
async fn session(tx: &mut futures::channel::mpsc::Sender<Event>) -> Result<(), String> {
    let client = Client::connect().await.map_err(|e| e.to_string())?;

    // Subscribe to signals *before* listing devices. Doing it the other way
    // round leaves a window in which a device could be plugged in, missed by
    // the not-yet-registered signal match, and absent from the already-taken
    // snapshot.
    let devices_proxy = client.devices_proxy();
    let presence = devices_proxy
        .receive_device_presence_changed()
        .await
        .map_err(|e| format!("could not subscribe to presence signals: {e}"))?;
    let policy = devices_proxy
        .receive_device_policy_changed()
        .await
        .map_err(|e| format!("could not subscribe to policy signals: {e}"))?;

    let dbus = zbus::fdo::DBusProxy::new(client.connection())
        .await
        .map_err(|e| format!("could not reach the bus daemon: {e}"))?;
    let owner_changed = dbus
        .receive_name_owner_changed_with_args(&[(0, crate::constants::USBGUARD_BUS)])
        .await
        .map_err(|e| format!("could not watch the USBGuard bus name: {e}"))?;

    // Prove the daemon answers before claiming we are connected. Constructing
    // proxies succeeds even when nothing is listening.
    let devices = client.list_devices().await.map_err(|e| e.to_string())?;
    debug_log!(
        crate::debug::DBUS,
        "connected; replaying {} device(s)",
        devices.len()
    );
    tx.send(Event::Connected { devices })
        .await
        .map_err(|e| e.to_string())?;

    let presence = presence.filter_map(|signal| async move {
        let args = signal.args().ok()?;
        Some(Raw::Presence(
            args.id,
            args.event,
            args.target,
            args.device_rule.clone(),
        ))
    });

    let policy = policy.filter_map(|signal| async move {
        let args = signal.args().ok()?;
        Some(Raw::Policy(
            args.id,
            args.target_old,
            args.target_new,
            args.device_rule.clone(),
            args.rule_id,
        ))
    });

    let owner_changed = owner_changed.filter_map(|signal| async move {
        let args = signal.args().ok()?;
        Some(Raw::Owner(args.new_owner.is_some()))
    });

    let mut merged = stream::select(
        stream::select(presence.boxed(), policy.boxed()),
        owner_changed.boxed(),
    );

    while let Some(raw) = merged.next().await {
        let event = match raw {
            Raw::Owner(true) => {
                // The daemon restarted under us. Everything we know is stale.
                debug_log!(crate::debug::DBUS, "usbguard reappeared; resyncing");
                return Ok(());
            }
            Raw::Owner(false) => {
                debug_log!(crate::debug::DBUS, "usbguard bus name released");
                let _ = tx
                    .send(Event::Disconnected {
                        reason: "the USBGuard service stopped".into(),
                    })
                    .await;
                return Ok(());
            }
            Raw::Presence(id, event, _target, rule) => match Device::from_rule(id, &rule) {
                Ok(device) => Event::Presence {
                    device,
                    event: PresenceEvent::from_dbus(event),
                },
                Err(e) => {
                    crate::error_log!(
                        crate::debug::DEVICE,
                        "unparseable presence rule for device {id}: {e}"
                    );
                    continue;
                }
            },
            Raw::Policy(id, old, new, rule, rule_id) => match Device::from_rule(id, &rule) {
                Ok(device) => Event::Policy {
                    device,
                    old: Target::from_dbus(old),
                    new: Target::from_dbus(new),
                    rule_id,
                },
                Err(e) => {
                    crate::error_log!(
                        crate::debug::DEVICE,
                        "unparseable policy rule for device {id}: {e}"
                    );
                    continue;
                }
            },
        };

        if tx.send(event).await.is_err() {
            // The consumer dropped; nothing left to do.
            return Ok(());
        }
    }

    Err("signal stream ended".into())
}
