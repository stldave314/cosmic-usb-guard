# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

### Added

- Bind a remembered USB decision to the port the device is in: "Bind to this
  USB port" in the decision prompt, and "Bound to USB port" in the device
  details. Everything a USB device reports can be copied by one built to
  impersonate it; the port cannot. A bound device in another port is blocked
  and asked about again, and the prompt says which port its rule names.
- Thunderbolt and USB4 support, through bolt. A Thunderbolt page shows the
  controller's security level and DMA protection and lists bolt's devices,
  with Allow once, Always allow and Forget. A device waiting to be allowed
  raises the same prompt and notification as a USB device.
- Thunderbolt health checks: security level, DMA protection, bolt running, and
  permission to authorize devices. Problems only firmware setup can fix come
  with numbered steps instead of a command.
- `--device <hash>` opens the app with that device's details expanded, and
  `--page thunderbolt` opens the new page.
- Screenshot of the Thunderbolt page.

### Fixed

- Allow on a device that was not plugged in and already always allowed failed
  with "permission denied". The click removed the existing rule in order to
  write the same one back, and rule removal is the one call the stock polkit
  rules do not grant without an administrator password. The button is no
  longer offered there, and a decision the policy already records no longer
  removes anything.
- Error messages were unreadable under themes whose warning color and its
  foreground are close together — white text on pale gray. They are now shown
  on a card in the theme's normal text color, with an error icon.

- The polkit rule the Status page offers never took effect for anyone with a
  restrictive umask. It was written with `sudo tee`, which created it as
  `0640 root:root`; polkitd does not run as root, could not read it, and
  logged only "Error loading script". It is now written with
  `install -m 0644`, and running the new command repairs a file the old one
  left behind.
- A health problem with a fix now says what was observed, and what to do with
  the copied command: paste it into a terminal, expect a password prompt and
  no output, then press Refresh. The banner's button reads "Copy command"
  rather than a bare "Copy".

- Device details lined up badly: rows with a copy button pushed their value
  sideways and stood taller than rows without one. Every row now reserves the
  button's slot, the policy notes sit in their own block below the fields, and
  the "Internal device", "Remember this decision" and hook "Enabled" switches
  use the Settings page's layout — label and description on the left, switch
  on the right — instead of a label pressed against the switch.
- The Status field of a device that is not plugged in read "Allowed". It now
  reads "Not connected"; the standing rule is reported on its own line.
- British spellings in three English UI strings.
- Field values in device details now wrap inside the card. The descriptor hash
  is one long word and ran under its copy button and out of the card.
- The hook editor said a hook receives "USBGUARD_DEVICE_NAME" only; it
  receives six variables, and now lists all of them.
- Running the test suite appended a fabricated "DaemonRunning: failed" event
  to the user's real decision history every time. Tests now keep journal
  entries in memory, and a test checks the real file is untouched.

- Security hardening from a review of the whole codebase:
  - Device names in notifications are escaped when the notification server
    renders markup. A device named with HTML could otherwise add a link to its
    own prompt, or hide the rest of it, including the warning that it can type.
  - A device claiming the root-hub vendor ID (`1d6b`) is only treated as a
    root hub when it sits on a bus's root port. Before, the vendor ID alone was
    enough to be hidden from the list and never prompted about.
  - A hook whose program is not an absolute path is refused when it would run,
    not just warned about in the editor; it was being looked up on `$PATH`.
  - The fix for an empty policy now warns that it allows every device plugged
    in at that moment, and restarts the daemon so the new policy takes effect.
  - The fix for `InsertedDevicePolicy` also writes the setting to the daemon's
    config file, so it survives a restart.
  - The polkit rule offered for undoing decisions also covers the `wheel`
    group, for distributions that use it instead of `sudo`.
- Notifications for prompts that had already gone — a device allowed or
  unplugged while its notification was being posted, or withdrawn during a
  refresh — stayed on screen with nothing behind them. They are now closed.
- The "bind to this USB port" switch stayed on for the next device's prompt
  after being used. It now applies to one device's prompt only.
- Dismissing a Thunderbolt prompt lasted only until the next refresh. It now
  lasts until the device stops waiting.
- A failed read from bolt dropped every Thunderbolt prompt, and the next good
  read announced each waiting device as newly plugged in.
