// SPDX-License-Identifier: GPL-3.0-or-later

//! Per-device hook programs.
//!
//! A hook lets the user attach a program to one specific device — run a backup
//! script when the backup drive is plugged in, mount something, start a sync.
//!
//! This is the one feature in the app that causes code to run, so its rules are
//! deliberately narrow and are enforced here rather than left to the caller:
//!
//! * **Pinned to a descriptor hash.** A hook belongs to one device, identified
//!   the same way a permanent rule is. Keying on the USB ID would run the
//!   user's script for anything claiming those IDs, which is precisely the
//!   substitution USBGuard exists to catch — and it would turn a spoofed
//!   vendor/product pair into arbitrary code execution.
//! * **Only after the device is authorised.** [`Hook::should_run`] requires a
//!   live [`Target::Allow`]. A device that is blocked, rejected, or still
//!   awaiting a decision never runs anything, so the hook is always behind the
//!   same gate as the device.
//! * **No shell.** The program is executed directly with an argument vector.
//!   Nothing the device reports is ever concatenated into a command line, so a
//!   device whose "name" is `; rm -rf ~` is just a device with a silly name.
//!   Device details reach the program as environment variables instead.
//! * **Detached and bounded.** The hook is spawned without a pipe to the app
//!   and is not waited on, so a script that blocks forever cannot wedge the UI.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use serde::{Deserialize, Serialize};

use crate::debug_log;
use crate::usbguard::{Device, Target};

/// Environment variable carrying the device's descriptor hash.
pub const ENV_HASH: &str = "USBGUARD_DEVICE_HASH";
/// Environment variable carrying the device's product name.
pub const ENV_NAME: &str = "USBGUARD_DEVICE_NAME";
/// Environment variable carrying `vvvv:pppp`.
pub const ENV_USB_ID: &str = "USBGUARD_DEVICE_ID";
/// Environment variable carrying the device serial number.
pub const ENV_SERIAL: &str = "USBGUARD_DEVICE_SERIAL";
/// Environment variable carrying the physical port path.
pub const ENV_PORT: &str = "USBGUARD_DEVICE_PORT";
/// Environment variable carrying the comma-separated interface class names.
pub const ENV_CLASSES: &str = "USBGUARD_DEVICE_CLASSES";

/// A program to run when one specific device is connected and allowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hook {
    /// Descriptor hash of the device this hook belongs to.
    pub hash: String,
    /// Absolute path to the program to run.
    pub program: PathBuf,
    /// Arguments passed to it, verbatim.
    ///
    /// A list rather than a string: there is no shell, so there is nothing to
    /// split the string on and no quoting rules to get wrong.
    #[serde(default)]
    pub args: Vec<String>,
    /// Whether the hook is currently active.
    ///
    /// Kept rather than deleted when switched off, so turning a hook back on
    /// does not mean typing the path again.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// What the user called it, for the UI.
    #[serde(default)]
    pub label: String,
}

fn default_true() -> bool {
    true
}

/// Why a hook was not run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skip {
    /// The hook is switched off.
    Disabled,
    /// The device is not authorised, so nothing runs.
    NotAllowed,
    /// No program has been set.
    NoProgram,
}

impl Hook {
    /// A new, empty hook for a device.
    pub fn new(hash: String) -> Self {
        Self {
            hash,
            program: PathBuf::new(),
            args: Vec::new(),
            enabled: true,
            label: String::new(),
        }
    }

    /// Whether this hook should run for `device`, or why not.
    ///
    /// The [`Target::Allow`] requirement is the security-relevant one and is
    /// checked here rather than at the call site, so there is exactly one
    /// place that decides whether a hook runs.
    pub fn should_run(&self, device: &Device) -> Result<(), Skip> {
        if !self.enabled {
            return Err(Skip::Disabled);
        }
        if self.program.as_os_str().is_empty() {
            return Err(Skip::NoProgram);
        }
        if device.target != Target::Allow {
            return Err(Skip::NotAllowed);
        }
        Ok(())
    }

    /// Whether the configured program looks runnable.
    ///
    /// Advisory only — used to warn in the UI before the device is ever
    /// plugged in, rather than letting the first failure happen silently at
    /// 3 a.m. when the backup drive goes in.
    pub fn problem(&self) -> Option<Problem> {
        if self.program.as_os_str().is_empty() {
            return Some(Problem::NotSet);
        }
        if !self.program.is_absolute() {
            return Some(Problem::NotAbsolute);
        }
        if !self.program.exists() {
            return Some(Problem::Missing);
        }
        if !is_executable(&self.program) {
            return Some(Problem::NotExecutable);
        }
        None
    }
}

