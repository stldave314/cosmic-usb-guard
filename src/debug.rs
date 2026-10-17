// SPDX-License-Identifier: GPL-3.0-or-later

//! Developer debug logging.
//!
//! Flip [`DEVELOPER_LOGGING`] to `true` while debugging and rebuild. Output
//! goes to [`PATH`], truncated once per process launch, with each line
//! prefixed by elapsed time and a short category tag so a run can be filtered
//! with `grep`.
//!
//! Logging goes to a *file* rather than stderr on purpose: a panel applet's
//! stderr is piped to `cosmic-panel` and is effectively unreadable.
//!
//! Genuine errors still go to stderr via [`crate::debug::error`]; this module
//! is for diagnostics, not a replacement for real error reporting.

use std::fmt::Arguments;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

/// Master switch. Set to `true` to turn debug logging on for a dev build.
const DEVELOPER_LOGGING: bool = false;

/// Effective switch. The `release-build` feature forces logging off at compile
/// time, so a packaged build can never ship with it on by accident — every
/// packaging target passes `--features release-build`.
pub const ENABLED: bool = DEVELOPER_LOGGING && !cfg!(feature = "release-build");

/// Log file location.
pub const PATH: &str = "/tmp/cosmic-usb-guard-debug.log";

// Short category tags.
/// D-Bus connection and method calls.
pub const DBUS: &str = "DBUS";
/// Device presence and policy events.
pub const DEVICE: &str = "DEVICE";
/// Policy rule changes.
pub const POLICY: &str = "POLICY";
/// Installation health checks.
pub const HEALTH: &str = "HEALTH";
/// User interface state.
pub const UI: &str = "UI";
/// Settings load and save.
pub const CONFIG: &str = "CONFIG";
/// Decision journal writes.
pub const JOURNAL: &str = "JOURNAL";
/// Desktop notifications.
pub const NOTIFY: &str = "NOTIFY";
/// Status icon lifecycle.
pub const TRAY: &str = "TRAY";
/// Per-device hook programs.
pub const HOOK: &str = "HOOK";
/// Thunderbolt/USB4 and bolt.
pub const THUNDERBOLT: &str = "TBOLT";

struct Sink {
    file: Option<File>,
    start: Instant,
}

static SINK: OnceLock<Mutex<Sink>> = OnceLock::new();

fn sink() -> &'static Mutex<Sink> {
    SINK.get_or_init(|| {
        Mutex::new(Sink {
            file: open_log(std::path::Path::new(PATH)),
            start: Instant::now(),
        })
    })
}

/// Open the log, truncated once per process launch.
///
/// The path is fixed and under `/tmp`, where any local user can create a
/// name first. So the file is opened without following a symlink — one would
/// otherwise point the truncation at any file this user can write — created
/// owner-only, and dropped if it turns out to belong to someone else, who
/// would then be able to read what was logged.
fn open_log(path: &std::path::Path) -> Option<File> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

    // Not truncated on open: only once it is known to be ours.
    #[allow(clippy::suspicious_open_options)]
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .ok()?;
    // SAFETY: `getuid` has no preconditions and cannot fail.
    let uid = unsafe { libc::getuid() };
    if file.metadata().ok()?.uid() != uid {
        return None;
    }
    file.set_len(0).ok()?;
    Some(file)
}

/// Write one already-formatted line. Called by [`debug_log!`]; not intended to
/// be used directly.
pub fn write(category: &str, args: Arguments<'_>) {
    if !ENABLED {
        return;
    }
    let Ok(mut sink) = sink().lock() else {
        return;
    };
    let elapsed = sink.start.elapsed().as_secs_f64();
    if let Some(file) = sink.file.as_mut() {
        let _ = writeln!(file, "[{elapsed:9.3}] {category:<7} {args}");
        let _ = file.flush();
    }
}

/// Emit a line to the debug log.
///
/// Expands to `if ENABLED { .. }`, so the optimizer removes it when logging is
/// off — but the arguments are still type-checked, which stops disabled call
/// sites from silently rotting.
#[macro_export]
macro_rules! debug_log {
    ($category:expr, $($arg:tt)*) => {
        if $crate::debug::ENABLED {
            $crate::debug::write($category, format_args!($($arg)*));
        }
    };
}

/// Report a genuine error: always to stderr, and to the debug log as well.
///
/// Unlike [`debug_log!`] this is never compiled out.
pub fn error(category: &str, args: Arguments<'_>) {
    eprintln!("cosmic-usb-guard: {category}: {args}");
    write(category, args);
}

/// Report a genuine error. See [`error`].
#[macro_export]
macro_rules! error_log {
    ($category:expr, $($arg:tt)*) => {
        $crate::debug::error($category, format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cosmic-usb-guard-debuglog-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_log_is_not_opened_through_a_symlink() {
        let dir = scratch("link");
        let target = dir.join("victim");
        std::fs::write(&target, "keep me").unwrap();
        let link = dir.join("log");
        std::os::unix::fs::symlink(&target, &link).unwrap();

        assert!(open_log(&link).is_none(), "followed a symlink");
        // And the file it pointed at was not truncated.
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "keep me");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_new_log_is_owner_only_and_an_old_one_is_truncated() {
        let dir = scratch("mode");
        let path = dir.join("log");
        std::fs::write(&path, "previous run").unwrap();

        let mut file = open_log(&path).expect("our own file");
        file.write_all(b"x").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "x");

        std::fs::remove_file(&path).unwrap();
        open_log(&path).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
