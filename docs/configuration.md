# Configuration

GoChat stores its configuration in a JSON file at:

| Platform | Path |
|----------|------|
| Linux | `~/.config/gochat/config.json` |
| macOS | `~/Library/Application Support/gochat/config.json` |
| Windows | `%APPDATA%\gochat\config.json` |

## Configuration File

### Example

```json
{
  "bounds": {
    "x": 100,
    "y": 100,
    "width": 1200,
    "height": 800
  },
  "maximized": false,
  "start_hidden": false,
  "icon_theme": "default",
  "use_tray": true,
  "auto_update": true,
  "third_party_auth_mode": false
}
```

## Options

### `bounds`

Window position and size.

| Property | Type | Default | Constraints | Description |
|----------|------|---------|-------------|-------------|
| `x` | integer | `100` | - | Window X position (pixels from left) |
| `y` | integer | `100` | - | Window Y position (pixels from top) |
| `width` | integer | `1200` | 400 - 3840 | Window width in pixels |
| `height` | integer | `800` | 300 - 2160 | Window height in pixels |

Values outside constraints are automatically clamped on load.

### `maximized`

| Type | Default |
|------|---------|
| boolean | `false` |

Whether the window is maximized. Restored on next launch.

### `start_hidden`

| Type | Default |
|------|---------|
| boolean | `false` |

Start the application minimized to system tray. Useful for auto-start scenarios.

### `icon_theme`

| Type | Default | Options |
|------|---------|---------|
| string | `"default"` | `default`, `colored`, `mono` |

Tray icon theme:
- `default` - Standard icon with notification badge states
- `colored` - Full-color variant
- `mono` - Monochrome variant (better for dark themes)

Invalid values are reset to `default`.

### `use_tray`

| Type | Default |
|------|---------|
| boolean | `true` |

Enable system tray icon. When disabled, closing the window exits the application.

### `auto_update`

| Type | Default |
|------|---------|
| boolean | `true` |

Enable automatic update checks on startup. Manual update check is always available via tray menu.

### `third_party_auth_mode`

| Type | Default |
|------|---------|
| boolean | `false` |

Enable third-party authentication mode for enterprise SSO. When enabled, navigation to authentication providers stays within the app instead of opening in an external browser.

See [Third-party Authentication](#third-party-authentication) for details.

## Custom CSS

Custom CSS allows you to customize the appearance of Google Chat.

### Location

| Platform | Path |
|----------|------|
| Linux | `~/.config/gochat/custom.css` |
| macOS | `~/Library/Application Support/gochat/custom.css` |
| Windows | `%APPDATA%\gochat\custom.css` |

### Example

```css
/* Darker sidebar */
body {
  --sidebar-bg: #1a1a1a !important;
}

/* Hide specific elements */
.some-selector {
  display: none !important;
}

/* Custom fonts */
* {
  font-family: 'Inter', sans-serif !important;
}
```

### Applying Changes

After editing `custom.css`:
1. Right-click the tray icon
2. Select **Reload**

Or restart the application.

## Third-party Authentication

For enterprise environments using SSO providers (Okta, Azure AD, etc.), enable third-party auth mode:

### Via Tray Menu

1. Right-click tray icon
2. Select **Auth Mode > Third-party**
3. App will restart

### Via Environment Variable

Set `NO_REDIRECT_URL` with comma-separated domains that should stay in-app:

```bash
export NO_REDIRECT_URL="accounts.google.com,accounts.youtube.com,your-sso.okta.com"
```

### Default Whitelist

The following domains are always whitelisted:
- `accounts.google.com`
- `accounts.youtube.com`
- `mail.google.com/ServiceLogin`
- `mail.google.com/chat`
- `chat.google.com`

## Deep Links

GoChat registers the `gchat://` protocol handler. Click links like:

```
gchat://chat.google.com/dm/CONVERSATION_ID
```

to open conversations directly in GoChat.

## Environment Variables

| Variable | Description |
|----------|-------------|
| `NO_REDIRECT_URL` | Comma-separated list of domains to keep in-app during third-party auth |

## Resetting Configuration

To reset to defaults:

1. Close GoChat
2. Delete the config directory:
   - Linux: `rm -rf ~/.config/gochat`
   - macOS: `rm -rf ~/Library/Application\ Support/gochat`
   - Windows: `rmdir /s "%APPDATA%\gochat"`
3. Restart GoChat
