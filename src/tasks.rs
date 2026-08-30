// SPDX-License-Identifier: GPL-3.0-or-later

//! Async work both binaries need: applying decisions and refreshing state.
//!
//! The [`Client`] is cached across calls so a decision does not pay for a new
//! D-Bus connection, and is dropped whenever the daemon turns out to be gone
//! so the next call reconnects rather than reusing a dead handle.

use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::debug_log;
use crate::state;
use crate::usbguard::{Client, Device, Error, Health, Target, health};

/// The shared client, created on first use.
fn cache() -> &'static Mutex<Option<Client>> {
    static CACHE: std::sync::OnceLock<Mutex<Option<Client>>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Get the cached client, connecting if necessary.
async fn client() -> Result<Client, Error> {
    let mut guard = cache().lock().await;
    if let Some(client) = guard.as_ref() {
        return Ok(client.clone());
    }
    let client = Client::connect().await?;
    *guard = Some(client.clone());
    Ok(client)
}

/// Drop the cached client so the next call reconnects.
async fn invalidate() {
    *cache().lock().await = None;
    debug_log!(crate::debug::DBUS, "cached client invalidated");
}

/// Run `f` with the client, dropping the cache if the service turns out to be
/// unavailable.
async fn with_client<T, F, Fut>(f: F) -> Result<T, Error>
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let client = match client().await {
        Ok(client) => client,
        Err(e) => {
            invalidate().await;
            return Err(e);
        }
    };

    match f(client).await {
        Err(e) if matches!(e, Error::ServiceUnavailable(_)) => {
            invalidate().await;
            Err(e)
        }
        other => other,
    }
}

/// Everything a refresh gathers, in one round trip's worth of calls.
#[derive(Debug, Clone)]
pub struct Refreshed {
    /// The device list, or the error that prevented reading it.
    pub devices: Result<Vec<Device>, String>,
    /// The health report. Always present: health checks describe failure as
    /// well as success, so there is nothing to fail into.
    pub health: Health,
    /// Hashes with standing policy rules, when the policy could be read.
    ///
    /// `None` means "not known", which the state treats as "ask anyway"
    /// rather than "no rules exist".
    pub known_hashes: Option<HashSet<String>>,
}

/// Re-read devices, policy and health.
pub async fn refresh() -> Arc<Refreshed> {
    let connected = client().await.ok();
    let connection = connected.as_ref().map(|c| c.connection().clone());

    let health = health::evaluate(connected.as_ref(), connection.as_ref()).await;

    let devices = match connected.as_ref() {
        Some(client) => client.list_devices().await.map_err(|e| e.to_string()),
        None => Err(crate::fl!("error-service-unavailable")),
    };

    let known_hashes = match connected.as_ref() {
        Some(client) => client
            .list_rules()
            .await
            .ok()
            .map(|rules| state::hashes_from_rules(&rules)),
        None => None,
    };

    if devices.is_err() {
        invalidate().await;
    }

    Arc::new(Refreshed {
        devices,
        health,
        known_hashes,
    })
}

/// Apply a decision to a device.
///
/// Returns the device ID so the caller can clear its busy marker without
/// threading it through the message type.
pub async fn decide(device_id: u32, target: Target, permanent: bool) -> (u32, Result<(), String>) {
    let result = with_client(|client| async move {
        client.apply_policy(device_id, target, permanent).await?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string());

    (device_id, result)
}

/// Revoke a device: remove its standing rules, then block it now.
///
/// Both halves matter. Removing the rule alone leaves the device authorised
/// until it is unplugged; blocking alone leaves a rule that re-authorises it
/// on the next replug. The rule is removed first so that, if the block fails,
/// the device is not left with a stale allow rule.
pub async fn revoke(device_id: u32, hash: String) -> (u32, Result<usize, String>) {
    let result = with_client(|client| async move {
        let removed = client.remove_rules_for_hash(&hash).await?;
        client.apply_policy(device_id, Target::Block, false).await?;
        Ok(removed)
    })
    .await
    .map_err(|e| e.to_string());

    (device_id, result)
}

/// Read a daemon runtime parameter.
pub async fn get_parameter(name: String) -> Result<String, String> {
    with_client(|client| async move { client.get_parameter(&name).await })
        .await
        .map_err(|e| e.to_string())
}
