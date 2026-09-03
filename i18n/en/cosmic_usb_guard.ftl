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
refresh = Refresh
copy = Copy
open-app = Open USB Guard
quit = Quit

## Navigation

page-devices = Devices
page-history = History
page-status = Status
page-settings = Settings

## Device list

devices-heading = Connected devices
devices-none = No USB devices
devices-none-description = Nothing is connected, or USBGuard cannot see this system's USB bus.
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
device-internal-description = Part of this machine. Hidden from the list and never asked about. This does not authorise it — allow it as well if it should work.
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
remember-decision = Remember this decision

## Warnings

warning-input-capable = This device can act as a keyboard and type on your behalf.
warning-storage = This device exposes storage.
warning-network = This device presents a network adapter, which can reroute your traffic.
warning-standing-conflict = A standing rule will set this back to "{ $target }" the next time it is connected.
warning-no-hash = USBGuard did not report a descriptor hash for this device, so a permanent rule cannot be pinned to it specifically.

## Status and health

status-ok = USBGuard is protecting this system
status-warning = USBGuard is running with problems
status-critical = This system is not protected
status-disconnected = Not connected to USBGuard
status-disconnected-description = { $reason }
status-checking = Checking…

check-daemon-running = USBGuard service is running
check-daemon-enabled = USBGuard service starts at boot
check-dbus-running = USBGuard D-Bus interface is running
check-dbus-enabled = USBGuard D-Bus interface starts at boot
check-ipc-reachable = The daemon answers requests
check-ipc-permission = You may make policy decisions
check-decisions-reversible = You may undo a permanent decision
check-inserted-policy = New devices wait for a decision
check-policy-not-empty = A device policy is configured

check-observed = Observed: { $value }
remedy-heading = Fix it by running:

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
event-service-up = USBGuard became available
event-service-down = USBGuard became unavailable
event-health-problem = Configuration problem

actor-user = by you
actor-policy = by USBGuard policy
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
setting-show-hardwired-description = Include devices USBGuard reports as hardwired, which cannot be unplugged.
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
setting-warn-on-health-problems-description = Show an alert when USBGuard is not set up to protect this system.
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

section-behaviour = Behaviour
section-display = Display
section-startup = Startup and status icon
section-privacy = History

## Errors

error-service-unavailable = USBGuard is not running
error-permission-denied = Not permitted
error-autostart = Could not change the autostart setting: { $message }
error-no-tray = This desktop has no system tray, so the status icon could not be shown. USB Guard opened its window instead.
error-cannot-remove-rule = Removing a standing rule needs administrator authorisation, which was not given. See "You may undo a permanent decision" on the Status page.

## About

repository = Repository
support = Report an issue
version = Version { $version }
