# Quick Build Reference

## Build Commands

### Development
```bash
npm run tauri dev          # Run in development mode
```

### Production Builds

#### macOS
```bash
./build-dmg.sh                              # Create DMG (recommended)
npm run tauri build -- --bundles app        # Just .app bundle
npm run tauri build -- --bundles dmg        # Try native DMG (may fail on Sequoia)
```

#### Windows
```powershell
npm run tauri build                         # Both MSI and NSIS
npm run tauri build -- --bundles msi        # MSI only
npm run tauri build -- --bundles nsis       # NSIS only
```

#### Linux
```bash
npm run tauri build                         # All formats (.deb, .rpm, .AppImage)
npm run tauri build -- --bundles deb        # Debian package only
npm run tauri build -- --bundles rpm        # RPM package only
npm run tauri build -- --bundles appimage   # AppImage only
```

## Build Output Locations

```
src-tauri/target/release/bundle/
├── macos/
│   └── XML Navigator.app              # macOS app bundle
├── dmg/
│   └── XML Navigator_1.0.0_x64.dmg   # macOS disk image
├── msi/
│   └── XML Navigator_1.0.0_x64_en-US.msi    # Windows MSI installer
├── nsis/
│   └── XML Navigator_1.0.0_x64-setup.exe    # Windows NSIS installer
├── deb/
│   └── xml-navigator_1.0.0_amd64.deb        # Debian/Ubuntu package
├── rpm/
│   └── xml-navigator-1.0.0-1.x86_64.rpm     # RedHat/Fedora package
└── appimage/
    └── xml-navigator_1.0.0_amd64.AppImage   # Universal Linux app
```

## Recommended Installers by Use Case

| Use Case | Recommended Format |
|----------|-------------------|
| **macOS** | DMG (use `build-dmg.sh`) |
| **Windows - Consumer** | NSIS (.exe) - Smaller, modern UI |
| **Windows - Enterprise** | MSI - Group Policy support |
| **Ubuntu/Debian** | .deb package |
| **Fedora/RedHat** | .rpm package |
| **Any Linux** | AppImage - No installation required |

## Quick Tips

### Smaller Build Size
```bash
# Build for specific target only
npm run tauri build -- --bundles nsis  # ~5-10MB NSIS installer
```

### Debug Build Issues
```bash
npm run tauri build -- --verbose       # Detailed output
npm run tauri build -- --debug         # Debug symbols included
```

### Build for Different Architectures
```bash
# Windows: x64 (default)
npm run tauri build

# macOS: Universal binary (Intel + Apple Silicon)
npm run tauri build -- --target universal-apple-darwin
```

## Platform-Specific Prerequisites

### macOS
- Xcode Command Line Tools: `xcode-select --install`

### Windows  
- Visual Studio Build Tools with C++
- WebView2 Runtime

### Linux (Ubuntu/Debian)
```bash
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Linux (Fedora/RHEL)
```bash
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| DMG creation fails on macOS | Use `./build-dmg.sh` script |
| MSI build fails on Windows | Ensure Visual Studio Build Tools installed |
| Linux build missing deps | Install webkit2gtk-4.1 and dependencies |
| Build size too large | Check bundle configuration, optimize assets |
| Slow build times | Use `--bundles` flag to build specific format only |

## Environment Variables

```bash
# Skip WebView2 download on Windows (if pre-installed)
TAURI_SKIP_WEBVIEW_INSTALL=true npm run tauri build

# Custom certificate for macOS signing
APPLE_CERTIFICATE=path/to/cert.p12 npm run tauri build

# Increase build verbosity
RUST_LOG=debug npm run tauri build
```

## CI/CD

For automated builds across all platforms, see:
- `.github/workflows/` - GitHub Actions workflows
- `WINDOWS_BUILD.md` - Detailed Windows build guide
- Tauri docs: https://v2.tauri.app/distribute/

## Signing & Distribution

### macOS
- Code signing: Configure in `tauri.conf.json` > `bundle.macOS.signingIdentity`
- Notarization: Required for Gatekeeper
- Distribution: DMG, Mac App Store, or direct download

### Windows
- Code signing: Use SignTool with certificate
- Distribution: Direct download, Microsoft Store (MSIX)

### Linux
- Signing: Optional (GPG for repositories)
- Distribution: Package repositories, Snap, Flatpak, or direct download