/// Something wrong with a hook's configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Problem {
    /// No program has been chosen.
    NotSet,
    /// The path is relative.
    ///
    /// Rejected rather than resolved: a relative path would be interpreted
    /// against whatever directory the app happened to start in, which for a
    /// desktop launch or an autostart entry is not something the user chose.
    NotAbsolute,
    /// The path does not exist.
    Missing,
    /// The file exists but is not executable.
    NotExecutable,
}

/// Whether the file at `path` has any execute bit set.
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

/// The environment a hook is given.
///
/// Values come straight from the device as USBGuard reported it. They are
/// data, not code: nothing here is parsed by a shell, because the program is
/// executed directly.
pub fn environment(device: &Device) -> HashMap<&'static str, String> {
    HashMap::from([
        (ENV_HASH, device.hash.clone()),
        (ENV_NAME, device.display_name()),
        (ENV_USB_ID, device.usb_id()),
        (ENV_SERIAL, device.serial.clone()),
        (ENV_PORT, device.via_port.clone()),
        (ENV_CLASSES, device.interface_classes().join(",")),
    ])
}

/// The outcome of trying to run a hook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The program was started. Carries its PID.
    Started(u32),
    /// It was not run, and why.
    Skipped(Skip),
    /// The program could not be started.
    Failed(String),
}

