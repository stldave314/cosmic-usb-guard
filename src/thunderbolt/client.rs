// SPDX-License-Identifier: GPL-3.0-or-later

//! High-level client for bolt.

use std::collections::HashMap;

use zbus::names::InterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

use crate::constants::{BOLT_BUS, BOLT_ENROLL_POLICY};
use crate::debug_log;
use crate::usbguard::Error;
use crate::usbguard::client::{call_interactive, classify, with_timeout};

use super::model::{Device, Domain, Manager, Snapshot};
use super::proxy::ManagerProxy;

const MANAGER_INTERFACE: &str = "org.freedesktop.bolt1.Manager";
const DEVICE_INTERFACE: &str = "org.freedesktop.bolt1.Device";
const DOMAIN_INTERFACE: &str = "org.freedesktop.bolt1.Domain";

/// A connection to bolt.
///
/// Cheap to clone: the underlying zbus connection is reference-counted.
#[derive(Clone)]
pub struct Client {
    connection: zbus::Connection,
    manager: ManagerProxy<'static>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

impl Client {
    /// Connect to the system bus and build the manager proxy.
    ///
    /// Like any zbus proxy this does not prove bolt is there; the first call
    /// does, and starts bolt through D-Bus activation if it is installed but
    /// not running.
    pub async fn connect() -> Result<Self, Error> {
        let connection = zbus::Connection::system().await.map_err(classify)?;
        Self::with_connection(connection).await
    }

    /// Build a client on an existing system bus connection.
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let manager = ManagerProxy::new(&connection).await.map_err(classify)?;
        Ok(Self {
            connection,
            manager,
        })
    }

    /// The underlying connection, for signal subscriptions.
    pub fn connection(&self) -> &zbus::Connection {
        &self.connection
    }

    /// Every property of one bolt object, in one round trip.
    async fn properties(
        &self,
        path: &str,
        interface: &'static str,
    ) -> Result<HashMap<String, OwnedValue>, Error> {
        let proxy = zbus::fdo::PropertiesProxy::builder(&self.connection)
            .destination(BOLT_BUS)
            .map_err(classify)?
            .path(path.to_string())
            .map_err(classify)?
            .build()
            .await
            .map_err(classify)?;
        let interface = InterfaceName::from_static_str_unchecked(interface);
        with_timeout(async { proxy.get_all(interface).await.map_err(zbus::Error::from) }).await
    }

    /// Read bolt's settings, domains and devices.
    ///
    /// An object that fails to read is skipped rather than failing the whole
    /// snapshot: a device can be unplugged between being listed and being
    /// read, and one missing row is better than an empty page.
    pub async fn snapshot(&self) -> Result<Snapshot, Error> {
        let manager = Manager::from_properties(
            &self
                .properties("/org/freedesktop/bolt", MANAGER_INTERFACE)
                .await?,
        );

        let mut domains = Vec::new();
        for path in with_timeout(self.manager.list_domains()).await? {
            if let Ok(properties) = self.properties(path.as_str(), DOMAIN_INTERFACE).await
                && let Some(domain) = Domain::from_properties(&properties)
            {
                domains.push(domain);
            }
        }

        let mut devices = Vec::new();
        for path in with_timeout(self.manager.list_devices()).await? {
            match self.properties(path.as_str(), DEVICE_INTERFACE).await {
                Ok(properties) => {
                    if let Some(device) = Device::from_properties(&properties) {
                        devices.push(device);
                    }
                }
                Err(e) => debug_log!(crate::debug::THUNDERBOLT, "skipping {path}: {e}"),
            }
        }

        debug_log!(
            crate::debug::THUNDERBOLT,
            "bolt: {} domain(s), {} device(s), auth mode {}",
            domains.len(),
            devices.len(),
            manager.auth_mode
        );

        Ok(Snapshot {
            manager: Some(manager),
            domains,
            devices,
            error: None,
        })
    }

    /// Authorize a device for this connection only.
    pub async fn authorize(&self, uid: &str) -> Result<(), Error> {
        debug_log!(crate::debug::THUNDERBOLT, "authorize {uid}");
        let path: OwnedObjectPath = with_timeout(self.manager.device_by_uid(uid)).await?;
        let device = zbus::Proxy::new(
            &self.connection,
            BOLT_BUS,
            path.to_string(),
            DEVICE_INTERFACE,
        )
        .await
        .map_err(classify)?;
        call_interactive::<_, ()>(&device, "Authorize", &("",)).await
    }

    /// Authorize a device and have bolt remember it.
    ///
    /// Enrolled with bolt's default policy, so the system's own configuration
    /// decides whether it is authorized automatically next time.
    pub async fn enroll(&self, uid: &str) -> Result<(), Error> {
        debug_log!(crate::debug::THUNDERBOLT, "enroll {uid}");
        let _: OwnedObjectPath = call_interactive(
            self.manager.inner(),
            "EnrollDevice",
            &(uid, BOLT_ENROLL_POLICY, ""),
        )
        .await?;
        Ok(())
    }

    /// Have bolt forget a device.
    ///
    /// A device that is authorized right now stays authorized until it is
    /// unplugged: the kernel offers no way to take PCIe access back from a
    /// connected Thunderbolt device.
    pub async fn forget(&self, uid: &str) -> Result<(), Error> {
        debug_log!(crate::debug::THUNDERBOLT, "forget {uid}");
        call_interactive::<_, ()>(self.manager.inner(), "ForgetDevice", &(uid,)).await
    }
}
