## Why

Google shut down their official Google Chat desktop app in March 2021, forcing users to rely on browser-based PWA which lacks desktop integration features. Existing Electron-based wrappers are resource-heavy (150MB+, 1.2GB+ RAM) and the single Tauri-based alternative is unmaintained (last release v0.0.8 with Tauri v1). Users need a modern, lightweight, actively maintained desktop client with proper system integration for notifications, tray management, and enterprise authentication support.

## What Changes

Build a complete Google Chat desktop client from scratch using Tauri v2:

- **Core Application**: Cross-platform desktop wrapper loading `https://mail.google.com/chat/u/0`
- **System Tray**: Native tray icon with dynamic states (normal/badge/offline) indicating message status
- **Desktop Notifications**: Native OS notifications for new messages with click-to-focus
- **Third-party Auth**: Support for enterprise SSO/OAuth login flows that redirect to external identity providers
- **Custom CSS Injection**: User customization via `custom.css` file for theming
- **Single Instance**: Prevent multiple app instances from running
- **Window State Persistence**: Remember window position, size, and maximize state
- **External Link Handling**: Open external links in default browser
- **Spellcheck**: Native spellcheck support with configurable languages
- **Auto-updater**: Built-in update mechanism

## Capabilities

### New Capabilities

- `system-tray`: Native system tray integration with dynamic icon states (normal/unread/offline), show/hide window toggle, and context menu with quick actions
- `desktop-notifications`: Native OS notifications for new messages with click-to-navigate functionality
- `third-party-auth`: Enterprise SSO/OAuth support allowing authentication through external identity providers without breaking the app flow
- `custom-css`: User-configurable CSS injection for visual customization of the chat interface
- `window-management`: Window state persistence, single-instance enforcement, and close-to-tray behavior
- `external-links`: Intelligent URL handling that opens external links in the default browser while keeping Google Chat URLs in-app
- `auto-update`: Built-in auto-updater using Tauri's update mechanism with GitHub releases

### Modified Capabilities

(None - this is initial implementation)

## Impact

**New Dependencies**:
- Tauri v2.x runtime
- Rust 1.70+ for backend
- Node.js 18+ for frontend tooling (Vite)

**Target Platforms**:
- Linux (x64, arm64) - .deb, .rpm, AppImage
- macOS (x64, arm64) - .dmg, .app
- Windows (x64) - .msi, .exe

**Resource Footprint** (target):
- Binary size: < 20MB
- Memory usage: ~800-1000MB (comparable to existing Tauri implementation)
- Significantly lighter than Electron alternatives (~150MB binary, 1.2GB+ RAM)

**Code Structure**:
```
gochat/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # App entry, window setup
│   │   ├── tray.rs      # Tray icon management
│   │   ├── config.rs    # Settings persistence
│   │   ├── injection.rs # JS/CSS injection scripts
│   │   └── auth.rs      # Third-party auth handling
│   ├── icons/
│   │   ├── default/     # normal, badge, offline icons
│   │   ├── mono/        # monochrome theme
│   │   └── colored/     # colored theme
│   └── tauri.conf.json
├── src/                 # Frontend (minimal)
│   ├── main.ts          # Notification permissions
│   └── styles.css       # Loading screen
└── package.json
```
