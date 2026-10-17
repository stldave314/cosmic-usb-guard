# SPDX-License-Identifier: GPL-3.0-or-later
# English (fallback) translation for cosmic-usb-guard.
#
# This is the fallback locale. Every other locale under i18n/ must define
# exactly the same set of keys, with the same argument placeholders — Fluent
# falls back silently, so a missing key shows up as stray English at runtime
# rather than as a build error. `tests/i18n.rs` enforces that.

app-title = USB Guard
app-description = Review and control which USB devices may connect

## Generic actions

allow = Allow
block = Block
reject = Reject
revoke = Revoke
forget = Forget rule
details = Details
dismiss = Dismiss
allow-once = Allow once
always-allow = Always allow
refresh = Refresh
copy = Copy
copy-command = Copy command
open-app = Open USB Guard
quit = Quit

## Navigation

page-devices = Devices
page-thunderbolt = Thunderbolt
page-history = History
page-status = Status
page-settings = Settings

## Device list

devices-heading = Connected devices
devices-none = No USB devices
devices-none-description = Nothing is connected, or the USBGuard service cannot see this system's USB bus.
devices-hidden = { $count } { $count ->
        [one] device
       *[other] devices
    } hidden by your display settings
devices-pending = { $count } { $count ->
        [one] device is
       *[other] devices are
    } waiting for a decision
devices-remembered = Not connected
devices-remembered-description = Devices with a standing rule that are not plugged in. Change or remove the rule here.
device-internal = Internal device
device-internal-toggle = Internal device
device-internal-description = Part of this machine. Hidden from the list and never asked about. This does not authorize it — allow it as well if it should work.
device-internal-no-hash = This device reports no descriptor hash, so it cannot be marked as internal.

## Device state

state-allowed = Allowed
state-blocked = Blocked
state-rejected = Rejected
state-unknown = Unknown
state-pending = Awaiting decision
state-disconnected = Not connected

## Standing policy rules

standing-allow = Always allowed
standing-block = Always blocked
standing-reject = Always rejected
standing-other = Standing rule: { $target }
bound-port-toggle = Bound to USB port
bound-port-description = The rule applies only while the device is plugged into port { $port }. Anywhere else it is blocked and you are asked again.
bound-port-description-unbound = Make the rule apply only on the port the device is plugged into now. A device copying its identity would also have to be in that port.
bound-port-unplugged = Plug the device in to bind its rule to the port it is in.

## Device fields

field-name = Name
field-usb-id = USB ID
field-serial = Serial number
field-port = Port
field-hash = Descriptor hash
field-interfaces = Interfaces
field-connection = Connection
field-status = Status
field-none = Not reported

## Decision prompt

prompt-heading = Allow this device?
prompt-description = { $name } was just connected. It stays blocked until you decide.
notify-new-device = New USB device connected
notify-auto-blocked = USB device refused
notify-auto-blocked-body = { $name } was refused by a standing rule and is not available. Open USB Guard to change that.
notify-manage = Manage device
notify-thunderbolt = Thunderbolt device waiting
notify-thunderbolt-body = { $name } is connected, but cannot reach the computer's memory until you allow it.
remember-decision = Remember this decision
bind-to-port = Bind to this USB port ({ $port })
bind-to-port-description = Applies only on this port. Plugged in anywhere else, the device is blocked and you are asked again.
prompt-wrong-port = Its rule is bound to port { $bound }, but it is now plugged into port { $port }.

## Warnings

warning-input-capable = This device can act as a keyboard and type on your behalf.
warning-storage = This device exposes storage.
warning-network = This device presents a network adapter, which can reroute your traffic.
warning-standing-conflict = A standing rule will set this back to "{ $target }" the next time it is connected.
warning-overriding-rule = A broader rule earlier in the policy also matches this device and is checked first, so it may decide instead of this device's own rule: { $rule }
warning-no-hash = The USBGuard service did not report a descriptor hash for this device, so a permanent rule cannot be pinned to it specifically.

## Status and health

status-ok = This system is protected
status-warning = Protection has problems
status-critical = This system is not protected
status-disconnected = Not connected to the USBGuard service
status-disconnected-description = { $reason }
status-checking = Checking…

