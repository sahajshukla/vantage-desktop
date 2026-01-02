# Vantage AuditOS Desktop Application

A cross-platform desktop application for Vantage AuditOS built with Tauri.

## Features

- Native desktop experience on Windows, macOS, and Linux
- License key activation and validation
- Secure machine binding (licenses tied to specific machines)
- Auto-updates
- Compiled Rust binary (code protection)

## Architecture

```
┌─────────────────────────────────────────┐
│         Vantage AuditOS Desktop         │
├─────────────────────────────────────────┤
│  ┌───────────────────────────────────┐  │
│  │     Tauri Shell (Rust Binary)     │  │
│  │  • License validation             │  │
│  │  • Machine ID generation          │  │
│  │  • Secure storage                 │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │       WebView (Native OS)         │  │
│  │  • Loads app.vantageauditos.com   │  │
│  │  • Minimal local assets           │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────┐
│     Vantage Cloud (Your Backend)        │
│  • API endpoints                        │
│  • License validation API               │
│  • User authentication                  │
└─────────────────────────────────────────┘
```

## Prerequisites

### All Platforms
- Node.js 18+
- Rust 1.70+

### macOS
```bash
xcode-select --install
```

### Windows
- Microsoft Visual Studio C++ Build Tools
- WebView2 (usually pre-installed on Windows 10/11)

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file \
    libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

## Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Node dependencies
```bash
npm install
```

### 3. Install Tauri CLI
```bash
cargo install tauri-cli
```

## Development

Run in development mode:
```bash
npm run dev
# or
cargo tauri dev
```

## Building

### Build for current platform
```bash
npm run build
# or
cargo tauri build
```

### Cross-platform builds

**macOS (Universal - Intel + Apple Silicon):**
```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo tauri build --target universal-apple-darwin
```

**Windows:**
```bash
# On Windows or with cross-compilation
cargo tauri build --target x86_64-pc-windows-msvc
```

**Linux:**
```bash
cargo tauri build --target x86_64-unknown-linux-gnu
```

## Build Outputs

After building, find installers in:
```
src-tauri/target/release/bundle/
├── dmg/              # macOS disk image
├── macos/            # macOS app bundle
├── msi/              # Windows installer
├── nsis/             # Windows NSIS installer
├── deb/              # Debian package
└── appimage/         # Linux AppImage
```

## Code Signing

### macOS
1. Get an Apple Developer ID certificate
2. Set environment variables:
```bash
export APPLE_CERTIFICATE="Developer ID Application: Your Company (XXXXXXXXXX)"
export APPLE_CERTIFICATE_PASSWORD="your-password"
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Company (XXXXXXXXXX)"
```

### Windows
1. Get a code signing certificate
2. Configure in `tauri.conf.json`:
```json
"windows": {
  "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
  "timestampUrl": "http://timestamp.digicert.com"
}
```

## Auto-Updates

The app supports auto-updates via Tauri's updater. Configure:

1. Generate update signing keys:
```bash
cargo tauri signer generate -w ~/.tauri/vantage.key
```

2. Add public key to `tauri.conf.json`:
```json
"updater": {
  "active": true,
  "pubkey": "YOUR_PUBLIC_KEY",
  "endpoints": ["https://releases.vantageauditos.com/..."]
}
```

3. Host update JSON at your endpoint:
```json
{
  "version": "1.0.1",
  "notes": "Bug fixes",
  "pub_date": "2024-01-01T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "url": "https://releases.vantageauditos.com/VantageAuditOS_1.0.1_x64.dmg.tar.gz",
      "signature": "..."
    }
  }
}
```

## License Activation Flow

1. User downloads and installs the app
2. On first launch, activation page is shown
3. User enters license key + email
4. App calls `POST /api/v1/licenses/activate` with:
   - `license_key`
   - `email`
   - `machine_id` (unique per installation)
   - `platform`
   - `app_version`
5. Server validates and returns license info
6. License stored securely in OS config directory
7. App loads the main web application

## Backend License API

You need to implement these endpoints:

### POST /api/v1/licenses/activate
```json
Request:
{
  "license_key": "XXXX-XXXX-XXXX-XXXX",
  "email": "user@company.com",
  "machine_id": "uuid-here",
  "platform": "macos",
  "app_version": "1.0.0"
}

Response (success):
{
  "valid": true,
  "organization": "Acme Corp",
  "email": "user@company.com",
  "expires_at": "2025-12-31T23:59:59Z",
  "tier": "enterprise"
}

Response (error):
{
  "valid": false,
  "message": "Invalid license key"
}
```

### POST /api/v1/licenses/deactivate
```json
Request:
{
  "license_key": "XXXX-XXXX-XXXX-XXXX",
  "machine_id": "uuid-here"
}
```

## Security Notes

1. **Rust Binary**: Core license logic compiles to native binary, hard to reverse engineer
2. **Machine Binding**: Licenses tied to unique machine IDs
3. **Server Validation**: License keys validated against your API
4. **No Source Code**: Your Python backend never ships to users
5. **Frontend**: Only UI code is in the app (no business logic)

## Troubleshooting

### "Failed to fetch" on activation
- Check internet connection
- Verify API endpoint is accessible
- Check CORS settings on your backend

### App won't start on macOS
```bash
xattr -cr /Applications/Vantage\ AuditOS.app
```

### Linux WebView issues
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

## CI/CD Integration

Example GitHub Actions workflow for building:

```yaml
name: Build Desktop App

on:
  push:
    tags:
      - 'desktop-v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies (Linux)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt update
          sudo apt install libwebkit2gtk-4.0-dev build-essential
      - run: npm install
      - run: cargo tauri build
      - uses: actions/upload-artifact@v4
        with:
          name: bundles-${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```

## Support

- Documentation: https://docs.vantageauditos.com
- Support: support@vantageauditos.com
