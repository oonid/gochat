# Auto-Update Configuration

GoChat uses Tauri's built-in updater plugin to deliver automatic updates to users. This document explains how updates work and how to configure signing keys for CI/CD.

## How Updates Work

1. **Update Check**: On startup (if `auto_update: true` in config), GoChat checks GitHub releases for a newer version
2. **Manual Check**: Users can check for updates via the tray menu → "Check for Updates"
3. **Download & Install**: Updates are downloaded and installed automatically when found
4. **Signature Verification**: All updates are cryptographically signed to ensure authenticity

## Signing Keys

Signing keys ensure that only official updates from the repository maintainer can be installed. Without valid signatures, the updater will reject the update.

### Key Files

| File | Location | Purpose |
|------|----------|---------|
| Private Key | `.tauri/gochat.key` | Signs update packages (KEEP SECRET!) |
| Public Key | `.tauri/gochat.key.pub` | Embedded in app to verify signatures |
| Public Key | `tauri.conf.json` | Same public key, embedded in build |

### Generating New Keys

If you need to regenerate keys (e.g., key compromise):

```bash
# Generate new keypair
cargo tauri signer generate -w .tauri/gochat.key -f

# Extract public key and update tauri.conf.json
cat .tauri/gochat.key.pub
```

> **Warning**: Regenerating keys invalidates all previous updates. Users on older versions won't be able to update.

## CI/CD Setup

### GitHub Actions

Add the private key as a repository secret:

1. Go to your repository → Settings → Secrets and variables → Actions
2. Create a new repository secret:
   - **Name**: `TAURI_SIGNING_PRIVATE_KEY`
   - **Value**: Contents of `.tauri/gochat.key` (the private key file)

3. (Optional) If your key has a password, also add:
   - **Name**: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
   - **Value**: Your key password

### Example GitHub Actions Workflow

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install Tauri CLI
        run: cargo install tauri-cli
      
      - name: Build and Release
        env:
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
          # TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
        run: cargo tauri build
```

### Environment Variables Reference

| Variable | Required | Description |
|----------|----------|-------------|
| `TAURI_SIGNING_PRIVATE_KEY` | Yes | The private key content (not file path) |
| `TAURI_SIGNING_PRIVATE_KEY_PATH` | Alt | Path to private key file (alternative to above) |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | If applicable | Password if the key was created with one |

### Reading Private Key for CI

To get the private key content for the secret:

```bash
# Output the private key (copy this for GitHub secret)
cat .tauri/gochat.key

# Or encode as base64 if needed
base64 -w 0 .tauri/gochat.key
```

## Update Endpoint

Updates are fetched from:
```
https://github.com/oonid/gochat/releases/latest/download/{target}-{arch}.json
```

Where:
- `{target}`: `linux`, `darwin` (macOS), or `windows`
- `{arch}`: `x86_64` or `aarch64`

Example: `https://github.com/oonid/gochat/releases/latest/download/linux-x86_64.json`

## Release Process

1. Update version in `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`
2. Create a git tag: `git tag v0.1.1`
3. Push tag: `git push origin v0.1.1`
4. CI builds and creates GitHub release with signed update files
5. Users receive update automatically on next launch

## Disabling Updates

Users can disable automatic update checks by setting `auto_update: false` in their config:

```json
// ~/.config/gochat/config.json
{
  "auto_update": false
}
```

Manual update checks from the tray menu will still work.

## Troubleshooting

### Update check fails
- Verify GitHub releases exist with correct JSON files
- Check that public key in `tauri.conf.json` matches the key used to sign

### Signature verification fails
- Ensure the same private key is used for signing in CI
- Verify public key wasn't accidentally changed

### Updates not detected
- Ensure version number in release is higher than installed version
- Check that release JSON files are attached to GitHub release
