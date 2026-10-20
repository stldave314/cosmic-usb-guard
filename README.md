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

![The Devices page with a remembered device expanded: its fields in one aligned
column, the standing rule, and the switches that bind the rule to a USB port or
mark the device as internal](docs/screenshots/devices.png)

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
  device now. Blocking alone would leave a rule that re-authorizes it on the
  next replug, which is the trap that makes people think USBGuard is broken.
- **Lets you change your mind.** Every permanent decision can be taken back,
  including one made about a device that is no longer plugged in. Changing a
  decision *replaces* the old rule rather than adding a second one, because
  USBGuard stops at the first rule that matches — a new `allow` appended behind
  an existing `reject` would never be reached.
- **Binds a decision to a USB port, if you want.** Everything a USB device
  reports about itself — IDs, name, serial, descriptors — can be copied by a
  device built to impersonate it. The port it is plugged into cannot. Bind a
  remembered decision to its port and the device is only allowed there;
  anywhere else it is blocked and you are asked again, with the reason.
- **Covers Thunderbolt and USB4.** USBGuard governs USB, but a Thunderbolt or
  USB4 dock, eGPU or NVMe enclosure tunnels PCIe, which can read memory
  directly — and that is authorized by the kernel and bolt, not USBGuard. The
  Thunderbolt page shows the controller's security level and DMA protection,
  lists the devices bolt knows, and lets you allow one once, always, or forget
  it. A new device waiting to be allowed raises the same prompt and
  notification as a USB one.
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
  and never authorizes anything on its own.
- **Runs a script for a device you trust.** Attach a program to one specific
  device and it runs when that device is connected — a backup script when the
  backup drive goes in. It only ever runs *after* the device is authorized, it
  is pinned to the device's descriptor hash rather than its USB ID, and it is
  executed directly with an argument vector rather than through a shell.
- **Says so when a device is refused silently.** A device with a standing block
  or reject rule never raises a prompt, so from your side the drive just does
  not appear. A notification explains why and links straight to the device.
- **Lives in the system tray.** No panel applet to add: the app publishes a
  StatusNotifierItem, which COSMIC's Status Area picks up on its own. It can
  start with your session, minimized to the icon, and keep watching when you
  close the window.
- **Keeps a history.** Insertions, removals, and every decision — yours,
  another front-end's, the command line's, or USBGuard's own policy —
  are written to a JSON Lines journal you can read with ordinary tools.
- **Checks that the protection actually works.** Not that it is installed:
  that the USBGuard service is running, that its D-Bus interface is up, that
  your account may make decisions, that you may also *undo* one, that the
  policy is not empty, and that `InsertedDevicePolicy` is not set to `allow` —
  which authorizes every new device before anyone can be asked about it. On a
  machine with Thunderbolt or USB4 it also checks the security level, DMA
  protection, that bolt is running, and that you may authorize devices. Each
  failed check comes with its fix: a command to copy and paste, or, for
  settings that only exist in firmware, numbered steps.

## Why the status checks matter

A USBGuard install can be present, syntactically valid, and completely inert.
The service can be disabled. The D-Bus bridge can be missing, in which case
every GUI silently sees nothing. The policy can be empty. `InsertedDevicePolicy`
can be `allow`, which means a device is authorized the instant it is plugged
in — so a prompt would be asking permission for something that has already
happened.

Thunderbolt has its own version of the same problem. A security level of
`none` connects every device on sight, and without the IOMMU's DMA protection
an allowed device can read any memory it likes. Both are firmware settings, and
nothing on the desktop says which you have.

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

USB Guard talks to USBGuard through its D-Bus bridge, which runs as root and
asks **polkit** whether you may do each thing. (USBGuard's own IPC allow-list,
which `usbguard add-user` edits, is only for the `usbguard` command line; it
does not affect USB Guard.) Polkit lets the active, local session read devices
and the policy. Making a decision, writing a rule and removing one need an
administrator password by default:

