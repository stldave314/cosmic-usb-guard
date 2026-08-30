# Roadmap

What is done, what is next, and what is deliberately out of scope.

This is a plan, not a promise. Items move when they turn out to be harder,
easier, or less useful than they looked.

---

## 0.1 — Core (current)

The thing works end to end: you get asked about a device, you decide, it is
recorded, and you can change your mind.

- [x] D-Bus client for `org.usbguard1`, generated from the interface definitions
      in `usbguard-dbus` rather than from guesswork
- [x] Rule-string parser with round-trip tests, including quoted values,
      escapes, and interface sets
- [x] Device model: USB IDs, name, serial, port, descriptor hash, interface
      classes, connection type
- [x] Self-healing event stream — reconnects with back-off, replays the full
      device list on every reconnect, and drops the stale list on disconnect
      rather than presenting it as current
- [x] Panel applet with a status-reflecting icon
- [x] Window application: Devices, History, Status, Settings
- [x] Decision prompt on insertion, with permanent-rule option pinned to the
      descriptor hash
- [x] Desktop notifications with inline Allow / Block / Details actions
- [x] Revoke that removes the standing rule *and* blocks the device
- [x] Keyboard-capable ("BadUSB") warnings, ranked above other classes
- [x] JSON Lines decision journal with rotation, and a History view over it
- [x] Installation health checks probed against the running system, each with a
      copyable fix-it command
- [x] Settings through `cosmic-config`, shared between both binaries
- [x] Localisation scaffolding with enforced locale parity
- [x] `.deb`, `.rpm` and tarball packaging, tag-triggered releases

## 0.2 — Filling the gaps

The known limitations, in rough order of how much they hurt.

- [ ] **Background watcher.** Today the applet must be in your panel for
      insertion prompts to happen at all. A small user service that watches
      USBGuard and raises the notification, with the applet and window as
      optional front-ends, would make the prompt reliable.
- [ ] **Policy editor.** View, reorder and remove the rules in
      `/etc/usbguard/rules.conf` from the Status page. Currently you can only
      remove rules indirectly, by revoking a device.
- [ ] **First-run setup assistant.** The USBGuard setup in the README is five
      manual steps and one of them can lock you out of your machine. Walking
      through it — generate a policy from what is connected, add the user, set
      `InsertedDevicePolicy`, enable both units — with a clear warning before
      the dangerous step, would be a much better introduction.
- [ ] **Search and filter on the Devices page**, for machines with a lot
      plugged in.
- [ ] **Screenshots in the README and AppStream metadata.**
- [ ] **Translations.** The machinery is in place and `tests/i18n.rs` enforces
      parity; it needs people. See [CONTRIBUTING.md](CONTRIBUTING.md).

## 0.3 — Sharper decisions

- [ ] **Better device identification.** Resolve vendor and product IDs to names
      via `usb.ids` when it is installed, falling back to what the device
      reports. A device's self-reported name is attacker-controlled; the USB-IF
      registry is not.
- [ ] **Device fingerprint changes.** Warn when a device presenting a
      previously-seen serial number now reports different interfaces — a strong
      signal that it is not the device you allowed.
- [ ] **Temporary allow.** Authorise until unplug or until a timer expires,
      without writing a rule. Useful for "I just need to copy one file".
- [ ] **Per-device notes.** A free-text label stored alongside the hash, so
      "the black drive from the conference" is identifiable a year later.
- [ ] **History export** to CSV and JSON, and a date-range filter.

## 0.4 — Policy-aware

- [ ] **Screen-lock integration.** Optionally block new devices while the
      session is locked, which is the case the physical-access threat model
      actually cares about. Needs care: it must not strand a user whose
      keyboard is USB.
- [ ] **Rule templates.** "Allow any keyboard from this vendor", "block all
      storage", expressed as USBGuard rules with a preview of what they would
      match before they are written.
- [ ] **Trusted ports.** Treat a specific port path as trusted, for a dock or
      an internal header.
- [ ] **Multi-seat awareness.** Currently the app assumes one interactive user.

## Not planned

Things that have come up and were decided against, with the reason, so they do
not get re-litigated.

- **Reimplementing USBGuard's enforcement.** This is a front-end. Authorisation
  belongs in a privileged daemon that has been reviewed; duplicating it in a
  GUI would be worse in every way.
- **Running the GUI as root.** It talks to the daemon over D-Bus with Polkit
  in between. That is the correct boundary.
- **A second configuration file for tuning values.** Implementation constants
  live in `src/constants.rs` at compile time. A runtime file for values that
  never change would add startup I/O, parsing, schema versioning, and a
  malformed-file failure mode for no benefit.
- **Flatpak.** A panel applet needs its desktop entry in the system scan path
  so `cosmic-panel` can find it, and needs to run un-sandboxed to reach the
  system bus. Native packages fit the app model; Flatpak does not.
- **Bundling a `usb.ids` copy.** If the system has one, use it. Shipping a
  snapshot means shipping a stale one.

---

## Contributing to the roadmap

Open an issue. If you want to take something on, say so on the issue first so
two people do not build it twice.
