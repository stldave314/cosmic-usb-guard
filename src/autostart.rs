// SPDX-License-Identifier: GPL-3.0-or-later

//! Starting the app with the session.
//!
//! Implemented as a freedesktop autostart entry — a `.desktop` file in
//! `$XDG_CONFIG_HOME/autostart` — rather than a systemd user unit, because
//! that is what every desktop environment reads, including COSMIC, and it does
//! not need `systemctl --user` to be reachable.
//!
//! The entry is written by the app rather than shipped by the package. Being a
//! per-user file in the user's own config directory, it can be turned on and
//! off without root, and uninstalling the package leaves an entry pointing at a
//! missing binary — which desktops ignore — rather than silently starting
//! something the user did not install.

use std::io::Write;
use std::path::PathBuf;

use crate::constants::{APP_ID, PKG_NAME};
use crate::debug_log;

/// Command-line flag that starts the app without showing its window.
pub const FLAG_MINIMIZED: &str = "--minimized";

/// Directory holding per-user autostart entries.
fn autostart_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autostart")
}

/// Path of this app's autostart entry.
pub fn path() -> PathBuf {
    autostart_dir().join(format!("{APP_ID}.desktop"))
}

/// Whether the app is set to start with the session.
pub fn is_enabled() -> bool {
    path().is_file()
}

/// The command the autostart entry runs.
///
/// Uses the running executable's own path so a build run from a source tree
/// autostarts itself rather than a differently-versioned system copy that may
/// not even be installed. Falls back to the packaged name if the path cannot
/// be determined.
fn exec_command(minimized: bool) -> String {
    let program = std::env::current_exe()
        .ok()
        .filter(|p| p.is_absolute())
        .map(|p| p.display().to_string())
        // A newline would end the `Exec=` line and turn whatever follows into
        // a bogus key, so a path containing one is not usable at all.
        .filter(|p| !p.contains(['\n', '\r']))
        .unwrap_or_else(|| PKG_NAME.to_string());

    let program = quote_exec(&program);

    if minimized {
        format!("{program} {FLAG_MINIMIZED}")
    } else {
        program
    }
}

/// Quote an argument for a desktop entry's `Exec` key.
///
/// The Desktop Entry Specification splits `Exec` on whitespace, so a build
/// living under a path with a space in it — `~/My Projects/…`, a flatpak-style
/// directory, anything — would otherwise autostart a truncated command that
/// silently fails at login. Reserved characters are escaped inside the quotes
/// as the specification requires.
fn quote_exec(argument: &str) -> String {
    const RESERVED: &[char] = &[
        ' ', '\t', '"', '\'', '\\', '>', '<', '~', '|', '&', ';', '$', '*', '?', '#', '(', ')', '`',
    ];

    if !argument.contains(RESERVED) {
        return argument.to_string();
    }

    let mut quoted = String::with_capacity(argument.len() + 2);
    quoted.push('"');
    for character in argument.chars() {
        if matches!(character, '"' | '`' | '$' | '\\') {
            quoted.push('\\');
        }
        quoted.push(character);
    }
    quoted.push('"');
    quoted
}

/// Write or remove the autostart entry.
///
/// `minimized` controls whether the session start shows the window or only the
/// status icon; it is ignored when `enabled` is false.
pub fn set(enabled: bool, minimized: bool) -> std::io::Result<()> {
    let path = path();

    if !enabled {
        match std::fs::remove_file(&path) {
            Ok(()) => debug_log!(crate::debug::CONFIG, "removed autostart entry"),
            // Already absent is the desired state, not a failure.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        return Ok(());
    }

    std::fs::create_dir_all(path.parent().unwrap_or(&path))?;

    let entry = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={name}\n\
         Comment={comment}\n\
         Exec={exec}\n\
         Icon={APP_ID}\n\
         Terminal=false\n\
         Categories=System;Security;\n\
         X-GNOME-Autostart-enabled=true\n",
        name = crate::fl!("app-title"),
        comment = crate::fl!("app-description"),
        exec = exec_command(minimized),
    );

    // Written whole and renamed over, so a crash mid-write cannot leave a
    // half-parsed entry that the desktop refuses at the next login.
    let temporary = path.with_extension("desktop.tmp");
    {
        let mut file = std::fs::File::create(&temporary)?;
        file.write_all(entry.as_bytes())?;
        file.sync_all()?;
    }
    std::fs::rename(&temporary, &path)?;

    debug_log!(
        crate::debug::CONFIG,
        "wrote autostart entry (minimized={minimized})"
    );
    Ok(())
}