- **Debian, Ubuntu and Pop!\_OS** already grant making a decision and writing a
  rule to members of `sudo` and `plugdev`, but not removing one, which is what
  undoing a decision needs.
- **Elsewhere**, all three ask for a password.

Being asked for a password before a device is authorized is a sound setup,
and the Status page treats it as one: it never offers to make deciding
passwordless. The one thing it does offer a fix for is the Debian and Ubuntu
mismatch, where deciding is free but undoing asks for an administrator — see
[Changing a decision](#changing-a-decision).

**3. Hold new devices for a decision instead of auto-authorizing them.**

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
| Shield, full | Every check passes and nothing is outstanding |
| Shield, half | A device is waiting for a decision, or a check is failing |
| Shield, low | Not connected to the USBGuard service, or this machine is not protected |

A pending decision never shows the all-clear icon, even when everything else is
healthy — you still have to act.

### Deciding about a device

When a device is plugged in, the popup opens with a card for it. **Allow**
authorizes it, **Block** leaves it de-authorized, **Dismiss** stops asking
without changing anything (the device stays blocked, because that is what
USBGuard already did).

**Remember this decision** writes a permanent rule so the same device is handled
the same way next time. It is off by default, deliberately: a standing rule is
the more consequential of the two choices and should be a deliberate act. You
can change that default in Settings.

**Bind to this USB port** appears under it while Remember is on, naming the
port the device is in. See [Binding a decision to a port](#binding-a-decision-to-a-port).

Permanent rules are pinned to the device's **descriptor hash**, not its USB ID.
A rule written as `allow id 0781:5567` authorizes anything claiming those IDs,
which is exactly what a spoofing device relies on. The hash identifies the
specific device.

### What a rule matches on

The rule USBGuard writes for a remembered decision requires every one of these
to match:

| Attribute | What it is |
| --- | --- |
| `id` | Vendor and product ID |
| `serial` | Serial number |
| `name` | Product name |
| `hash` | A hash of the name, IDs, serial **and** the raw USB descriptors |
| `parent-hash` | The hash of the hub or controller it is plugged into |
| `with-interface` | Every interface class it exposes |
| `with-connect-type` | How it is connected, e.g. `hotplug` |
| `via-port` | The physical port — **only when you bind it** |

`parent-hash` already means the same device plugged into a port on a different
USB controller is asked about again. What none of the self-reported attributes
can do is stop a device built to copy a specific one: USB has no cryptographic
device identity, so a device that clones another's serial and descriptors
matches its rule. Binding the port closes that gap as far as USB allows — a
clone also has to be plugged into the same port.

### Changing a decision

Answered "Block" when you meant "Allow"? The device keeps a row for as long as
it has a standing rule, whether or not it is still plugged in — look under
**Not connected** if it is unplugged. Two controls take a decision back:

- **Allow** writes the opposite decision. Where a standing rule already
  contradicts it, the old rule is removed first and the decision is treated as
  permanent regardless of the "remember" toggle, since a one-off answer would
  be undone on the next replug. It is not offered for an unplugged device
  whose rule already says "Always allowed": the rule is the whole decision
  there, and it is already in place.
- **Forget rule** removes the standing rule and changes nothing else. The
  device falls back to USBGuard's implicit target, which is to block and ask —
  so you get the question again rather than having to commit to the opposite
  answer.

Removing a rule is the one operation most distributions do not grant without a
password. The polkit rules Debian and Ubuntu ship for USBGuard give
`appendRule` and `applyDevicePolicy` to the `sudo` and `plugdev` groups but
leave `removeRule` at `auth_admin`, so undoing a decision raises an
authentication dialog even though making one does not. The Status page reports
this as **You may undo a permanent decision** and offers a drop-in polkit rule,
`/etc/polkit-1/rules.d/50-usbguard-remove-rule.rules`, that grants removal —
and only removal — to active local members of `sudo`, `wheel` and `plugdev`.
It is only offered when your session can already make decisions without a
password, because only then is it no widening: `appendRule` takes an insert
position, so anyone who can append a rule can put an `allow` at the top of the
policy, which is strictly more powerful than removing one. Where making a
decision asks for a password too, undoing one asking for the same is
consistent, and the check passes.

Version 0.3.0 offered a broader rule, `50-usbguard-desktop.rules`, that also
made deciding passwordless. If you applied it, the current command removes
it, or remove it yourself:
`sudo rm /etc/polkit-1/rules.d/50-usbguard-desktop.rules`.

### Binding a decision to a port

Two places offer it:

- **The decision prompt**, as **Bind to this USB port (3-5)** under "Remember
  this decision". It needs Remember on, because a one-off decision writes no
  rule to bind.
- **The device details**, as **Bound to USB port**, for any device with a
  standing rule. Turning it on rewrites the rule with the port the device is
  in now; turning it off removes the port. A device that is not plugged in can
  be released but not bound, since there is no current port to bind it to.

Ports are named the way the kernel names them: `3-5` is port 5 on bus 3, and
`1-2.4` is port 4 of a hub on port 2 of bus 1, so moving a hub moves the
device's port too. Plugged in anywhere else, a bound device does not match its
rule and is blocked; the prompt then says **Its rule is bound to port 3-5, but
it is now plugged into port 1-2**, so the second prompt does not look like a
bug. Notification buttons never bind — that is a choice made in the window,
where the port is shown.

USBGuard's own permanent rules never include the port, so a bound rule is
written by the app: the device's rule with `via-port` added, appended to the
policy, and the decision then applied without a second rule.

### Internal devices

Expand a device and turn on **Internal device** to say it is part of this
machine. It then drops out of the list (Settings has a **Show internal
devices** toggle) and is never asked about on a new boot.

This is a display and prompting preference, not an authorization. The device
stays exactly as USBGuard has it, so allow it as well if it should work — the
Allow button stays available on its row. The mark is stored against the
descriptor hash, so a device that merely claims the same vendor and product IDs
does not inherit it.

### The Devices page

Every device, sorted so anything wanting attention is at the top. Click a row
to expand it and see the full details, with copy buttons for the fields you
might want to paste somewhere.

Root hubs and soldered-in devices are hidden by default — they are part of the
machine, not something you plugged in. A root hub is recognized by its vendor
ID *and* by sitting on a bus's root port, since the vendor ID alone is
something any device can claim. So are devices you have marked as
internal. Turn any of them on in Settings. A device awaiting a decision is
always shown regardless of the filters.

Below the connected devices, a **Not connected** section lists anything that
has a standing rule but is not plugged in, reconstructed from the policy. These
rows carry the rule's target ("Always blocked", "Always rejected") rather than a
live authorization, because there is no live device to report on. Rules that are
not pinned to a descriptor hash are left out: `allow id 1234:5678` describes
every device claiming those IDs rather than one device, so showing it as a
device with buttons would misrepresent what it does.

![The Status page, listing each check against the running system with its
observed value, grouped under USB and Thunderbolt](docs/screenshots/status.png)

### The Thunderbolt page

Shown only on machines with a Thunderbolt or USB4 controller.

![The Thunderbolt page, showing the controller's security level and DMA
protection, and a note that bolt's automatic authorization is
off](docs/screenshots/thunderbolt.png)

The top card says what the controller lets through. Under the `user` and
`secure` security levels each new device has to be allowed before it gets PCIe
access; `dponly`, `usbonly` and `nopcie` never tunnel PCIe at all; `none`
connects everything without asking, and the Status page explains how to change
it in firmware. **DMA protection** on means an allowed device still cannot read
memory it was not given.

Each device bolt knows has up to three actions:

- **Allow once** authorizes it for this connection only.
- **Always allow** authorizes it and has bolt remember it. bolt then authorizes
  it by itself next time, according to its default policy — unless bolt's
  automatic authorization is off, in which case the page says so and remembered
  devices wait to be allowed like new ones.
- **Forget** removes bolt's record of it.

Thunderbolt cannot take access back from a connected device: the kernel has no
way to revoke PCIe from it while it is plugged in. Forgetting a device takes
effect the next time it is connected, and the page says so.

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
# ~/bin/backup.sh — USB Guard runs this once the drive is authorized.
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

- **It only runs after the device is authorized.** A blocked, rejected or
  still-undecided device never runs anything. Without that, plugging in a
  device would execute code before you had agreed to trust it — which is the
  attack USBGuard exists to stop.
- **It is pinned to the descriptor hash**, the same identity a permanent rule
  uses. A hook keyed on the USB ID would fire for anything claiming those IDs,
  turning a spoofed vendor/product pair into arbitrary code execution.
- **There is no shell.** The program is executed directly with an argument
  vector; nothing the device reports is ever concatenated into a command line.
  A device whose name is `; rm -rf ~` is just a device with a silly name.
- **It must be a full path.** A program given as a bare name is refused when
  it would run, rather than looked up on `$PATH`, so what runs cannot depend
  on the environment the app happened to start in.
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
| Show soldered-in devices | Off | Include devices the USBGuard service reports as hardwired |
| Show root hubs | Off | Include the host controllers the USB ports hang off |
| Show internal devices | Off | Include devices you have marked as part of this machine |
| Show disconnected devices | On | List devices that have a standing rule but are not plugged in |
| Highlight keyboard-capable devices | On | Call out devices that could inject keystrokes |
| Show the status icon | On | Display the shield in the system tray |
| Keep running when the window is closed | On | Closing the window leaves USB Guard watching |
| Start automatically at login | Off | Add a freedesktop autostart entry |
| Start without opening the window | Off | At login, show only the status icon |
| Keep a decision history | On | Record events to the journal |
| Warn about configuration problems | On | Show an alert when this system is not fully protected |

![The Settings page, showing the startup and status icon
options](docs/screenshots/settings.png)

Settings are stored through `cosmic-config`.

---

## Command-line options

| Option | Effect |
| --- | --- |
| `--minimized` | Start with the window hidden, showing only the status icon. What the autostart entry uses. |
| `--page <name>` | Open at `devices`, `thunderbolt`, `history`, `status` or `settings`. An unrecognized name is ignored rather than fatal, and `thunderbolt` is ignored on a machine without Thunderbolt or USB4. |
| `--device <hash>` | Open with this device's details expanded. The hash is the **Descriptor hash** shown, and copyable, in the details. |

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

**The Status page says "Not connected to the USBGuard service".**
The D-Bus bridge is the usual cause. `sudo systemctl enable --now
usbguard-dbus.service`. It is a separate unit from `usbguard.service` and is not
enabled by default on most distributions.

**Allow asks for a password every time, or says it was not permitted.**
Polkit decides this, not USBGuard. Asking for a password is the upstream
default and a reasonable one — whoever is at the keyboard proves who they are
before a device is let in — so the Status page counts it as fine. Debian,
Ubuntu and Pop!\_OS let `sudo` and `plugdev` members decide without one; on
other distributions, an administrator can add a polkit rule for
`org.usbguard.Devices1.applyDevicePolicy` and `org.usbguard.Policy1.appendRule`
if they decide the trade is worth it. If it says *refused*, polkit is refusing
this session outright; see the next entry. `usbguard add-user` does not help
here — it changes who may use the `usbguard` command line, not what polkit
allows the desktop.

**The Status page says reading was refused and shows "To fix it" steps.**
Polkit lets only the active, local session read from USBGuard. Over SSH, in a
remote desktop session, or in a session you have switched away from, it
refuses. Use USB Guard in the session at the computer and press Refresh.

**"Forget rule" or "Revoke" asks for an administrator password, or says the
removal was not authorized.**
Expected on a stock Debian, Ubuntu or Pop!\_OS install. The polkit rules shipped
with USBGuard grant `appendRule` and `applyDevicePolicy` to the `sudo` and
`plugdev` groups but not `removeRule`, so making a decision is unattended and
undoing one is not. Authenticate when asked, or take the drop-in rule the
Status page offers under **You may undo a permanent decision**, which grants
removal alone.

**I ran the command from the Status page, it printed nothing, and rule removal
still asks for a password.**
Printing nothing is normal; check whether polkit actually loaded the rule:

```sh
journalctl -u polkit -b | grep usbguard-remove-rule
```

`Error loading script` means polkitd cannot read the file. Earlier versions of
this app suggested writing it with `sudo tee`, which creates the file with
whatever permissions your umask leaves; with a restrictive umask it comes out
`0640 root:root`, and polkitd, which does not run as root, cannot open it.
Copy the command from the Status page again: it now uses `install -m 0644`,
which replaces the file with a readable one.

Fixing the permissions with `chmod` alone is not enough. Polkit reloads its
rules when a file in that directory is created, changed or deleted, but a
permission change is only an attribute change, so polkit keeps the failed load
until something else prompts a reload. Replacing the file, as `install` does,
is what triggers it. Press Refresh on the Status page afterward.

**I clicked Allow, it looked like it worked, and the device was blocked again
after replugging.**
Fixed as of this version, but worth knowing why: USBGuard stops at the first
rule that matches, and a permanent decision is *appended* to the policy. An
`allow` rule written while a `block` or `reject` rule for the same device is
still present sits behind it and never fires. The app now removes the
conflicting rule before writing the new one. If you have an older policy with
both, `usbguard list-rules` will show them and the earlier one is the one in
force.

**A device I bound to a port is blocked again.**
It is in a different port. The prompt names both; check the port path shown
in its details. A hub counts: plugging the hub into another port changes the
port of everything on it. Allow it there with **Bind to this USB port** on to
move the binding, or turn **Bound to USB port** off in the details.

**A Thunderbolt dock or enclosure does not work.**
Open the **Thunderbolt** page. A device marked **Waiting to be allowed** has no
PCIe access until you choose **Allow once** or **Always allow**. If the page
says automatic authorization is off, remembered devices wait too: bolt was
told not to authorize anything by itself. `boltctl config auth-mode enabled`
turns that back on.

**The Status page says to change something in firmware setup.**
The Thunderbolt security level and DMA protection are stored in the
computer's firmware, so no command can change them; the check shows numbered
steps instead. Menu names vary by manufacturer, but the settings are usually
under Security, Config or Advanced, and the options read "User
Authorization" / "Secure Connect" and "Kernel DMA Protection" / "VT-d" /
"AMD-Vi".

**Nothing prompts when I plug something in.**
Check `InsertedDevicePolicy` on the Status page. If it is `allow` or `keep`, the
device is authorized before we hear about it. Set it to `apply-policy`.

**Everything is blocked, including things I allowed before.**
Your policy is probably empty — check the Status page. Its fix regenerates the
policy with `usbguard generate-policy` and restarts the daemon. That allows
**every device plugged in at that moment**, so unplug anything you do not
recognize first, and leave connected only the devices you want to keep
working.

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
`POLICY`, `HEALTH`, `UI`, `CONFIG`, `JOURNAL`, `NOTIFY`, `TRAY`, `HOOK`,
`TBOLT`) so you can `grep` a run.

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
- **The hook editor is not screenshotted.** The screenshots here were captured
  on a real session, opened at each page with `--page` and at a device with
  `--device`; opening the hook editor needs a click, and synthetic input does
  not reach the window under Xwayland.
- **Thunderbolt authorization is tested without hardware.** The Thunderbolt
  page, its health checks and the security-level and DMA readings are verified
  against a real Thunderbolt 4 controller, but allowing, remembering and
  forgetting a device are covered by unit tests only, since none was plugged
  in when they were written.
- **Port binding is only as stable as the port path.** The kernel's port names
  include the hub chain, so a docking station or hub that enumerates
  differently, or is moved, changes the port of everything on it.
- **Port binding assumes USBGuard's default rule generation.** With
  `DeviceRulesWithPort=true` in `usbguard-daemon.conf`, USBGuard writes the
  port into every rule it generates, so every remembered decision comes out
  bound whether or not you chose to bind it. The app cannot read that file to
  tell; leave the option at its default of `false`.
- **One rule per device.** For a device with several rules pinned to its hash
  — for example one per port, written by hand — the app shows and edits the
  first, which is the one USBGuard checks first. A rule pinning a *set* of
  hashes (`hash one-of { … }`) is left alone rather than treated as one
  device's.
- **Broader rules are warned about, not moved.** A rule such as
  `allow id 0781:5567` placed before a device's own rule is checked first and
  can overrule it. The device details say so and quote the rule; reorder or
  remove it in `/etc/usbguard/rules.conf` if it should not apply.
- **"Always allow" for Thunderbolt trusts the device's own identity at the
  `user` security level.** bolt remembers a device by a unique ID that the
  device reports, so a device that copies another's ID is authorized like it.
  The `secure` level adds a key challenge for remembered devices; choose it in
  firmware setup if your hardware offers it.
- **Port binding is tested without a live device.** Writing, comparing and
  rewriting bound rules is covered by unit tests, and the switch's states were
  checked in the running app, but no bound rule has yet been written to a live
  daemon by this version.
- **The decision history needs a home directory.** With neither `HOME` nor
  `XDG_DATA_HOME` set, nothing is recorded rather than writing it somewhere
  another user could reach.

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
                 ┌────────▼─────────┐     ┌────────────────────┐
                 │  usbguard client │     │  thunderbolt client│  zbus + sysfs
                 └────────┬─────────┘     └─────────┬──────────┘
                          │  org.usbguard1          │  org.freedesktop.bolt
                 ┌────────▼─────────┐     ┌─────────▼──────────┐
                 │  usbguard-dbus   │     │  boltd             │
                 │  usbguard-daemon │     │  kernel thunderbolt│  ← what actually
                 └──────────────────┘     └────────────────────┘     enforces
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
| `thunderbolt::sysfs` | Reads the kernel's Thunderbolt security level and DMA protection |
| `thunderbolt::model`, `client`, `events` | bolt's devices and domains, its actions, and a change stream from its signals |
| `thunderbolt::health` | Judges the Thunderbolt setup and says how to fix it, including in firmware |
| `state` | The model both binaries render, and its transitions |
| `ui` | Shared views, emitting a common `Action` |
| `journal` | The append-only decision log |

This crate enforces nothing. USBGuard and the kernel's Thunderbolt subsystem do.
If this app is not running, your policy is exactly as it was.

---

## Building

```sh
cargo build --release --features release-build
cargo test
cargo clippy --all-targets
```

`tinyvec` 1.13.0 did not compile — it declared a `vec` module that shadowed
the `vec!` macro it uses — and reaches this crate transitively through
libcosmic, so `Cargo.lock` used to pin 1.12.0. 1.13.3 builds, and the pin is
gone; if a later release breaks again, `cargo update -p tinyvec --precise
<version>` holds it back.

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
  stock system. A third check resolves every name against the theme COSMIC
  actually renders with — Cosmic, the themes it inherits directly, and
  hicolor — because a name that exists only in Adwaita passes the baseline
  check and still renders blank on COSMIC; that is how `thunderbolt-symbolic`
  reached the navigation bar invisible. Without the COSMIC theme installed it
  fails rather than skipping, unless `ICON_TEST_WITHOUT_COSMIC=1` says that is
  expected (CI sets it).
- `src/state.rs` measures the real journal file before and after a test records
  an event, because a test used to append a fabricated "daemon failed" entry
  to the user's own history on every run.
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

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

USBGuard itself is a separate project, licensed GPL-2.0-or-later.
