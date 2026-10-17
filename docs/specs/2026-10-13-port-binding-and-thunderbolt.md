# Port binding, Thunderbolt health checks and Thunderbolt devices

Status: approved 2026-10-13.

## Goal

Make a remembered USB decision as specific as the hardware allows, cover the
Thunderbolt/USB4 attack surface that USBGuard cannot see, and make every
configuration problem the app reports fixable from the app itself.

## 1. Bind a USB decision to its port

Every attribute in a USBGuard rule is reported by the device itself, so a
device built to impersonate another (same IDs, serial and descriptors) matches
its rules. The physical port is the one thing it cannot choose. USBGuard's
`via-port` attribute pins a rule to a port path such as `3-5` or `1-2.4`; the
path includes the hub chain.

The daemon's own permanent rules (`applyDevicePolicy` with `permanent=true`)
omit `via-port`, so a bound rule is written by the app: the device's live rule
text, retargeted, appended with `appendRule`, then the decision applied with
`applyDevicePolicy(permanent=false)`. Appending first means a failed write
changes nothing.

- **Decision prompt.** "Bind to this USB port (3-5)" switch under "Remember
  this decision". Enabled only while Remember is on; off by default.
- **Notification actions.** Never bind.
- **Device detail.** "Bound to USB port" switch for any device with a standing
  rule. Plugged in: rewrites the rule with or without `via-port`. Unplugged and
  bound: can unbind. Unplugged and unbound: disabled, "Plug the device in to
  bind it to its port."
- **Wrong port.** A bound device on another port does not match its rule and
  is blocked by the implicit policy. It is treated as needing a decision, and
  the prompt says "Bound to port A, now on port B".
- An "already recorded" decision compares the port as well as the target, so
  binding or unbinding is never skipped as a no-op.

## 2. Health checks with fixes

`Check` keeps its `remedy` (a shell command to copy) and gains `steps`: a
reference to translated, numbered instructions for problems only firmware
setup can fix. The Status page groups checks into USBGuard and Thunderbolt.

Thunderbolt checks run only when `/sys/bus/thunderbolt/devices/domain*`
exists. Kernel state is read from sysfs (authoritative, needs no permission);
bolt is reached over D-Bus.

| Check | Problem | Severity | Fix |
|---|---|---|---|
| Security level | `none` with DMA protection off | Critical | Steps (firmware) |
| Security level | `none` with DMA protection on | Warning | Steps (firmware) |
| Security level | unrecognized value | Warning | none |
| DMA protection | `iommu_dma_protection` is 0 | Warning | Steps (firmware) |
| bolt | not reachable on the system bus | Warning | install / enable command |
| Authorization | polkit challenge or refusal for `org.freedesktop.bolt.authorize`/`enroll` | Warning | drop-in polkit rule, written with `install -m 0644` |

`user`, `secure`, `dponly` and `usbonly` are all acceptable security levels.

## 3. Thunderbolt page

A "Thunderbolt" navigation entry, present only when a domain exists.

- **Summary.** Each domain's security level and DMA protection, and bolt's
  automatic-authorization mode when it is off (remembered devices then also
  wait to be allowed).
- **Devices.** bolt's devices, excluding hosts: name, vendor, state (waiting
  for authorization, authorized, not connected, authorization failed) and
  whether bolt remembers it.
- **Actions.** Allow once (`Device.Authorize`), Always allow
  (`Manager.EnrollDevice` with policy `default`, so bolt's own `DefaultPolicy`
  decides what happens next time; at `secure` level bolt stores a key),
  Forget (`Manager.ForgetDevice`). Thunderbolt has no way to revoke PCIe access
  from a connected device, so Forget takes effect at the next connection, and
  the UI says so.
- **Prompts.** A peripheral that arrives waiting for authorization raises an
  in-app prompt — remembered or not, since with bolt's automatic authorization
  off a remembered device waits too — and, if enabled in Settings, a
  notification with Allow once / Always allow / Details. Same settings as USB.
- **Updates.** bolt's `DeviceAdded`/`DeviceRemoved` and `PropertiesChanged`
  signals trigger a re-read; the periodic refresh is the safety net.

## Structure

- `src/thunderbolt/` — `proxy`, `client`, `model`, `sysfs`, `events`, `health`.
- `src/ui/thunderbolt.rs` — the page and its prompt card.
- State gains Thunderbolt devices, domains and pending prompts alongside the
  USB ones; the refresh and subscription loops feed both.

## Testing

Unit tests for rule editing, port comparison, sysfs and bolt value parsing,
severity mapping, and which actions are offered. Health checks and the domain
summary are verified against this machine. Authorize, enroll and forget need a
Thunderbolt or USB4 peripheral and are otherwise covered only by unit tests.
