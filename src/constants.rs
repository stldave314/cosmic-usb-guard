// SPDX-License-Identifier: GPL-3.0-or-later

//! Centralised compile-time tuning values.
//!
//! These are *implementation* constants, deliberately not exposed through a
//! runtime config file. User-facing settings live in [`crate::config`], which
//! is backed by `cosmic-config`. Keeping tuning values here means there is one
//! place to change them and no second config mechanism to parse, version, or
//! fail to load.

use std::time::Duration;

/// Reverse-DNS application ID for the main window application.
pub const APP_ID: &str = "io.github.stldave314.CosmicUsbGuard";

/// Reverse-DNS application ID for the panel applet.
pub const APPLET_ID: &str = "io.github.stldave314.CosmicUsbGuardApplet";

/// Package metadata, derived from `Cargo.toml` so it cannot drift.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Upstream repository URL, derived from `Cargo.toml`.
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
/// Crate name, derived from `Cargo.toml`.
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Issue tracker, derived from the repository URL.
pub fn issues_url() -> String {
    format!("{REPOSITORY}/issues")
}

// ---------------------------------------------------------------------------
// D-Bus
// ---------------------------------------------------------------------------

/// Well-known bus name owned by `usbguard-dbus`.
///
/// Object paths and interface names are *not* mirrored here. `zbus::proxy`
/// requires them as literals in its attributes, so a copy in this file could
/// only ever be a second, silently divergent source of truth; see
/// [`crate::usbguard::proxy`]. This one is here because the bus name is also
/// needed for the `NameOwnerChanged` watch, which is not part of a proxy.
pub const USBGUARD_BUS: &str = "org.usbguard1";

/// How long to wait for a single USBGuard D-Bus method call before giving up.
///
/// Calls are Polkit-mediated; when an authentication dialog is shown the reply
/// is blocked until the user answers it, so this has to be generous.
pub const DBUS_CALL_TIMEOUT: Duration = Duration::from_secs(120);

/// Delay before retrying a failed connection to the USBGuard D-Bus service.
pub const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// Upper bound on the reconnect back-off.
pub const RECONNECT_DELAY_MAX: Duration = Duration::from_secs(60);

// ---------------------------------------------------------------------------
// Polling and refresh
// ---------------------------------------------------------------------------

/// Interval at which devices, policy and health are re-read.
///
/// Health depends on systemd unit state and daemon parameters, neither of
/// which emits a signal we can subscribe to, so it has to be polled. The
/// device list is picked up separately from D-Bus signals; re-reading it on
/// the same timer is a safety net for a signal missed across a daemon restart.
///
/// One timer rather than two: the refresh reads everything at once, so a
/// second, faster timer would only duplicate the same Polkit-mediated calls.
pub const REFRESH_INTERVAL: Duration = Duration::from_secs(20);

// ---------------------------------------------------------------------------
// Event journal
// ---------------------------------------------------------------------------

/// File name of the append-only decision journal, inside the app data dir.
pub const JOURNAL_FILE: &str = "events.jsonl";

/// Maximum journal size before it is rotated to `events.jsonl.1`.
pub const JOURNAL_MAX_BYTES: u64 = 4 * 1024 * 1024;

/// Number of journal entries loaded into memory for the history view.
pub const JOURNAL_VIEW_LIMIT: usize = 1000;

// ---------------------------------------------------------------------------
// Notifications
// ---------------------------------------------------------------------------

/// Notification timeout, in milliseconds, for the "new device" prompt.
///
/// `0` means "never expire"; a decision prompt should not vanish on its own.
pub const NOTIFY_PROMPT_TIMEOUT_MS: i32 = 0;

/// Notification timeout for informational messages, in milliseconds.
pub const NOTIFY_INFO_TIMEOUT_MS: i32 = 8_000;

// ---------------------------------------------------------------------------
// UI layout
// ---------------------------------------------------------------------------

/// Maximum height of the applet popup, in logical pixels.
pub const POPUP_MAX_HEIGHT: f32 = 700.0;
/// Minimum width of the applet popup, in logical pixels.
pub const POPUP_MIN_WIDTH: f32 = 400.0;
/// Maximum width of the applet popup, in logical pixels.
pub const POPUP_MAX_WIDTH: f32 = 520.0;

/// Default size of the main window, in logical pixels.
pub const WINDOW_DEFAULT_SIZE: (f32, f32) = (1000.0, 720.0);
/// Minimum size of the main window, in logical pixels.
pub const WINDOW_MIN_SIZE: (f32, f32) = (620.0, 480.0);

/// Number of pending prompts shown inline in the applet popup before the rest
/// are collapsed behind a "show all" affordance.
pub const APPLET_MAX_INLINE_PROMPTS: usize = 3;

/// Character budget for a device description before it is elided in the UI.
pub const DESCRIPTION_ELIDE_CHARS: usize = 64;

// ---------------------------------------------------------------------------
// USBGuard daemon expectations
// ---------------------------------------------------------------------------

// The daemon's configuration file is deliberately not referenced. It is
// root-readable only, and reading it would answer "what was configured"
// rather than "what is in force" — which are different whenever the file has
// been edited without a reload. Every check in `usbguard::health` asks the
// running daemon instead.

/// systemd unit that must be running for any of this to work.
pub const UNIT_DAEMON: &str = "usbguard.service";

/// systemd unit that exposes the D-Bus interface we talk to.
pub const UNIT_DBUS: &str = "usbguard-dbus.service";

/// Runtime parameter deciding what happens to a newly inserted device.
pub const PARAM_INSERTED_DEVICE_POLICY: &str = "InsertedDevicePolicy";

/// Value of [`PARAM_INSERTED_DEVICE_POLICY`] required for interactive
/// prompting to be meaningful: the device is blocked until a human decides.
pub const INSERTED_POLICY_PREFERRED: &str = "apply-policy";

/// Values of [`PARAM_INSERTED_DEVICE_POLICY`] that defeat interactive
/// prompting entirely, because the device is authorised before we can ask.
pub const INSERTED_POLICY_UNSAFE: &[&str] = &["allow", "keep"];
