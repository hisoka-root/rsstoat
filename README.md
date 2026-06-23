<div align="center">
  <img src="assets/icons/icon.png" alt="rsStoat Logo" width="128" height="128">
</div>

<div align="center">
<h1>
  rsStoat
</h1>

[![Version](https://img.shields.io/badge/version-1.4.0-blue.svg)](https://github.com/hisoka-root/rsstoat/releases)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Desktop client for [Stoat](https://stoat.chat) chat, built with [Tauri v2](https://v2.tauri.app).

**Version** 1.4.0 &middot; A port of the Stoat desktop client from **Electron** to **Tauri v2** (Rust backend, smaller bundles).

Available for **Windows** (MSI), **macOS** (DMG), and **Linux** (DEB, Arch).
</div>

**Assets are copyright &copy; hisoka.**

## Installation

Downloads can be found on the [releases page](https://github.com/hisoka-root/rsstoat/releases).

- **Debian/Ubuntu**: download the `.deb` and install with `sudo dpkg -i rsstoat_*.deb`
- **Arch Linux**: build from source using the `PKGBUILD` in this repo:
  ```bash
  makepkg -si
  ```

## Roadmap

- [x] Port from Electron to Tauri v2 (Rust backend)
- [x] Discord Rich Presence
- [x] System tray & minimize-to-tray
- [x] Auto-launch on startup
- [x] Window state persistence
- [x] Autoupdater
- [x] DEB packaging
- [x] Arch Linux PKGBUILD
- [ ] **Native frontend** — replace the embedded web URL with a local Tauri webview frontend, eliminating the dependency on `stoat.chat/app`

## Known Limitations

These are due to missing APIs in Tauri v2.11 and will be resolved once Tauri v2.12 ships.

| Feature | Status | Notes |
|---|---|---|
| Screen sharing (custom picker) | Native picker used | Tauri lacks `setDisplayMediaRequestHandler`. Screen sharing works via the native browser picker instead of Stoat's custom UI. |
| Auto-grant mic/camera | Prompt shown | Permission handler API lands in v2.12. Currently a browser permission prompt appears on first voice channel join. |
| macOS screen sharing | Needs entitlements | Requires `Info.plist` with camera/mic usage descriptions + entitlements. |

## Development

### Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/) (run `corepack enable`)

### Setup

```bash
git clone https://github.com/hisoka-root/rsstoat.git rsstoat
cd rsstoat

# install JS dependencies
pnpm install

# run in development mode
pnpm dev

# build for production
pnpm build
```
