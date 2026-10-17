// SPDX-License-Identifier: GPL-3.0-or-later

//! A self-healing stream of "bolt's state changed" notifications.
//!
//! bolt announces new and departed devices with `DeviceAdded`/`DeviceRemoved`
//! and everything else — a device moving from connected to authorized, an
//! enrollment, a forget — as `PropertiesChanged` on the device object. The
//! stream does not try to interpret them. Each burst becomes one tick, and the
//! consumer re-reads the whole snapshot, which is a handful of round trips and
//! cannot drift from what bolt actually holds.

use futures::{SinkExt, StreamExt};

use crate::constants::{BOLT_BUS, RECONNECT_DELAY, RECONNECT_DELAY_MAX, THUNDERBOLT_EVENT_SETTLE};
use crate::debug_log;

/// Forward a tick for every burst of bolt signals, reconnecting as needed.
///
/// Never returns while the receiver is alive.
pub async fn pump(tx: &mut futures::channel::mpsc::Sender<()>) {
    let mut backoff = RECONNECT_DELAY;

    loop {
        let started = std::time::Instant::now();
        match session(tx).await {
            Ok(()) => return,
            Err(reason) => {
                debug_log!(crate::debug::THUNDERBOLT, "signal session ended: {reason}");
            }
        }
        // A session that ran for a while was a working connection that later
        // broke, not a failure to connect, so the back-off starts over.
        if started.elapsed() > RECONNECT_DELAY_MAX {
            backoff = RECONNECT_DELAY;
        }
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(RECONNECT_DELAY_MAX);
    }
}

/// One connection's worth of signals.
///
/// Returns `Ok(())` only when the receiver has gone, which ends the pump.
async fn session(tx: &mut futures::channel::mpsc::Sender<()>) -> Result<(), String> {
    let connection = zbus::Connection::system()
        .await
        .map_err(|e| e.to_string())?;

    // Every signal bolt sends from any of its objects. The sender is the
    // well-known name, which the bus resolves to whoever owns it at delivery,
    // so this keeps working across bolt restarting.
    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .sender(BOLT_BUS)
        .and_then(|b| b.path_namespace("/org/freedesktop/bolt"))
        .map_err(|e| e.to_string())?
        .build();
    let mut stream = zbus::MessageStream::for_match_rule(rule, &connection, Some(64))
        .await
        .map_err(|e| e.to_string())?;

    debug_log!(crate::debug::THUNDERBOLT, "subscribed to bolt signals");

    while let Some(message) = stream.next().await {
        if message.is_err() {
            continue;
        }

        // Let the rest of the burst arrive, then drop it: one re-read covers
        // all of it.
        tokio::time::sleep(THUNDERBOLT_EVENT_SETTLE).await;
        while let Some(Some(_)) = futures::FutureExt::now_or_never(stream.next()) {}

        if tx.send(()).await.is_err() {
            return Ok(());
        }
    }

    Err("bolt signal stream ended".into())
}
