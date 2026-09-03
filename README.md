# COSMIC USB Guard

A [COSMIC](https://system76.com/cosmic) desktop front-end for
[USBGuard](https://usbguard.github.io/), with a system-tray status icon.

USBGuard is excellent at enforcing USB policy and unpleasant to live with. It
blocks unknown devices by default, then expects you to notice that nothing
happened, open a terminal, run `usbguard list-devices`, work out which of the
sixteen entries is the drive you just plugged in, and type its ID into
`usbguard allow-device`. Most people give up and disable it.

This puts a shield in your system tray instead. Plug something in and you get
asked about it, with enough information to answer. Everything you decide is
recorded, you can change your mind later, and you can attach a script to a
device you trust — so plugging in the backup drive can start the backup.

![The Devices page, showing a connected device and a disconnected one that a
standing rule still rejects](docs/screenshots/devices.png)

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
- **Lets you change your mind.** Every permanent decision can be taken back,
  including one made about a device that is no longer plugged in. Changing a
  decision *replaces* the old rule rather than adding a second one, because
  USBGuard stops at the first rule that matches — a new `allow` appended behind
  an existing `reject` would never be reached.
- **Shows devices that are not connected.** Anything with a standing rule is
  listed in its own section even when unplugged, so a rule can be changed or
  removed without hunting for the device. This matters most for a *rejected*
  device, which USBGuard detaches on sight, so plugging it back in does not
  reliably give you anything to click.
- **Lets you mark internal devices.** A fingerprint reader or card reader on an
  internal header is part of the machine, but USBGuard often cannot tell — a
  Goodix sensor reports `with-connect-type "not used"`, not `"hardwired"`, so
  no heuristic gets it right. Mark it yourself and it stops appearing in the
  list and stops being asked about. The mark is pinned to the descriptor hash
  and never authorises anything on its own.
- **Runs a script for a device you trust.** Attach a program to one specific
  device and it runs when that device is connected — a backup script when the
  backup drive goes in. It only ever runs *after* the device is authorised, it
  is pinned to the device's descriptor hash rather than its USB ID, and it is
  executed directly with an argument vector rather than through a shell.
- **Says so when a device is refused silently.** A device with a standing block
  or reject rule never raises a prompt, so from your side the drive just does
  not appear. A notification explains why and links straight to the device.
- **Lives in the system tray.** No panel applet to add: the app publishes a
  StatusNotifierItem, which COSMIC's Status Area picks up on its own. It can
  start with your session, minimised to the icon, and keep watching when you
  close the window.
- **Keeps a history.** Insertions, removals, and every decision — yours,
  another front-end's, the command line's, or USBGuard's own policy —
  are written to a JSON Lines journal you can read with ordinary tools.
- **Checks that USBGuard actually works.** Not that it is installed: that the
  service is running, that the D-Bus interface is up, that your account may
  make decisions, that you may also *undo* one, that the policy is not empty,
  and that `InsertedDevicePolicy` is not set to `allow` — which authorises
  every new device before anyone can be asked about it. Each failed check comes
  with the command that fixes it.

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

Without this, USB Guard can see devices but cannot act on them.

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

### The status icon

There is nothing to add to the panel. Launch USB Guard and it publishes a
`StatusNotifierItem`, which the **Status Area** applet — in the COSMIC panel by
default — picks up on its own. The same works on any desktop with a
StatusNotifierItem host (KDE Plasma, and GNOME with the AppIndicator
extension).

Left-click the icon to open the window; right-click for Open, Refresh and Quit.
If the desktop has no tray at all, the app says so and opens its window instead
of hiding where you cannot reach it.

To have it watching from login, turn on **Start automatically at login** in
Settings — optionally with **Start without opening the window**, which shows
only the icon.

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

### Changing a decision

Answered "Block" when you meant "Allow"? The device keeps a row for as long as
it has a standing rule, whether or not it is still plugged in — look under
**Not connected** if it is unplugged. Two controls take a decision back:

- **Allow** writes the opposite decision. Where a standing rule already
  contradicts it, the old rule is removed first and the decision is treated as
  permanent regardless of the "remember" toggle, since a one-off answer would
  be undone on the next replug.
- **Forget rule** removes the standing rule and changes nothing else. The
  device falls back to USBGuard's implicit target, which is to block and ask —
  so you get the question again rather than having to commit to the opposite
  answer.

Removing a rule is the one operation most distributions do not grant without a
password. The polkit rules Debian and Ubuntu ship for USBGuard give
`appendRule` and `applyDevicePolicy` to the `sudo` and `plugdev` groups but
leave `removeRule` at `auth_admin`, so undoing a decision raises an
authentication dialog even though making one does not. The Status page reports
this as **You may undo a permanent decision** and offers a drop-in polkit rule
that grants it to the same groups. That grant is not a widening in practice:
`appendRule` takes an insert position, so anyone who can already append a rule
can put an `allow` at the top of the policy — which is strictly more powerful
than removing one.

### Internal devices

Expand a device and turn on **Internal device** to say it is part of this
machine. It then drops out of the list (Settings has a **Show internal
devices** toggle) and is never asked about on a new boot.

This is a display and prompting preference, not an authorisation. The device
stays exactly as USBGuard has it, so allow it as well if it should work — the
Allow button stays available on its row. The mark is stored against the
descriptor hash, so a device that merely claims the same vendor and product IDs
does not inherit it.

### The Devices page

Every device, sorted so anything wanting attention is at the top. Click a row
to expand it and see the full details, with copy buttons for the fields you
might want to paste somewhere.

Root hubs and soldered-in devices are hidden by default — they are part of the
machine, not something you plugged in. So are devices you have marked as
internal. Turn any of them on in Settings. A device awaiting a decision is
always shown regardless of the filters.

Below the connected devices, a **Not connected** section lists anything that
has a standing rule but is not plugged in, reconstructed from the policy. These
rows carry the rule's target ("Always blocked", "Always rejected") rather than a
live authorisation, because there is no live device to report on. Rules that are
not pinned to a descriptor hash are left out: `allow id 1234:5678` describes
every device claiming those IDs rather than one device, so showing it as a
device with buttons would misrepresent what it does.

![The Status page, listing each check against the running system with its
observed value](docs/screenshots/status.png)

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

## Running a script when a device is connected

Expand a device on the **Devices** page and use **Run a program when
connected**. Give it a full path, optionally some arguments (one per line), and
a name so you can tell it apart later.

The classic case: plug in the backup drive, the backup starts.

```sh
#!/bin/sh
# ~/bin/backup.sh — USB Guard runs this once the drive is authorised.
set -eu
exec restic -r "/run/media/$USER/backup" backup "$HOME/Documents"
```

The program receives the device as environment variables, so one script can
serve several devices:

| Variable | Example |
| --- | --- |
| `USBGUARD_DEVICE_HASH` | `ifdabAPA9pgbVCvjqyUPQSihHiNCta+T9OWu2HOXKJQ=` |
| `USBGUARD_DEVICE_NAME` | `SanDisk 3.2Gen1` |
| `USBGUARD_DEVICE_ID` | `0781:55a9` |
| `USBGUARD_DEVICE_SERIAL` | `00002931052124091945` |
| `USBGUARD_DEVICE_PORT` | `1-2` |
| `USBGUARD_DEVICE_CLASSES` | `Mass storage` |

### What this feature will and will not do

This is the one part of the app that causes code to run, so the rules are
narrow and enforced in `src/hooks.rs` rather than left to the caller:

- **It only runs after the device is authorised.** A blocked, rejected or
  still-undecided device never runs anything. Without that, plugging in a
  device would execute code before you had agreed to trust it — which is the
  attack USBGuard exists to stop.
- **It is pinned to the descriptor hash**, the same identity a permanent rule
  uses. A hook keyed on the USB ID would fire for anything claiming those IDs,
  turning a spoofed vendor/product pair into arbitrary code execution.
- **There is no shell.** The program is executed directly with an argument
  vector; nothing the device reports is ever concatenated into a command line.
  A device whose name is `; rm -rf ~` is just a device with a silly name.
- **It is detached.** stdin, stdout and stderr go to `/dev/null` and the app
  does not wait, so a long backup cannot hold up the UI. If you want output,
  redirect it inside your own script.

The script still runs as you, with your privileges. Treat it exactly as you
would a cron job or a udev rule: it is your code, and USB Guard only decides
when to start it.

---

## When a device is refused without asking

A device with a standing `block` or `reject` rule never raises a prompt — that
is the point of a standing rule. The failure mode is that the drive simply does
not appear and nothing on screen says why.

USB Guard notifies instead, with a **Manage device** action that opens the
window at that device so you can change the rule. Turn it off with **Tell me
when a device is refused** if you would rather it stayed quiet.

---

## Settings

| Setting | Default | What it does |
| --- | --- | --- |
| Ask about new devices | On | Raise a decision prompt for a device with no standing rule |
| Send a desktop notification | On | Also notify, so a prompt is not missed when the panel is hidden |
| Open the window automatically | On | Let USB Guard open its own window when a decision is waiting |
| Tell me when a device is refused | On | Notify when a standing rule refuses a device without asking |
| Remember decisions by default | Off | Pre-tick "remember this decision" in the prompt |
| Show soldered-in devices | Off | Include devices USBGuard reports as hardwired |
| Show root hubs | Off | Include the host controllers the USB ports hang off |
| Show internal devices | Off | Include devices you have marked as part of this machine |
| Show disconnected devices | On | List devices that have a standing rule but are not plugged in |
| Highlight keyboard-capable devices | On | Call out devices that could inject keystrokes |
| Show the status icon | On | Display the shield in the system tray |
| Keep running when the window is closed | On | Closing the window leaves USB Guard watching |
| Start automatically at login | Off | Add a freedesktop autostart entry |
| Start without opening the window | Off | At login, show only the status icon |
| Keep a decision history | On | Record events to the journal |
| Warn about configuration problems | On | Show an alert when USBGuard is not set up to protect this system |

![The Settings page, showing the startup and status icon
options](docs/screenshots/settings.png)

Settings are stored through `cosmic-config`.

---

## Command-line options

| Option | Effect |
| --- | --- |
| `--minimized` | Start with the window hidden, showing only the status icon. What the autostart entry uses. |
| `--page <name>` | Open at `devices`, `history`, `status` or `settings`. An unrecognised name is ignored rather than fatal. |

---

## Translations

Twelve locales ship alongside English: Arabic, Chinese (Simplified), French,
German, Hindi, Italian, Japanese, Korean, Portuguese (Brazil), Russian,
Spanish and Turkish.

**They are machine-generated and have not been reviewed by native speakers.**
`tests/i18n.rs` proves every locale carries exactly the fallback's key set with
the same `{ $placeholders }` — it cannot prove the wording is right, and
several of these strings are security warnings. Corrections are welcome and
easy: edit `i18n/<locale>/cosmic_usb_guard.ftl` directly, or update the table
and re-run `scripts/i18n/generate.py`, which copies the English file's
structure and rejects any translation that drops or renames a placeholder.

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

**"Forget rule" or "Revoke" asks for an administrator password, or says the
removal was not authorised.**
Expected on a stock Debian, Ubuntu or Pop!\_OS install. The polkit rules shipped
with USBGuard grant `appendRule` and `applyDevicePolicy` to the `sudo` and
`plugdev` groups but not `removeRule`, so making a decision is unattended and
undoing one is not. Authenticate when asked, or take the drop-in rule the
Status page offers under **You may undo a permanent decision**, which grants
`removeRule` to the same groups.

**I clicked Allow, it looked like it worked, and the device was blocked again
after replugging.**
Fixed as of this version, but worth knowing why: USBGuard stops at the first
rule that matches, and a permanent decision is *appended* to the policy. An
`allow` rule written while a `block` or `reject` rule for the same device is
still present sits behind it and never fires. The app now removes the
conflicting rule before writing the new one. If you have an older policy with
both, `usbguard list-rules` will show them and the earlier one is the one in
force.

**Nothing prompts when I plug something in.**
Check `InsertedDevicePolicy` on the Status page. If it is `allow` or `keep`, the
device is authorised before we hear about it. Set it to `apply-policy`.

**Everything is blocked, including things I allowed before.**
Your policy is probably empty — check the Status page. Regenerate it with
`usbguard generate-policy` while your known-good devices are connected.

**The status icon does not appear.**
Something has to host it. On COSMIC that is the **Status Area** applet — check
it is in your panel under **Settings → Desktop → Panel**. On a desktop with no
StatusNotifierItem host at all, USB Guard says so and falls back to showing its
window. You can confirm a host is running with:

```sh
busctl --user list | grep StatusNotifierWatcher
```

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

- **USB Guard has to be running to prompt.** There is no separate background
  daemon; the app itself is what watches. Turn on **Start automatically at
  login** so it is there before you plug anything in, and leave **Keep running
  when the window is closed** on so closing the window does not stop it.
- **The translations are machine-generated and unreviewed.** Twelve locales
  ship, and `tests/i18n.rs` proves each has the same keys and placeholders as
  English — but not that the wording is right. Several strings are security
  warnings where a bad translation could mislead. Corrections are very welcome.
- **The hook editor is not screenshotted.** The three screenshots here were
  captured on a real session; driving the UI far enough to open the hook editor
  needed synthetic input that does not reach the window under Xwayland.

---

## How it works

```
  ┌──────────────────┐        ┌─────────────────────────┐
  │  tray status     │        │  window application     │
  │  icon (ksni,     │◀──────▶│  (devices, history,     │
  │  StatusNotifier) │        │   status, settings)     │
  └──────────────────┘        └───────────┬─────────────┘
                                          │
                          ┌───────────────┴───────────────┐
                          │  state, views, journal, hooks │
                          └───────────────┬───────────────┘
                                          │
                 ┌────────▼─────────┐
                 │  usbguard client │  zbus
                 └────────┬─────────┘
                          │  org.usbguard1 (system bus)
                 ┌────────▼─────────┐
                 │  usbguard-dbus   │
                 │  usbguard-daemon │  ← the thing that actually enforces
                 └──────────────────┘
```

One binary. The tray runs on its own D-Bus connection and reports through a
channel, so every state change still happens on the UI thread. Layering inside
the crate, innermost first:

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

The committed `Cargo.lock` pins `tinyvec` to 1.12.0. Version 1.13.0 does not
compile — it declares a `vec` module that shadows the `vec!` macro it uses —
and it reaches us transitively through libcosmic. If `cargo update` pulls it
forward again, `cargo update -p tinyvec --precise 1.12.0` puts it back.

The test suite includes checks that are designed not to pass vacuously:

- `tests/i18n.rs` asserts every locale carries exactly the fallback key set with
  identical placeholders, and fails if the fallback locale is missing rather
  than finding nothing to compare.
- `tests/icons.rs` asserts every icon name resolves against the installed icon
  themes, and first asserts that a control icon exists — so an uninstalled icon
  theme fails loudly instead of reporting green. A second check requires the
  names to resolve in a *baseline* theme (Adwaita or hicolor), because a
  Pop!\_OS or GNOME machine has icons that exist nowhere else; that is how
  `audio-card-usb-symbolic` passed locally and rendered as a blank square on a
  stock system.
- `src/hooks.rs` asserts a hook never runs for a device that is blocked or
  rejected, and then runs a real script and reads back what it saw — so a
  broken environment or argument vector fails in CI rather than silently at
  3 a.m. when the backup drive goes in.
- `src/autostart.rs` writes and removes a real autostart entry under a scratch
  `XDG_CONFIG_HOME`, so the test never touches your own session, and checks
  that a path containing a space is quoted rather than silently truncated.
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