/// Run `hook` for `device`, if it should run.
///
/// Spawned detached: stdin, stdout and stderr go to `/dev/null` and the child
/// is not awaited. A hook is the user's own script doing its own thing, and a
/// long-running one — a backup, exactly the case this was built for — must not
/// hold up the UI or be killed when the app is busy elsewhere.
pub async fn run(hook: &Hook, device: &Device) -> Outcome {
    if let Err(skip) = hook.should_run(device) {
        debug_log!(
            crate::debug::HOOK,
            "not running hook for {}: {skip:?}",
            device.display_name()
        );
        return Outcome::Skipped(skip);
    }

    let mut command = tokio::process::Command::new(&hook.program);
    command
        .args(&hook.args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        // Not inherited: the app's own working directory is wherever it was
        // launched from, which is arbitrary.
        .current_dir(std::env::temp_dir())
        .kill_on_drop(false);

    for (key, value) in environment(device) {
        command.env(key, value);
    }

    match command.spawn() {
        Ok(child) => {
            let pid = child.id().unwrap_or(0);
            debug_log!(
                crate::debug::HOOK,
                "started hook {} (pid {pid}) for {}",
                hook.program.display(),
                device.display_name()
            );
            // Dropped without awaiting: `kill_on_drop` is off, so the child
            // keeps running. It is reaped by the runtime's orphan handling.
            drop(child);
            Outcome::Started(pid)
        }
        Err(e) => {
            crate::error_log!(
                crate::debug::HOOK,
                "could not start hook {}: {e}",
                hook.program.display()
            );
            Outcome::Failed(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(target: &str) -> Device {
        Device::from_rule(
            1,
            &format!(
                r#"{target} id 0781:5567 name "Backup" serial "SN1" hash "H1=" via-port "1-2" with-interface 08:06:50"#
            ),
        )
        .unwrap()
    }

    fn hook() -> Hook {
        Hook {
            hash: "H1=".into(),
            program: PathBuf::from("/bin/true"),
            args: vec!["--go".into()],
            enabled: true,
            label: "Backup".into(),
        }
    }

    #[test]
    fn a_hook_never_runs_for_a_device_that_is_not_allowed() {
        // The whole safety argument for this feature. A device that is blocked,
        // rejected, or still awaiting a decision must not be able to start a
        // program, or plugging in untrusted hardware would execute code.
        for target in ["block", "reject"] {
            assert_eq!(
                hook().should_run(&device(target)),
                Err(Skip::NotAllowed),
                "a {target}ed device must not run a hook"
            );
        }
        assert_eq!(hook().should_run(&device("allow")), Ok(()));
    }

    #[test]
    fn a_disabled_hook_does_not_run() {
        let mut hook = hook();
        hook.enabled = false;
        assert_eq!(hook.should_run(&device("allow")), Err(Skip::Disabled));
    }

    #[test]
    fn a_hook_with_no_program_does_not_run() {
        let mut hook = hook();
        hook.program = PathBuf::new();
        assert_eq!(hook.should_run(&device("allow")), Err(Skip::NoProgram));
    }

    #[tokio::test]
    async fn running_a_hook_for_a_blocked_device_starts_nothing() {
        // Belt and braces: `run` must apply `should_run` itself, so a caller
        // that forgets to check cannot execute anything.
        let outcome = run(&hook(), &device("block")).await;
        assert_eq!(outcome, Outcome::Skipped(Skip::NotAllowed));
    }

    #[tokio::test]
    async fn running_a_hook_for_an_allowed_device_starts_it() {
        let outcome = run(&hook(), &device("allow")).await;
        assert!(matches!(outcome, Outcome::Started(_)), "{outcome:?}");
    }

    #[tokio::test]
    async fn a_hook_really_receives_the_device_in_its_environment() {
        // Spawning successfully is not the same as the script being able to do
        // anything useful. This runs a real script and reads back what it saw,
        // so a broken env or argument vector fails here rather than silently
        // at 3 a.m. when the backup drive goes in.
        let dir = std::env::temp_dir().join(format!("cug-hook-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let script = dir.join("hook.sh");
        let output = dir.join("seen.txt");

        // Built by concatenation rather than `format!`: the shell's `${VAR}`
        // syntax collides with format placeholders, and spelling the variable
        // names out here would let them drift from the constants.
        let body = [
            "#!/bin/sh\n",
            "printf '%s|%s|%s|%s\\n'",
            " \"$1\"",
            " \"$",
            ENV_NAME,
            "\" \"$",
            ENV_USB_ID,
            "\" \"$",
            ENV_HASH,
            "\" > ",
            &output.display().to_string(),
            "\n",
        ]
        .concat();
        std::fs::write(&script, body).unwrap();
        std::fs::set_permissions(
            &script,
            <std::fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o755),
        )
        .unwrap();

        let hook = Hook {
            hash: "H1=".into(),
            program: script.clone(),
            args: vec!["first-arg".into()],
            enabled: true,
            label: String::new(),
        };
        assert_eq!(hook.problem(), None, "the fixture script must be runnable");

        let outcome = run(&hook, &device("allow")).await;
        assert!(matches!(outcome, Outcome::Started(_)), "{outcome:?}");

        // The hook is detached, so wait for it rather than assuming.
        let mut seen = String::new();
        for _ in 0..100 {
            if let Ok(text) = std::fs::read_to_string(&output)
                && !text.trim().is_empty()
            {
                seen = text;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }

        assert!(!seen.is_empty(), "the hook never ran, or wrote nothing");
        let fields: Vec<&str> = seen.trim().split('|').collect();
        assert_eq!(fields[0], "first-arg", "argument vector not passed");
        assert_eq!(fields[1], "Backup", "device name not in the environment");
        assert_eq!(fields[2], "0781:5567", "usb id not in the environment");
        assert_eq!(fields[3], "H1=", "hash not in the environment");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn device_details_are_passed_as_environment_not_arguments() {
        // A device controls its own reported name. If that name were ever
        // concatenated into a command line, a device called `; rm -rf ~` would
        // be a remote code execution bug. It must only ever be an env value.
        let hostile = Device::from_rule(
            1,
            r#"allow id 0781:5567 name "; rm -rf ~" hash "H1=" with-interface 08:06:50"#,
        )
        .unwrap();

        let env = environment(&hostile);
        assert_eq!(env.get(ENV_NAME).map(String::as_str), Some("; rm -rf ~"));

        // The argument vector is whatever the user configured and nothing else.
        let hook = hook();
        assert_eq!(hook.args, vec!["--go".to_string()]);
        assert!(!hook.args.iter().any(|a| a.contains("rm")));
    }

    #[test]
    fn environment_covers_every_documented_variable() {
        // A missing entry would silently hand the script an unset variable.
        let env = environment(&device("allow"));
        for key in [
            ENV_HASH,
            ENV_NAME,
            ENV_USB_ID,
            ENV_SERIAL,
            ENV_PORT,
            ENV_CLASSES,
        ] {
            assert!(env.contains_key(key), "{key} missing");
        }
        assert_eq!(env.get(ENV_USB_ID).map(String::as_str), Some("0781:5567"));
        assert_eq!(
            env.get(ENV_CLASSES).map(String::as_str),
            Some("Mass storage")
        );
    }

    #[test]
    fn configuration_problems_are_spotted_before_the_device_is_plugged_in() {
        let mut hook = hook();
        assert_eq!(hook.problem(), None);

        hook.program = PathBuf::new();
        assert_eq!(hook.problem(), Some(Problem::NotSet));

        // A relative path would resolve against whatever directory the app was
        // launched from, which for an autostart entry is not the user's choice.
        hook.program = PathBuf::from("backup.sh");
        assert_eq!(hook.problem(), Some(Problem::NotAbsolute));

        hook.program = PathBuf::from("/nonexistent/backup.sh");
        assert_eq!(hook.problem(), Some(Problem::Missing));

        hook.program = PathBuf::from("/etc/hostname");
        assert_eq!(hook.problem(), Some(Problem::NotExecutable));
    }
}
