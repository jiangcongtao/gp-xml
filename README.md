# XML Navigator

A modern desktop application for exploring XML files, built with Tauri and Monaco Editor.

## Features

- **Tree View**: Navigate XML structure with collapsible tree view
- **XML View**: Syntax-highlighted XML source with Monaco Editor
- **Search & Filter**: Search nodes by name or value
- **Sorting**: Sort tree nodes and detail properties
- **Help Schema**: Load JSON schema for field validation and tooltips
- **Native File Dialog**: Open and save files using system dialogs
- **System Tray**: Quick access from system tray
- **Auto Update**: Built-in update mechanism (when configured)
- **Cross-Platform**: Works on macOS, Windows, and Linux

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version)
- [Rust](https://rustup.rs/) (stable)
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft C++ Build Tools, WebView2
  - **Linux**: `webkit2gtk-4.1`, `libappindicator3`, `librsvg2`

### Install Dependencies

```bash
npm install
```

### Development Mode

```bash
npm run tauri dev
```

### Production Build

#### macOS

Due to a known issue with DMG creation on macOS Sequoia (15.x), use the provided build script:

```bash
./build-dmg.sh
```

This script will:
1. Build the `.app` bundle
2. Create a DMG file using `hdiutil`
3. Place the DMG in `src-tauri/target/release/bundle/dmg/`

**Alternative: Build without DMG**

```bash
npm run tauri build -- --bundles app
```

#### Windows

See [WINDOWS_BUILD.md](./WINDOWS_BUILD.md) for detailed instructions.

**Quick start:**

```powershell
# On Windows, build both MSI and NSIS installers
npm run tauri build
```

Installers will be in:
- `src-tauri\target\release\bundle\msi\` - MSI installer
- `src-tauri\target\release\bundle\nsis\` - NSIS installer

#### Linux

```bash
npm run tauri build
```

Creates `.deb`, `.rpm`, and `.AppImage` packages in `src-tauri/target/release/bundle/`.

Build artifacts will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
xml-navigator/
├── src/                      # Frontend source
│   ├── index.html           # Main HTML file
│   └── monaco/              # Local Monaco Editor resources
├── src-tauri/               # Tauri/Rust backend
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   ├── capabilities/        # Permission configuration
│   ├── icons/               # Application icons
│   └── src/
│       └── main.rs          # Rust entry point
├── .github/workflows/       # CI/CD configuration
├── package.json             # Node.js configuration
└── README.md
```

## Auto Update Setup

To enable auto-updates:

1. Generate signing keys:
   ```bash
   npm run tauri signer generate -- -w ~/.tauri/xml-navigator.key
   ```

2. Add secrets to GitHub repository:
   - `TAURI_SIGNING_PRIVATE_KEY`: Content of the private key
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: Password for the key

3. Update `src-tauri/tauri.conf.json`:
   ```json
   "plugins": {
     "updater": {
       "endpoints": ["https://github.com/YOUR_USERNAME/xml-navigator/releases/latest/download/latest.json"],
       "pubkey": "YOUR_PUBLIC_KEY"
     }
   }
   ```

## Keyboard Shortcuts

| Action | macOS | Windows/Linux |
|--------|-------|---------------|
| Open File | ⌘O | Ctrl+O |
| Save | ⌘S | Ctrl+S |
| Save As | ⌘⇧S | Ctrl+Shift+S |
| Expand All | ⌘E | Ctrl+E |
| Collapse All | ⌘⇧E | Ctrl+Shift+E |
| Toggle Tree | ⌘T | Ctrl+T |
| Close Window | ⌘W | Ctrl+W |

## License

MIT

