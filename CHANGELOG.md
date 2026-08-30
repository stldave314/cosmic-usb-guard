# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