check-daemon-running = The USBGuard service is running
check-daemon-enabled = The USBGuard service starts at boot
check-dbus-running = The USBGuard D-Bus interface is running
check-dbus-enabled = The USBGuard D-Bus interface starts at boot
check-ipc-reachable = The daemon answers requests
check-ipc-permission = You may make policy decisions
check-decisions-reversible = You may undo a permanent decision
check-inserted-policy = New devices wait for a decision
check-policy-not-empty = A device policy is configured
check-thunderbolt-security = Thunderbolt devices must be allowed before they connect
check-thunderbolt-dma = Memory is protected from Thunderbolt devices
check-thunderbolt-service = The Thunderbolt service (bolt) is running
check-thunderbolt-permission = You may allow Thunderbolt devices
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt and USB4

check-observed = Observed: { $value }
observed-unknown = unknown
observed-no-bus = no connection to the system bus
observed-responding = responding
observed-granted = granted
observed-password = requires an administrator password
observed-refused = refused
observed-polkit-unavailable = polkit unavailable
observed-rules = rules in the policy: { $count }
observed-unreadable = could not be read
observed-running = running
observed-not-installed = not installed
observed-stopped = installed but not running
observed-on = on
observed-off = off
observed-not-reported = not reported
remedy-heading = Fix it by running:
remedy-explanation = Copy the command, paste it into a terminal and press Enter. It asks for your password and prints nothing when it succeeds. Press Refresh afterward to check that it worked.
remedy-policy-warning = Read this first: it allows every device that is plugged in right now. Unplug anything you do not recognize before running it.
steps-heading = Fix it in firmware setup:
steps-firmware-only = This setting is stored in the computer's firmware, so no command can change it.
steps-restart = Save your work and restart the computer.
steps-open-setup = While it starts, press the key that opens firmware setup. The key is usually shown on screen; common ones are F1 or Enter (Lenovo), F2 (Dell, ASUS, Acer and most others), F10 (HP) and Del (desktop boards).
steps-tb-find = Find the Thunderbolt or USB4 settings, usually under Security, Config or Advanced.
steps-tb-set = Set the security level to "User Authorization" or "Secure Connect". Avoid "No Security" and "Legacy".
steps-dma-set = Turn on "Kernel DMA Protection", or the option it depends on: "VT-d" on Intel, "AMD-Vi" or "IOMMU" on AMD. It is usually under Security, Virtualization or Advanced.
steps-save = Save the changes, exit, and let the computer start. Then press Refresh on this page.
steps-session-only = Polkit lets only the person at the computer use USBGuard: the active, local session. It refuses a session reached over SSH or remote desktop, and one that has been switched away from.
steps-session-heading = To fix it:
steps-session-local = Run USB Guard in the session at the computer itself.
steps-session-switch = If you switched users, switch back to this session.
steps-session-refresh = Press Refresh on this page.

## History

history-heading = Decision history
history-empty = Nothing recorded yet
history-empty-description = Device events and the decisions made about them will appear here.
history-clear = Clear history
history-entries = { $count } { $count ->
        [one] entry
       *[other] entries
    }
history-filter-all = All events
history-filter-decisions = Decisions only

event-inserted = Connected
event-removed = Disconnected
event-updated = Changed
event-allowed = Allowed
event-blocked = Blocked
event-rejected = Rejected
event-revoked = Revoked
event-service-up = The USBGuard service became available
event-service-down = The USBGuard service became unavailable
event-health-problem = Configuration problem

actor-user = by you
actor-policy = by the USBGuard service's rules
actor-external = outside this app
actor-system = automatic

## Hooks

hook-heading = Run a program when connected
hook-description = Runs only after this device is allowed. A blocked device never runs anything.
hook-program = Program
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = Arguments
hook-arguments-placeholder = One per line
hook-label = Name
hook-label-placeholder = Backup
hook-enabled = Enabled
hook-save = Save hook
hook-remove = Remove hook
hook-none = No program set
hook-problem-not-set = Choose a program to run.
hook-problem-not-absolute = Enter the full path, starting with a slash.
hook-problem-missing = That file does not exist.
hook-problem-not-executable = That file is not executable. Run: chmod +x
hook-variables = The program receives the device details as environment variables: { $names }.

## Settings

