// SPDX-License-Identifier: GPL-3.0-or-later

//! Thunderbolt and USB4: the part of the attack surface USBGuard cannot see.
//!
//! USBGuard governs USB, including USB tunneled over USB4. PCIe tunneled over
//! Thunderbolt or USB4 — docks, external GPUs, NVMe enclosures — is authorized
//! by the kernel's Thunderbolt subsystem, with bolt deciding what to
//! authorize. A PCIe device can read memory directly, so this is the more
//! dangerous half of the port.
//!
//! Layers, innermost first:
//!
//! - [`sysfs`] reads the kernel's security level and DMA protection.
//! - [`model`] turns bolt's properties into typed devices and domains.
//! - [`proxy`] holds the raw zbus interface definition.
//! - [`client`] wraps it with error classification and timeouts.
//! - [`events`] turns bolt's signals into a self-healing change stream.
//! - [`health`] judges the configuration and says how to fix it.

pub mod client;
pub mod events;
pub mod health;
pub mod model;
pub mod proxy;
pub mod sysfs;

pub use client::Client;
pub use model::{Device, Domain, Manager, SecurityLevel, Snapshot, Status};