- Thunderbolt devices were prompted about at security levels that never
  authorize anything, where Allow could only fail.
- Releasing a port binding on a device blocked for being on the wrong port
  cleared its prompt but left it blocked. The rule's decision is now applied.
- A slow bolt no longer delays the USB device list and checks: both are read
  at the same time.

- The fix offered when USB Guard was not allowed to use USBGuard was
  `usbguard add-user`, which edits USBGuard's IPC allow-list. That list only
  governs the `usbguard` command line: the D-Bus bridge USB Guard uses runs as
  root and is governed by polkit. "You may make policy decisions" now asks
  polkit about making a decision, and offers one polkit rule granting making,
  writing and removing to active local members of `sudo`, `wheel` and
  `plugdev`. It replaces the earlier rule that granted removal alone. When
  reading itself is refused, the check explains that polkit only serves the
  active local session, with steps, rather than offering a command.
- Decisions are recorded in the history once the daemon has accepted them,
  not when the button is pressed; a refused decision no longer appears as
  made. Thunderbolt allow, always-allow and forget are recorded too.
- The history file is created owner-only, in an owner-only directory, and is
  not written at all when there is no home directory — it used to fall back
  to a fixed path under `/tmp` that another user could create first.
- A notification click only acts on the device it described. Device IDs are
  reused, so a click on an old notification could otherwise allow whichever
  device had since been given that ID.
- A broader rule earlier in the policy — `allow id 0781:5567`, say — that
  could overrule a device's own rule is now named in the device's details.
- Rules with a condition (`if …`) failed to parse and were skipped, so the
  app could not see them at all. The condition is now kept as one value.
- A rule pinning a set of hashes (`hash one-of { … }`) is no longer treated
  as the first device's rule, so forgetting or changing that device cannot
  rewrite another's.
- Dismissing a USB prompt lasted only until the next refresh. It now lasts
  until the device is unplugged or plugged in again.
- A device that kept re-enumerating could keep clearing the latch that stops
  a refused call being retried; events from the daemon no longer clear it.
  Health checks no longer make their own policy calls on every tick.
- The values shown after "Observed:" are translated.
- The autostart entry's command line now doubles `%` and escapes backslashes
  for both layers of unescaping the desktop-entry format applies.
- The developer debug log refuses to open through a symlink, is created
  owner-only, and is not used if another user owns the file.

### Changed

- Dependencies updated, including libcosmic. `tinyvec` 1.13.3 builds, so the
  1.12.0 pin is gone. zbus stays on 5.x: the tray icon (`ksni`) and libcosmic
  both use zbus 5, and moving this crate to 6 would ship two D-Bus stacks.
- "USB Guard" always means this app and "USBGuard" always means the system
  service it fronts. Overall verdicts no longer name the service — "This
  system is protected", "Protection has problems" — since they now cover
  Thunderbolt too, and service mentions read "the USBGuard service".

- Redrawn application and symbolic icons. The trident is now the recognizable
  USB one: arrow tip, round and square branch ends at different heights, and
  the plug circle at the base.

## [0.2.0]

### Added

- Status icon in the system tray, published as a `StatusNotifierItem`, with
  Open, Refresh and Quit. Replaces the panel applet — nothing to add to the
  panel by hand.
- Autostart, "start minimized to the icon", "keep running when the window is
  closed" and "hide the status icon" settings.
- Per-device hook programs: attach a program to one trusted device and it runs
  once that device is connected *and* authorized. Pinned to the descriptor
  hash, executed directly with an argument vector rather than through a shell,
  and detached so a long-running script cannot block the UI.
- Notification when a standing rule refuses a device without prompting, with a
  "Manage device" action that opens the window at that device.
- Devices that are not plugged in but still have a standing rule are listed in
  their own section, so a permanent decision can be changed without the device.
- Devices can be marked as internal, pinned to the descriptor hash. Internal
  devices are hidden and never prompted about, and the mark authorizes nothing.
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
- Localization scaffolding with a test enforcing locale key and placeholder
  parity.
- `.deb`, `.rpm` and tarball packaging, and tag-triggered releases.

[Unreleased]: https://github.com/stldave314/cosmic-usb-guard/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/stldave314/cosmic-usb-guard/releases/tag/v0.1.0
