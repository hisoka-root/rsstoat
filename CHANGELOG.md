# Changelog

## [1.4.0](https://github.com/hisoka-root/rsstoat/releases/tag/v1.4.0) (2026-06-23)

Match upstream Stoat v1.4.0 release.

### Features

* Screen sharing via native browser `getDisplayMedia()` picker
* Notification permission auto-granted to prevent "failed to enable" errors
* Windows taskbar badge overlay with unread count
* Exposed `onceScreenPicker` / `screenPickerCallback` bridge APIs

### Fixes

* CI: fixed `--bundles` flag not recognized by tauri build
* CI: fixed updater.json signature extraction (filtered pnpm noise)
* CI: switched to env vars for signing key + password
* CI: node 22, pnpm version from packageManager field

## [1.3.0](https://github.com/hisoka-root/rsstoat/releases/tag/v1.3.0) (2026-06-17)

Initial release of rsStoat — a Tauri v2 port of the Stoat desktop client.

### Features

* Desktop client for Stoat chat, built on Tauri v2
* Windows (MSI), macOS (DMG), and Linux (DEB) builds
* Discord Rich Presence integration
* System tray with minimize-to-tray support
* Window state persistence (size, position, maximized)
* Auto-launch on startup
* Badge count support
* Autoupdater support

### Changes from upstream

* Migrated from Electron to Tauri v2 (Rust backend)
* Switched from Flatpak/Snap to DEB packaging
* License changed from AGPL-3.0 to MPL-2.0
