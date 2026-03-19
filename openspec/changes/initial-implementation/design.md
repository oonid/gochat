## Context

We are building a Google Chat desktop client using Tauri v2, inspired by three reference projects:
1. **google-chat-tauri** - Tauri v1 implementation, unmaintained, basic features only
2. **google-chat-electron** - Electron, unmaintained, feature-rich
3. **google-chat-linux** - Electron, actively maintained, most complete feature set

The goal is to combine the lightweight footprint of Tauri with the feature completeness of the Electron implementations, targeting cross-platform support (Linux, macOS, Windows).

**Key Technical Challenge**: Google Chat's web interface uses favicon changes to indicate message state. We need to monitor these changes via JavaScript injection and communicate them to the Rust backend for tray icon updates.

## Goals / Non-Goals

**Goals:**
- Build a production-ready Google Chat desktop client with Tauri v2
- Support system tray with dynamic icons reflecting message state
- Enable enterprise authentication via third-party OAuth/SSO
- Allow user customization via custom CSS injection
- Target binary size < 20MB and memory ~800-1000MB
- Cross-platform builds for Linux, macOS, Windows

**Non-Goals:**
- Mobile builds (iOS/Android)
- Custom backend/servers
- Modifying Google Chat's web interface beyond CSS injection
- Multi-account support (single account per session)
- Google Workspace API integration

## Decisions

### D1: Use Tauri v2 over v1

**Rationale**: Tauri v2 has significant improvements:
- Better system tray API (`TrayIconBuilder` vs deprecated `SystemTray`)
- Improved plugin system for single-instance, fs, persisted-scope
- Better webview2 (Windows) and WebKitGTK (Linux) integration
- Active development and security updates

**Alternative Considered**: Fork and upgrade existing google-chat-tauri (v1). Rejected because v1 is deprecated and the codebase is minimal - starting fresh with v2 patterns is cleaner.

### D2: Favicon Monitoring via JavaScript Injection

**Rationale**: Google Chat changes favicon dynamically. We inject JavaScript to:
1. Poll for favicon changes every 1.5 seconds
2. Emit events to Rust via Tauri's IPC
3. Rust updates tray icon based on favicon URL patterns

**Implementation**:
```javascript
// Injected script (simplified)
const FAVICON_SELECTORS = ['link#favicon256', 'link[rel="shortcut icon"]'];
setInterval(() => {
  const favicon = document.querySelector(FAVICON_SELECTORS.join(','));
  if (favicon?.href !== lastHref) {
    lastHref = favicon?.href;
    window.__TAURI__.event.emit('favicon-changed', favicon?.href);
  }
}, 1500);
```

**Alternative Considered**: Monitor network requests for favicon changes. Rejected because favicon is cached client-side and network monitoring is unreliable.

### D3: Configuration via JSON file

**Rationale**: Simple JSON file in user config directory for:
- Window bounds and state
- User preferences (icon theme, start hidden, auto-update)
- Third-party auth mode

**File Location**:
- Linux/macOS: `~/.config/gochat/config.json`
- Windows: `%APPDATA%\gochat\config.json`

**Schema**:
```json
{
  "bounds": { "x": 100, "y": 100, "width": 1200, "height": 800 },
  "maximized": false,
  "startHidden": false,
  "iconTheme": "default",
  "useTray": true,
  "autoUpdate": true,
  "thirdPartyAuthMode": false
}
```

**Alternative Considered**: Use Tauri's built-in store plugin. Rejected for simplicity - JSON file is sufficient for our needs and easier to debug/edit manually.

### D4: Icon Themes as Embedded Resources

**Rationale**: Embed three icon themes (default, colored, mono) in the binary using Tauri's resource system. Each theme has three states: normal, badge, offline.

**Structure**:
```
src-tauri/icons/
├── default/
│   ├── normal.png
│   ├── badge.png
│   └── offline.png
├── colored/
│   ├── normal.png
│   ├── badge.png
│   └── offline.png
└── mono/
    ├── normal.png
    ├── badge.png
    └── offline.png
```

**Alternative Considered**: Load icons from filesystem. Rejected to ensure icons are always available and reduce complexity.

### D5: Third-party Auth via URL Whitelist

**Rationale**: When third-party auth mode is enabled, we maintain a list of URLs that should navigate internally (not open in browser). This allows OAuth flows to complete.

**Default Whitelist**:
- `accounts.google.com`
- `accounts.youtube.com`
- `mail.google.com/ServiceLogin`
- `mail.google.com/chat`
- `chat.google.com`

**Custom URLs**: Via `NO_REDIRECT_URL` environment variable (comma-separated).

**Alternative Considered**: Detect OAuth flows automatically. Rejected because OAuth redirect patterns vary by provider - explicit whitelist is more reliable.

### D6: Custom CSS via File System Watch

**Rationale**: Load `custom.css` from config directory and inject into webview. File is read at startup and on page navigation.

**Alternative Considered**: Watch file for changes and hot-reload. Deferred - simple reload is sufficient for MVP.

### D7: Docker-based Development

**Rationale**: Use Docker to run Node.js/pnpm instead of requiring local Node.js installation. All frontend tooling runs via `scripts/dpnpm.sh` which wraps pnpm commands in a `node:20-bookworm-slim` container with persistent pnpm store volume.

**Workflow**:
- `./scripts/dpnpm.sh install` - Install dependencies
- `./scripts/dpnpm.sh build` - Build frontend
- `./scripts/dev.sh` - Run full dev environment (vite + tauri)

**Alternative Considered**: Require local Node.js installation. Rejected to simplify developer setup and ensure consistent Node.js version across environments.

## Risks / Trade-offs

### R1: Google may break favicon patterns → Mitigation
Favicon URL patterns are reverse-engineered. Google could change them.
- **Mitigation**: Make pattern matching configurable; fall back to "normal" state if pattern unrecognized

### R2: Third-party auth may not work for all providers → Mitigation
OAuth flows vary significantly between providers.
- **Mitigation**: Allow user to add custom URLs via environment variable; provide clear documentation

### R3: WebKitGTK (Linux) limitations → Mitigation
Linux uses WebKitGTK which has some limitations compared to Chromium.
- **Mitigation**: Test thoroughly on Linux; use feature detection; document known issues

### R4: Tauri v2 is relatively new → Mitigation
Tauri v2 was released recently and may have undiscovered issues.
- **Mitigation**: Pin versions; monitor Tauri releases for security patches; maintain ability to upgrade

## Open Questions

1. **Spellcheck languages**: Should we auto-detect from system locale or require explicit configuration? (Electron apps use explicit config)
   - **Decision**: Auto-detect from system, allow override via config

2. **Wayland support**: Should we explicitly support Wayland with `--ozone-platform-hint` equivalent?
   - **Decision**: Tauri/WebKitGTK handles Wayland automatically; no special handling needed

3. **Deep link protocol**: Register `gchat://` protocol handler for opening chat links from browser?
   - **Decision**: Yes, implement for all platforms (Linux: .desktop file, macOS: Info.plist, Windows: registry)
