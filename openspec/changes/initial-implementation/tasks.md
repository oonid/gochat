## 1. Project Setup

- [x] 1.1 Initialize Tauri v2 project with `pnpm create tauri-app` (manually created structure)
- [x] 1.2 Configure TypeScript and Vite for minimal frontend
- [x] 1.3 Set up Rust toolchain and verify Tauri CLI works
- [x] 1.4 Create initial `tauri.conf.json` with window and bundle settings
- [x] 1.5 Set up `.gitignore` and initial commit

## 2. Core Window and WebView

- [x] 2.1 Configure main window to load `https://mail.google.com/chat/u/0`
- [x] 2.2 Set up window title, icon, and initial dimensions (1200x800)
- [x] 2.3 Implement window show/hide on ready-to-show event
- [x] 2.4 Configure webview security settings (CSP, allowed domains)
- [x] 2.5 Add loading screen/splash while Google Chat loads

## 3. System Tray

- [ ] 3.1 Create tray icon with initial state (normal)
- [ ] 3.2 Build tray context menu (Show/Hide, Reload, Auth toggle, Quit)
- [ ] 3.3 Implement tray click to toggle window visibility
- [ ] 3.4 Add three icon states: normal, badge, offline
- [ ] 3.5 Create icon themes: default, colored, mono
- [ ] 3.6 Implement dynamic icon switching based on favicon state

## 4. Favicon Monitoring

- [ ] 4.1 Create JavaScript injection script for favicon monitoring
- [ ] 4.2 Implement IPC communication from JS to Rust for favicon changes
- [ ] 4.3 Add favicon URL pattern matching for icon state determination
- [ ] 4.4 Handle edge cases (no favicon, loading state, errors)

## 5. Desktop Notifications

- [ ] 5.1 Request notification permission on first launch
- [ ] 5.2 Integrate with Google Chat's notification API
- [ ] 5.3 Implement click-to-focus on notification click
- [ ] 5.4 Handle notification permission denied gracefully

## 6. Window Management

- [ ] 6.1 Implement single instance enforcement using Tauri plugin
- [ ] 6.2 Save window bounds on close (x, y, width, height)
- [ ] 6.3 Save and restore maximized state
- [ ] 6.4 Implement close-to-tray behavior (hide on close button)
- [ ] 6.5 Add start hidden configuration option
- [ ] 6.6 Handle second instance with window focus and optional deep link

## 7. External Link Handling

- [ ] 7.1 Implement URL scheme detection (internal vs external)
- [ ] 7.2 Open external URLs in default browser via `shell.open`
- [ ] 7.3 Clean Google redirect URLs before opening
- [ ] 7.4 Handle Google Meet links to open in browser
- [ ] 7.5 Preserve internal navigation within chat.google.com

## 8. Third-Party Authentication

- [ ] 8.1 Implement auth mode toggle (regular vs third-party)
- [ ] 8.2 Create URL whitelist for internal navigation during auth
- [ ] 8.3 Support `NO_REDIRECT_URL` environment variable
- [ ] 8.4 Handle app restart when toggling auth mode
- [ ] 8.5 Test with common SSO providers (Okta, Azure AD, etc.)

## 9. Custom CSS Injection

- [ ] 9.1 Create config directory structure if not exists
- [ ] 9.2 Load custom.css from config directory
- [ ] 9.3 Inject CSS into webview after page load
- [ ] 9.4 Create template custom.css file on first run
- [ ] 9.5 Handle CSS injection errors gracefully

## 10. Configuration System

- [ ] 10.1 Create config module with JSON file persistence
- [ ] 10.2 Define configuration schema with defaults
- [ ] 10.3 Implement config loading on startup
- [ ] 10.4 Implement config saving on changes and exit
- [ ] 10.5 Add configuration validation

## 11. Auto-Update

- [ ] 11.1 Configure Tauri updater with GitHub releases endpoint
- [ ] 11.2 Generate signing keys for updates
- [ ] 11.3 Implement update check on startup
- [ ] 11.4 Add manual update check option in menu
- [ ] 11.5 Handle update download and installation prompt

## 12. Deep Link Protocol

- [ ] 12.1 Register `gchat://` protocol handler
- [ ] 12.2 Convert `gchat://` URLs to `https://` for navigation
- [ ] 12.3 Handle incoming deep links from OS
- [ ] 12.4 Configure .desktop file (Linux), Info.plist (macOS), registry (Windows)

## 13. Cross-Platform Build

- [ ] 13.1 Configure Linux targets (deb, rpm, AppImage)
- [ ] 13.2 Configure macOS targets (dmg, app for Intel and ARM)
- [ ] 13.3 Configure Windows targets (msi, nsis)
- [ ] 13.4 Set up GitHub Actions for CI/CD
- [ ] 13.5 Test builds on all platforms

## 14. Documentation and Polish

- [ ] 14.1 Write README with installation instructions
- [ ] 14.2 Document configuration options
- [ ] 14.3 Add LICENSE file
- [ ] 14.4 Create app icons for all platforms
- [ ] 14.5 Final testing and bug fixes
