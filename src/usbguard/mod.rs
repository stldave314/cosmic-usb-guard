// SPDX-License-Identifier: GPL-3.0-or-later

//! Everything that talks to the USBGuard daemon.
//!
//! Layers, innermost first:
//!
//! - [`rule`] parses USBGuard's rule strings, which are the only form in which
//!   the daemon describes a device.
//! - [`model`] turns a parsed rule into a [`Device`] the UI can render.
//! - [`proxy`] holds the raw zbus interface definitions.
//! - [`client`] wraps those in a [`Client`] with error classification and
//!   timeouts.
//! - [`events`] owns the connection lifecycle and produces a single
//!   self-healing stream of [`Event`]s.
//! - [`health`] probes whether USBGuard is actually protecting the machine.
//!
//! No layer above `client` should need to touch zbus directly.

pub mod client;
pub mod events;
pub mod health;
pub mod model;
pub mod proxy;
pub mod rule;

pub use client::{Client, Error};
pub use events::Event;
pub use health::{Check, CheckId, Health, Severity};
pub use model::{Device, Interface, PolicyRule, PresenceEvent};
pub use rule::{Rule, Target};
