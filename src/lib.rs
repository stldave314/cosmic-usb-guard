// SPDX-License-Identifier: GPL-3.0-or-later
//
// cosmic-usb-guard — a COSMIC front-end and panel indicator for USBGuard.
// Copyright (C) 2026 the cosmic-usb-guard contributors.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along
// with this program. If not, see <https://www.gnu.org/licenses/>.

//! Shared core for the `cosmic-usb-guard` window application and its panel
//! applet.
//!
//! This crate does not enforce USB policy — USBGuard does. Everything here is
//! a front-end: it observes what the daemon reports, presents it, relays the
//! user's decisions back, and records what happened.

#![warn(missing_docs)]

pub mod app;
pub mod applet;
pub mod config;
pub mod constants;
pub mod debug;
pub mod i18n;
pub mod journal;
pub mod notify;
pub mod state;
pub mod subscription;
pub mod tasks;
pub mod ui;
pub mod usbguard;

/// One-time process setup shared by both binaries.
///
/// Installs a tracing subscriber, selects a locale, and notes the build in the
/// debug log so a log file identifies which version produced it.
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    i18n::init();

    debug_log!(
        debug::CONFIG,
        "{} {} starting",
        constants::PKG_NAME,
        constants::VERSION
    );
}
