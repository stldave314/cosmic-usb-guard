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
details = Details
close = Close
cancel = Cancel
dismiss = Dismiss
refresh = Refresh
retry = Retry
copy = Copy
copied = Copied
open-app = Open USB Guard
quit = Quit
back = Back

## Navigation

page-devices = Devices
page-history = History
page-status = Status
page-settings = Settings
about = About

## Device list

devices-heading = Connected devices
devices-none = No USB devices
devices-none-description = Nothing is connected, or USBGuard cannot see this system's USB bus.
devices-hidden = { $count } internal { $count ->
        [one] device
       *[other] devices
    } hidden
devices-pending = { $count } { $count ->
        [one] device is
       *[other] devices are
    } waiting for a decision
show-hidden = Show internal devices

## Device state

state-allowed = Allowed
state-blocked = Blocked
state-rejected = Rejected
state-unknown = Unknown
state-pending = Awaiting decision

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
remember-decision = Remember this decision
remember-decision-description = Write a permanent rule so this exact device is handled the same way next time.
revoke-heading = Revoke this device?
revoke-description = This removes the standing rule for { $name } and blocks it now.

## Warnings

warning-input-capable = This device can act as a keyboard and type on your behalf.
warning-storage = This device exposes storage.
warning-network = This device presents a network adapter, which can reroute your traffic.
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
check-inserted-policy = New devices wait for a decision
check-policy-not-empty = A device policy is configured

check-observed = Observed: { $value }
remedy-heading = Fix it by running:

## History

history-heading = Decision history
history-empty = Nothing recorded yet
history-empty-description = Device events and the decisions made about them will appear here.
history-clear = Clear history
history-cleared = History cleared
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

## Settings

setting-prompt-on-insert = Ask about new devices
setting-prompt-on-insert-description = Show a decision prompt when a device with no standing rule is connected.
setting-notify-on-insert = Send a desktop notification
setting-notify-on-insert-description = Also notify, so a prompt is not missed when the panel is hidden.
setting-auto-open-popup = Open the panel popup automatically
setting-auto-open-popup-description = Let the indicator open itself when a device needs a decision.
setting-default-permanent = Remember decisions by default
setting-default-permanent-description = Tick "remember this decision" ahead of time in the prompt.
setting-show-hardwired = Show internal devices
setting-show-hardwired-description = Include devices that are soldered in and cannot be unplugged.
setting-show-root-hubs = Show root hubs
setting-show-root-hubs-description = Include the host controllers that the USB ports hang off.
setting-warn-input-capable = Highlight keyboard-capable devices
setting-warn-input-capable-description = Call out devices that could inject keystrokes.
setting-journal-enabled = Keep a decision history
setting-journal-enabled-description = Record device events and decisions to { $path }.
setting-warn-on-health-problems = Warn about configuration problems
setting-warn-on-health-problems-description = Show an alert when USBGuard is not set up to protect this system.

section-behaviour = Behaviour
section-display = Display
section-privacy = History

## Errors

error-service-unavailable = USBGuard is not running
error-service-unavailable-description = Start the service, then try again.
error-permission-denied = Not permitted
error-permission-denied-description = Your user account is not allowed to change USB policy.
error-timeout = USBGuard did not respond in time
error-generic = Something went wrong: { $message }
error-action-failed = Could not { $action } { $name }: { $message }

## About

repository = Repository
support = Report an issue
version = Version { $version }