setting-prompt-on-insert = Ask about new devices
setting-prompt-on-insert-description = Show a decision prompt when a device with no standing rule is connected.
setting-notify-on-insert = Send a desktop notification
setting-notify-on-insert-description = Also notify, so a prompt is not missed when the panel is hidden.
setting-auto-open-popup = Open the panel popup automatically
setting-auto-open-popup-description = Let the indicator open itself when a device needs a decision.
setting-default-permanent = Remember decisions by default
setting-default-permanent-description = Tick "remember this decision" ahead of time in the prompt.
setting-show-hardwired = Show soldered-in devices
setting-show-hardwired-description = Include devices the USBGuard service reports as hardwired, which cannot be unplugged.
setting-show-root-hubs = Show root hubs
setting-show-root-hubs-description = Include the host controllers that the USB ports hang off.
setting-show-internal = Show internal devices
setting-show-internal-description = Include devices you have marked as part of this machine.
setting-show-disconnected = Show disconnected devices
setting-show-disconnected-description = List devices that have a standing rule but are not plugged in, so a decision can be changed without them.
setting-warn-input-capable = Highlight keyboard-capable devices
setting-warn-input-capable-description = Call out devices that could inject keystrokes.
setting-journal-enabled = Keep a decision history
setting-journal-enabled-description = Record device events and decisions to { $path }.
setting-warn-on-health-problems = Warn about configuration problems
setting-warn-on-health-problems-description = Show an alert when this system is not fully protected.
setting-notify-on-auto-block = Tell me when a device is refused
setting-notify-on-auto-block-description = Notify when a standing rule refuses a device without asking, so it does not just silently fail to work.
setting-show-tray-icon = Show the status icon
setting-show-tray-icon-description = Display the shield in the system tray. Turning this off does not stop the app watching for devices.
setting-autostart = Start automatically at login
setting-autostart-description = Add USB Guard to your session so it is watching before you plug anything in.
setting-start-minimized = Start without opening the window
setting-start-minimized-description = At login, show only the status icon.
setting-run-in-background = Keep running when the window is closed
setting-run-in-background-description = Closing the window leaves USB Guard watching. Turn this off and closing it quits, which also stops the prompts.

section-behaviour = Behavior
section-display = Display
section-startup = Startup and status icon
section-privacy = History

## Errors

error-service-unavailable = The USBGuard service is not running
error-permission-denied = Not permitted
error-autostart = Could not change the autostart setting: { $message }
error-no-tray = This desktop has no system tray, so the status icon could not be shown. USB Guard opened its window instead.
error-cannot-remove-rule = Removing a standing rule needs administrator authorization, which was not given. See "You may undo a permanent decision" on the Status page.
error-thunderbolt-permission = You are not allowed to authorize Thunderbolt devices. The Status page has a fix under "You may allow Thunderbolt devices".
error-bolt-unavailable = The Thunderbolt service (bolt) is not running. The Status page shows how to start it.
error-thunderbolt = The Thunderbolt service reported an error: { $message }

## About

repository = Repository
support = Report an issue
version = Version { $version }

## Thunderbolt page

tb-heading = Thunderbolt and USB4
tb-security = Security level
tb-level-none = None: every device connects without being asked about
tb-level-user = User authorization: each new device must be allowed
tb-level-secure = Secure connect: each new device must be allowed, and remembered devices prove who they are with a key
tb-level-dponly = DisplayPort only: no device gets PCIe access
tb-level-usbonly = USB only: no device gets PCIe access
tb-level-nopcie = PCIe disabled: no device gets PCIe access
tb-dma = DMA protection
tb-dma-on = On: devices cannot read memory they were not given
tb-dma-off = Off: an allowed device can read any memory
tb-auto-off = Automatic authorization is turned off in bolt, so remembered devices also wait to be allowed each time.
tb-never-asks = At this security level there is nothing to allow: devices are never asked about.
tb-has-problems = There is a problem with the Thunderbolt setup.
tb-open-status = Show how to fix it
tb-prompt-heading = Allow this Thunderbolt device?
tb-prompt-description = { $name } is connected. Until you allow it, it cannot reach the computer's memory or the PCIe bus.
tb-devices = Devices
tb-no-devices = No Thunderbolt or USB4 devices have been connected.
tb-remembered = Remembered
tb-not-remembered = Not remembered
tb-status-waiting = Waiting to be allowed
tb-status-authorized = Allowed
tb-status-error = Authorization failed
tb-status-working = Connecting…
tb-forget = Forget
tb-forget-note = A connected Thunderbolt device keeps its access until it is unplugged; there is no way to take it back sooner. Forgetting a device takes effect the next time it is connected.