/// Whether this process was asked to start without its window.
pub fn started_minimized() -> bool {
    std::env::args().skip(1).any(|arg| arg == FLAG_MINIMIZED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_entry_lands_under_the_users_config_directory() {
        let path = path();
        assert!(path.is_absolute() || path.starts_with("."), "{path:?}");
        assert!(path.to_string_lossy().contains("autostart"));
        assert!(path.extension().is_some_and(|e| e == "desktop"));
    }

    #[test]
    fn the_minimized_flag_is_only_added_when_asked() {
        assert!(exec_command(true).ends_with(FLAG_MINIMIZED));
        assert!(!exec_command(false).contains(FLAG_MINIMIZED));
    }

    #[test]
    fn a_path_with_a_space_is_quoted_rather_than_silently_truncated() {
        // The Desktop Entry Specification splits Exec on whitespace, so an
        // unquoted `/home/me/My Projects/app` autostarts `/home/me/My` and
        // fails at login with nothing to explain it.
        assert_eq!(quote_exec("/usr/bin/app"), "/usr/bin/app");
        assert_eq!(
            quote_exec("/home/me/My Projects/app"),
            "\"/home/me/My Projects/app\""
        );
        // Reserved characters are escaped inside the quotes.
        assert_eq!(quote_exec("/tmp/a$b"), "\"/tmp/a\\$b\"");
        assert_eq!(quote_exec("/tmp/a\"b"), "\"/tmp/a\\\"b\"");
    }

    #[test]
    fn the_exec_line_is_a_single_absolute_path_when_one_is_known() {
        // A relative Exec would be resolved against the session's working
        // directory at login, which is not something the user chose.
        let command = exec_command(false);
        assert!(command.starts_with('/') || command == PKG_NAME, "{command}");
        assert!(!command.contains('\n'), "a newline would break the entry");
    }

    #[test]
    fn writing_then_removing_leaves_nothing_behind() {
        // Uses a scratch HOME so the developer's real autostart directory is
        // never touched by the test suite.
        let scratch = std::env::temp_dir().join(format!(
            "cosmic-usb-guard-autostart-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&scratch);
        std::fs::create_dir_all(&scratch).unwrap();

        // SAFETY: single-threaded within this test, and the value is restored
        // before it returns.
        let previous = std::env::var_os("XDG_CONFIG_HOME");
        unsafe { std::env::set_var("XDG_CONFIG_HOME", &scratch) };

        crate::i18n::init();
        assert!(!is_enabled());

        set(true, true).unwrap();
        assert!(is_enabled(), "entry should exist after enabling");
        let body = std::fs::read_to_string(path()).unwrap();
        assert!(body.starts_with("[Desktop Entry]"));
        assert!(body.contains(FLAG_MINIMIZED));
        // No stray temporary file left from the atomic write.
        assert!(!path().with_extension("desktop.tmp").exists());

        set(false, false).unwrap();
        assert!(!is_enabled(), "entry should be gone after disabling");
        // Removing an absent entry is not an error.
        set(false, false).unwrap();

        match previous {
            Some(value) => unsafe { std::env::set_var("XDG_CONFIG_HOME", value) },
            None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
        }
        let _ = std::fs::remove_dir_all(&scratch);
    }
}
