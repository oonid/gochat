# GoChat

A lightweight, cross-platform desktop client for Google Chat built with Tauri v2.

## Features

- **System Tray Integration**: Native tray icon with dynamic states (normal/unread/offline)
- **Desktop Notifications**: Native OS notifications with click-to-focus
- **Third-party Authentication**: Support for enterprise SSO/OAuth login flows
- **Custom CSS Injection**: User customization via `custom.css` file
- **Window State Persistence**: Remembers position, size, and maximize state
- **Single Instance**: Prevents multiple app instances
- **External Link Handling**: Opens external links in default browser
- **Auto-updater**: Built-in update mechanism
- **Deep Link Protocol**: `gchat://` protocol handler

## Installation

### Linux

Download from [Releases](https://github.com/oonid/gochat/releases):

| Package | Description |
|---------|-------------|
| `.deb` | Debian/Ubuntu package |
| `.rpm` | Fedora/RHEL package |
| `.AppImage` | Portable, no installation required |

```bash
# Debian/Ubuntu
sudo dpkg -i gochat_0.1.0_amd64.deb
sudo apt-get install -f  # Install dependencies

# Fedora
sudo rpm -i gochat-0.1.0.x86_64.rpm

# AppImage
chmod +x gochat_0.1.0_amd64.AppImage
./gochat_0.1.0_amd64.AppImage
```

**Dependencies**: `libwebkit2gtk-4.1-0`

### macOS

Download from [Releases](https://github.com/oonid/gochat/releases):

| Package | Architecture |
|---------|-------------|
| `.dmg` (Intel) | x86_64 |
| `.dmg` (Apple Silicon) | arm64 |

```bash
# Open DMG and drag to Applications
open gochat_0.1.0_x64.dmg
# or
open gochat_0.1.0_aarch64.dmg
```

### Windows

Download from [Releases](https://github.com/oonid/gochat/releases):

| Package | Description |
|---------|-------------|
| `.msi` | Windows Installer |
| `.exe` | NSIS Installer |

```powershell
# MSI
msiexec /i gochat_0.1.0_x64.msi

# NSIS
.\gochat_0.1.0_x64-setup.exe
```

### Portable Binaries

Standalone binaries are also available:

- `gochat-{version}-linux-x64.tar.gz`
- `gochat-{version}-macos-x64.zip`
- `gochat-{version}-macos-arm64.zip`
- `gochat-{version}-windows-x64.zip`

Note: Portable binaries require system dependencies to be installed.

## Running from Source

### Prerequisites

- **Rust** 1.70+
- **Docker** (for running without Node.js) **OR** **Node.js** 18+ with **pnpm**

#### Linux System Dependencies

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Option 1: Using Docker (No Node.js Required)

Run without installing Node.js/pnpm locally:

```bash
# Clone repository
git clone https://github.com/oonid/gochat.git
cd gochat

# Run in development mode (uses Docker for frontend)
./scripts/dev.sh
```

The `dev.sh` script:
1. Starts Vite dev server in a Docker container
2. Runs `cargo tauri dev` to launch the app

Additional Docker scripts:

```bash
./scripts/vite.sh start   # Start Vite container
./scripts/vite.sh stop    # Stop Vite container
./scripts/vite.sh status  # Check container status
./scripts/vite.sh logs    # View container logs
./scripts/dpnpm.sh install    # Run pnpm install in Docker
./scripts/dpnpm.sh build      # Run pnpm build in Docker
```

### Option 2: Using Local Node.js

If you have Node.js and pnpm installed:

```bash
# Clone repository
git clone https://github.com/oonid/gochat.git
cd gochat

# Install dependencies
pnpm install

# Development mode
pnpm tauri dev

# Build release
pnpm tauri build
```

### Build Release Binaries

```bash
# With Docker
./scripts/dpnpm.sh install
cargo tauri build

# With local pnpm
pnpm tauri build
```

## Configuration

Configuration file location:

- Linux/macOS: `~/.config/gochat/config.json`
- Windows: `%APPDATA%\gochat\config.json`

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `bounds` | object | `{x, y, width, height}` | Window position and size |
| `maximized` | boolean | `false` | Window maximized state |
| `startHidden` | boolean | `false` | Start minimized to tray |
| `iconTheme` | string | `"default"` | Tray icon theme: `default`, `colored`, `mono` |
| `useTray` | boolean | `true` | Enable system tray |
| `autoUpdate` | boolean | `true` | Enable auto-updates |
| `thirdPartyAuthMode` | boolean | `false` | Enable third-party auth for SSO |

### Custom CSS

Create `~/.config/gochat/custom.css` to customize the Google Chat interface:

```css
/* Example: Dark theme tweaks */
body {
  background-color: #1a1a1a;
}
```

### Third-party Authentication

For enterprise SSO that redirects to external identity providers:

1. Enable via tray menu: **Auth Mode > Third-party**
2. Or set environment variable:
   ```bash
   export NO_REDIRECT_URL="accounts.google.com,accounts.youtube.com,your-sso-provider.com"
   ```

## Deep Links

GoChat registers the `gchat://` protocol. Open chat links from browser:

```
gchat://chat.google.com/dm/your-conversation-id
```

## Development

### Project Structure

```
gochat/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # App entry, window setup
│   │   ├── tray.rs      # Tray icon management
│   │   ├── config.rs    # Settings persistence
│   │   ├── injection.rs # JS/CSS injection
│   │   └── auth.rs      # Third-party auth
│   └── tauri.conf.json
├── src/                 # Frontend (minimal)
│   ├── main.ts
│   └── styles.css
└── package.json
```

### Scripts

```bash
# With Docker (no Node.js required)
./scripts/dev.sh           # Full dev workflow (vite + tauri dev)
./scripts/vite.sh start    # Start Vite dev server in Docker
./scripts/vite.sh stop     # Stop Vite container
./scripts/dpnpm.sh install # Run pnpm commands in Docker

# With local pnpm
pnpm dev          # Start development server
pnpm build        # Build frontend
pnpm tauri dev    # Development mode with hot reload
pnpm tauri build  # Build release binaries
```

## License

MIT License - see [LICENSE](LICENSE) file.

## Acknowledgments

Inspired by:
- [google-chat-tauri](https://github.com/GoogleChatTauri/google-chat-tauri) (Tauri v1)
- [google-chat-electron](https://github.com/robyf/google-chat-electron) (Electron)
- [google-chat-linux](https://github.com/sqrtwolf/google-chat-linux) (Electron)
