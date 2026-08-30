# COSMIC USB Guard

A [COSMIC](https://system76.com/cosmic) desktop front-end and panel indicator
for [USBGuard](https://usbguard.github.io/).

USBGuard is excellent at enforcing USB policy and unpleasant to live with. It
blocks unknown devices by default, then expects you to notice that nothing
happened, open a terminal, run `usbguard list-devices`, work out which of the
sixteen entries is the drive you just plugged in, and type its ID into
`usbguard allow-device`. Most people give up and disable it.

This puts a shield icon in the COSMIC panel instead. Plug something in and you
get asked about it, with enough information to answer. Everything you decide is
recorded, and you can change your mind later.

> **Status:** early. The core works and is tested; see [ROADMAP.md](ROADMAP.md)
> for what is planned and [Known limitations](#known-limitations) for what is
> not there yet.

---

## What it does

- **Prompts on insertion.** A new device stays blocked until you answer. The
  prompt shows the product name, vendor and product IDs, serial number, port,
  interface classes and descriptor hash.
- **Warns about keyboard-capable devices.** A device that claims a
  human-interface class can type on your behalf. That is the whole basis of the
  "BadUSB" family of attacks, and it is called out separately from whatever
  else the device claims to be — a "headset" that also registers a keyboard is
  shown as an input device.
- **Lists everything.** Every device USBGuard can see, what it is, and whether
  it is allowed or blocked, with the internal hubs filtered out by default.
- **Revokes properly.** "Revoke" removes the standing rule *and* blocks the
  device now. Blocking alone would leave a rule that re-authorises it on the
  next replug, which is the trap that makes people think USBGuard is broken.
- **Keeps a history.** Insertions, removals, and every decision — yours,
  another front-end's, the command line's, or USBGuard's own policy —
  are written to a JSON Lines journal you can read with ordinary tools.
- **Checks that USBGuard actually works.** Not that it is installed: that the
  service is running, that the D-Bus interface is up, that your account may
  make decisions, that the policy is not empty, and that `InsertedDevicePolicy`
  is not set to `allow` — which authorises every new device before anyone can
  be asked about it. Each failed check comes with the command that fixes it.

## Why the status checks matter

A USBGuard install can be present, syntactically valid, and completely inert.
The service can be disabled. The D-Bus bridge can be missing, in which case
every GUI silently sees nothing. The policy can be empty. `InsertedDevicePolicy`
can be `allow`, which means a device is authorised the instant it is plugged
in — so a prompt would be asking permission for something that has already
happened.

None of that is visible by looking at the icon in your panel. The Status page
probes the running system for each of these and tells you which one is wrong.

---

## Requirements

- COSMIC desktop (Pop!\_OS 24.04 or newer, or any COSMIC session)
- `usbguard` 1.1.0 or newer, with the D-Bus bridge
- A Rust toolchain (1.93+) if building from source

On Debian and Ubuntu derivatives:

```sh
sudo apt install usbguard
```

## Setting up USBGuard

> **Read this before enabling USBGuard.** Starting the daemon with an empty
> policy blocks every USB device, *including your keyboard and mouse*. On a
> laptop with a built-in keyboard you will be fine; on a desktop with USB
> peripherals you can lock yourself out of your own machine. Generate a policy
> from your currently connected devices **first**.

**1. Capture your current devices as the baseline policy.**

Plug in the peripherals you want to keep working — keyboard, mouse, dock,
webcam — then:

```sh
sudo sh -c 'usbguard generate-policy > /etc/usbguard/rules.conf'
sudo chmod 0600 /etc/usbguard/rules.conf
```

**2. Let your user account make decisions.**

Without this, the applet can see devices but cannot act on them.

```sh
sudo usbguard add-user "$USER" --devices modify,list --policy list --exceptions list
```

**3. Hold new devices for a decision instead of auto-authorising them.**

```sh
sudo usbguard set-parameter InsertedDevicePolicy apply-policy
```

**4. Start both services.** The second one is the D-Bus bridge; without it no
graphical front-end can talk to the daemon at all.

```sh
sudo systemctl enable --now usbguard.service
sudo systemctl enable --now usbguard-dbus.service
```

**5. Check it.** Open USB Guard and look at the Status page. Every row should
be green. If one is not, it tells you the command to run.

### If you lock yourself out

Boot with the daemon disabled from a recovery shell or a TTY on the built-in
keyboard:

```sh
sudo systemctl stop usbguard.service
```

Or, from another machine over SSH, the same command. Then fix
`/etc/usbguard/rules.conf` and start it again.

---

## Installing

### From source

```sh
git clone https://github.com/stldave314/cosmic-usb-guard
cd cosmic-usb-guard
./install.sh
```

`install.sh` builds both binaries and installs them along with the desktop
entries, icons and AppStream metadata. It passes `--features release-build`, so
an installed build can never carry developer debug logging.

### Packages

`.deb`, `.rpm` and a portable tarball are attached to each
[release](https://github.com/stldave314/cosmic-usb-guard/releases).

```sh
sudo apt install ./cosmic-usb-guard_*.deb
```

### Adding the panel indicator

The applet does not appear in the panel by itself. Open **Settings → Desktop →
Panel → Configure panel applets**, then add **USB Guard**.

---

## Using it

### The panel icon

| Icon | Meaning |
| --- | --- |
| Shield, full | USBGuard is running and healthy, nothing outstanding |
| Shield, half | A device is waiting for a decision, or a check is failing |
| Shield, low | Not connected to USBGuard, or this machine is not protected |

A pending decision never shows the all-clear icon, even when everything else is
healthy — you still have to act.

### Deciding about a device

When a device is plugged in, the popup opens with a card for it. **Allow**
authorises it, **Block** leaves it de-authorised, **Dismiss** stops asking
without changing anything (the device stays blocked, because that is what
USBGuard already did).

**Remember this decision** writes a permanent rule so the same device is handled
the same way next time. It is off by default, deliberately: a standing rule is
the more consequential of the two choices and should be a deliberate act. You
can change that default in Settings.

Permanent rules are pinned to the device's **descriptor hash**, not its USB ID.
A rule written as `allow id 0781:5567` authorises anything claiming those IDs,
which is exactly what a spoofing device relies on. The hash identifies the
specific device.

### The Devices page

Every device, sorted so anything wanting attention is at the top. Click a row
to expand it and see the full details, with copy buttons for the fields you
might want to paste somewhere.

Internal hubs and soldered-in devices are hidden by default — they are part of
the machine, not something you plugged in. Turn them on in Settings. A device
awaiting a decision is always shown regardless of the filters.

### The History page

Every event, newest first, with who caused it: you, USBGuard's own policy, or
something outside this app. Filter to decisions only if the plug/unplug noise
gets in the way.

The journal is written to `~/.local/share/cosmic-usb-guard/events.jsonl`, one
JSON object per line, and rotated at 4 MiB:

```sh
jq -r 'select(.kind == "allowed") | "\(.timestamp) \(.device.name)"' \
  ~/.local/share/cosmic-usb-guard/events.jsonl
```

This is a record, not a security boundary — it lives in your home directory and
enforces nothing. USBGuard's own audit log is the authoritative one.

---

## Settings

| Setting | Default | What it does |
| --- | --- | --- |
| Ask about new devices | On | Raise a decision prompt for a device with no standing rule |
| Send a desktop notification | On | Also notify, so a prompt is not missed when the panel is hidden |
| Open the panel popup automatically | On | Let the indicator open itself when a decision is waiting |
| Remember decisions by default | Off | Pre-tick "remember this decision" in the prompt |
| Show internal devices | Off | Include devices that are soldered in and cannot be unplugged |
| Show root hubs | Off | Include the host controllers the USB ports hang off |
| Highlight keyboard-capable devices | On | Call out devices that could inject keystrokes |
| Keep a decision history | On | Record events to the journal |
| Warn about configuration problems | On | Show an alert when USBGuard is not set up to protect this system |

Settings are stored through `cosmic-config` and shared between the applet and
the window, so changing one changes both.

---

## Troubleshooting

**The Status page says "Not connected to USBGuard".**
The D-Bus bridge is the usual cause. `sudo systemctl enable --now
usbguard-dbus.service`. It is a separate unit from `usbguard.service` and is not
enabled by default on most distributions.

**Devices are listed but Allow does nothing, or asks for a password every time.**
Your account is not in USBGuard's IPC allow-list, so every call falls through to
a Polkit prompt. Run the `usbguard add-user` command from
[Setting up USBGuard](#setting-up-usbguard) and log out and back in.

**Nothing prompts when I plug something in.**
Check `InsertedDevicePolicy` on the Status page. If it is `allow` or `keep`, the
device is authorised before we hear about it. Set it to `apply-policy`.

**Everything is blocked, including things I allowed before.**
Your policy is probably empty — check the Status page. Regenerate it with
`usbguard generate-policy` while your known-good devices are connected.

**The applet is not in the panel list.**
The desktop entry has to be in the system scan path, which means
`/usr/share/applications/`. A build installed under `~/.local` will not be found
by `cosmic-panel`. Use `install.sh`, which installs system-wide.

**I already have `usbguardgui` installed.**
Both will prompt for the same device. Pick one:
`sudo systemctl --user disable --now USBGuardGUIDBus` or uninstall the other.

**Something else is wrong.**
Turn on developer logging: set `DEVELOPER_LOGGING` to `true` in `src/debug.rs`,
rebuild *without* `--features release-build`, and read
`/tmp/cosmic-usb-guard-debug.log`. Categories are tagged (`DBUS`, `DEVICE`,
`POLICY`, `HEALTH`, `UI`, `CONFIG`, `JOURNAL`, `NOTIFY`) so you can `grep` a run.

---

## Known limitations

- **The applet must be in your panel for insertion prompts.** There is no
  background daemon yet; if the applet is not running, nothing is watching.
  Tracked in [ROADMAP.md](ROADMAP.md).
- **The window application does not prompt.** It shows pending decisions when
  it is open, but the applet is what notices an insertion.
- **English only.** The localisation system is wired up and enforced by
  `tests/i18n.rs`; no other locales are translated yet. Contributions welcome.
- **No screenshots in this README yet.**

---

## How it works

```
  ┌──────────────────┐        ┌─────────────────────────┐
  │  panel applet    │        │  window application     │
  │  (prompts,       │        │  (devices, history,     │
  │   notifications) │        │   status, settings)     │
  └────────┬─────────┘        └───────────┬─────────────┘
           └──────────────┬───────────────┘
                          │  shared state, views, journal
                 ┌────────▼─────────┐
                 │  usbguard client │  zbus
                 └────────┬─────────┘
                          │  org.usbguard1 (system bus)
                 ┌────────▼─────────┐
                 │  usbguard-dbus   │
                 │  usbguard-daemon │  ← the thing that actually enforces
                 └──────────────────┘
```

Both binaries are thin: they render the same `State` and issue the same
actions. Layering inside the crate, innermost first:

| Module | Responsibility |
| --- | --- |
| `usbguard::rule` | Parses USBGuard rule strings, the only form in which the daemon describes a device |
| `usbguard::model` | Turns a parsed rule into a `Device` |
| `usbguard::proxy` | Raw zbus interface definitions, taken from the introspection XML in `usbguard-dbus` |
| `usbguard::client` | Timeouts and error classification over those proxies |
| `usbguard::events` | Owns the connection lifecycle; one self-healing event stream with reconnect back-off |
| `usbguard::health` | Probes whether USBGuard is actually protecting the machine |
| `state` | The model both binaries render, and its transitions |
| `ui` | Shared views, emitting a common `Action` |
| `journal` | The append-only decision log |

This crate enforces nothing. USBGuard does. If this app is not running, your USB
policy is exactly as it was.

---

## Building

```sh
cargo build --release --features release-build
cargo test
cargo clippy --all-targets
```

The test suite includes checks that are designed not to pass vacuously:

- `tests/i18n.rs` asserts every locale carries exactly the fallback key set with
  identical placeholders, and fails if the fallback locale is missing rather
  than finding nothing to compare.
- `tests/icons.rs` asserts every icon name resolves against the installed icon
  themes, and first asserts that a control icon exists — so an uninstalled icon
  theme fails loudly instead of reporting green.
- `scripts/verify-release-build.sh` proves `--features release-build` actually
  strips developer logging, by building both ways with the developer switch
  forced on and checking whether the log path survives into the binary. It fails
  if the "logging on" build does *not* contain the path, because in that state
  the check could not detect a regression.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Bug reports and translations are both
very welcome.

## Licence

GPL-3.0-or-later. See [LICENSE](LICENSE).

USBGuard itself is a separate project, licensed GPL-2.0-or-later.
