# Contributing

Thanks for looking. Bug reports, translations and code are all welcome.

## Reporting a bug

Please include:

- What you expected and what happened instead.
- The output of the **Status** page, or `usbguard --version` plus
  `systemctl is-active usbguard.service usbguard-dbus.service`.
- Your distribution and COSMIC version.

If the problem involves a specific device, the expanded device details from the
Devices page are exactly what is needed. **Redact the serial number and
descriptor hash if the device is not yours to identify** — the hash is a stable
identifier for that physical device.

If something is misbehaving rather than obviously broken, a debug log helps:
set `DEVELOPER_LOGGING` to `true` in `src/debug.rs`, rebuild *without*
`--features release-build`, reproduce, and attach
`/tmp/cosmic-usb-guard-debug.log`.

## Building and testing

```sh
cargo build
cargo test
cargo clippy --all-targets
```

Both `cargo check` and `cargo clippy` must be warning-free before a change
lands. Warnings from dependencies are out of scope.

## Translations

This is the easiest way to help and the most visible.

1. Copy `i18n/en/cosmic_usb_guard.ftl` to `i18n/<locale>/cosmic_usb_guard.ftl`.
2. Translate the values. Leave the keys alone.
3. Keep every `{ $placeholder }` intact and spelled the same. A mangled
   placeholder only misbehaves in that one language, at runtime.
4. `cargo test --test i18n`.

`tests/i18n.rs` will tell you about missing keys, keys that no longer exist,
duplicates, and placeholder mismatches. It is not advisory — CI runs it.

**When you change a translatable string, change it in every locale in the same
commit.** Fluent falls back silently, so a key added only to `en` shows up as
stray English elsewhere rather than as a build error.

## Code

A few conventions this codebase holds to. They are not arbitrary; each one
exists because the alternative caused a real problem.

**Prove security-relevant behaviour against a running system.** Reading the
code is not evidence. A control can be present, valid, reviewed and completely
inert. Everything in `usbguard::health` is an empirical probe for this reason,
and `scripts/verify-release-build.sh` builds the binary both ways rather than
trusting that a `cfg!` does what it says.

**A check that cannot fail is not a check.** `tests/icons.rs` asserts a control
icon exists before asserting ours do, so an uninstalled icon theme fails loudly
instead of finding nothing to check and reporting green. `tests/i18n.rs` fails
if the fallback locale is missing. Apply the same standard to new tests.

**Fail towards asking.** When the app cannot determine something — whether a
device has a standing rule, whether the daemon is healthy — it must behave as
though the answer were the one that prompts the user, not the one that stays
quiet. See `State::has_standing_rule`.

**Never present stale state as current.** When the daemon goes away the device
list is cleared rather than left on screen; showing yesterday's authorisations
as though they were live is worse than showing nothing.

**User-facing strings go through `fl!`.** No hardcoded English in the UI,
including error and status messages.

**Settings versus constants.** Anything a user would reasonably change belongs
in `src/config.rs`, with a UI control. Implementation tuning values belong in
`src/constants.rs` as compile-time constants. Do not add a second runtime
configuration file.

**Debug logging goes to a file, behind the `debug_log!` macro.** A panel
applet's stderr is piped to `cosmic-panel` and is effectively unreadable.
Genuine errors still go to stderr as well, via `error_log!`.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`,
`docs:`, `chore:`, `build:`, `refactor:`. Only `feat:` and `fix:` are
version-bumping.

Write a body explaining *why*, not what — the diff already says what.

Keep the README current in the same commit as the change. If a feature is added
or observable behaviour changes, the feature list, settings table, usage and
troubleshooting sections need to match. If nothing user-visible changed, say so
rather than leaving it silently stale.

## Licence

By contributing you agree that your work is licensed under GPL-3.0-or-later,
the same as the project.
