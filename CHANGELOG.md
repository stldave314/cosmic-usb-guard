# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0]

### Added

- Status icon in the system tray, published as a `StatusNotifierItem`, with
  Open, Refresh and Quit. Replaces the panel applet — nothing to add to the
  panel by hand.
- Autostart, "start minimised to the icon", "keep running when the window is
  closed" and "hide the status icon" settings.
- Per-device hook programs: attach a program to one trusted device and it runs
  once that device is connected *and* authorised. Pinned to the descriptor
  hash, executed directly with an argument vector rather than through a shell,
  and detached so a long-running script cannot block the UI.
- Notification when a standing rule refuses a device without prompting, with a
  "Manage device" action that opens the window at that device.
- Devices that are not plugged in but still have a standing rule are listed in
  their own section, so a permanent decision can be changed without the device.
- Devices can be marked as internal, pinned to the descriptor hash. Internal
  devices are hidden and never prompted about, and the mark authorises nothing.
- `DecisionsReversible` health check, asking Polkit whether this session may
  remove a policy rule, with a drop-in polkit rule as its remedy.
- `--minimized` and `--page <name>` command-line options.
- Twelve machine-translated locales: Arabic, Chinese (Simplified), French,
  German, Hindi, Italian, Japanese, Korean, Portuguese (Brazil), Russian,
  Spanish and Turkish. Unreviewed; corrections welcome.
- Screenshots in the README.

### Changed

- Changing a permanent decision now *replaces* the conflicting rule instead of
  appending a second one. USBGuard stops at the first matching rule, so an
  `allow` written behind an existing `reject` for the same device never fired.
- Policy-changing D-Bus calls now set `ALLOW_INTERACTIVE_AUTHORIZATION`, so a
  Polkit `auth_admin` action raises an authentication dialog instead of coming
  back as a bare `AccessDenied` with nothing shown to the user.
- The Status page headline is `title4`; at `title3` it wrapped to three
  oversized lines beside the Refresh button.
- Dependencies updated. `tinyvec` is pinned to 1.12.0 in `Cargo.lock`: 1.13.0
  does not compile.

### Removed

- The `cosmic-usb-guard-applet` binary and its desktop entry. **Breaking:** an
  existing panel applet entry will stop working; launch the application
  instead and it puts its own icon in the tray.

## [0.1.0]

First release.

### Added

- COSMIC panel applet with an icon reflecting protection status, and a popup
  for allowing or denying devices.
- Window application with Devices, History, Status and Settings pages.
- Decision prompt on device insertion, with an optional permanent rule pinned
  to the device's descriptor hash rather than its spoofable USB ID.
- Desktop notifications carrying inline Allow, Block and Details actions.
- Warnings for devices presenting a human-interface class, which can inject
  keystrokes.
- Revoke, which removes a device's standing rule and blocks it in one step.
- JSON Lines decision journal with rotation, and a History view over it.
- Installation health checks probed against the running system — service and
  D-Bus unit state, IPC reachability, IPC permission, `InsertedDevicePolicy`
  and policy contents — each with a copyable fix-it command.
- Settings persisted through `cosmic-config` and shared by both binaries.
- Localisation scaffolding with a test enforcing locale key and placeholder
  parity.
- `.deb`, `.rpm` and tarball packaging, and tag-triggered releases.

[Unreleased]: https://github.com/stldave314/cosmic-usb-guard/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/stldave314/cosmic-usb-guard/releases/tag/v0.1.0
