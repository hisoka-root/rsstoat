# Changelog

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
