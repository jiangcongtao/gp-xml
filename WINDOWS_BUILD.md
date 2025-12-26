# Windows Build Guide

This guide explains how to create Windows installer packages for XML Navigator.

## Prerequisites

### 1. Windows System Requirements

- Windows 10/11 (64-bit)
- Visual Studio 2019 or later with C++ Build Tools
- WebView2 Runtime (usually pre-installed on Windows 11)

### 2. Install Required Tools

#### Install Rust
```powershell
# Download and run rustup-init.exe from https://rustup.rs/
# Or use winget:
winget install Rustlang.Rustup
```

#### Install Node.js
```powershell
winget install OpenJS.NodeJS.LTS
```

#### Install Visual Studio Build Tools
```powershell
# Download from: https://visualstudio.microsoft.com/downloads/
# Or use winget:
winget install Microsoft.VisualStudio.2022.BuildTools
```

During installation, select "Desktop development with C++" workload.

#### Install WebView2 (if not already installed)
```powershell
winget install Microsoft.EdgeWebView2Runtime
```

## Building on Windows

### 1. Clone and Setup

```powershell
cd path\to\project
npm install
```

### 2. Build Installers

Tauri creates two types of Windows installers by default:

#### Build All Installers
```powershell
npm run tauri build
```

This creates:
- **MSI installer** (Windows Installer) - via WiX Toolset
- **NSIS installer** - via NSIS (Nullsoft Scriptable Install System)

Output location: `src-tauri\target\release\bundle\`

#### Build Specific Installer

**MSI only:**
```powershell
npm run tauri build -- --bundles msi
```

**NSIS only:**
```powershell
npm run tauri build -- --bundles nsis
```

## Installer Types

### MSI Installer (Windows Installer)
- **File**: `XML Navigator_1.0.0_x64_en-US.msi`
- **Best for**: Enterprise deployments, Group Policy installation
- **Features**: 
  - Official Windows installer format
  - Support for silent installation
  - Integration with Windows Installer service
  - Better for corporate environments

### NSIS Installer
- **File**: `XML Navigator_1.0.0_x64-setup.exe`
- **Best for**: Consumer applications, smaller file size
- **Features**:
  - Modern installer UI
  - More customization options
  - Smaller file size
  - Faster installation

## Configuration Options

### WebView2 Installation

The `webviewInstallMode` controls how WebView2 is handled:

```json
"webviewInstallMode": {
  "type": "downloadBootstrapper"
}
```

Options:
- `"downloadBootstrapper"` - Downloads WebView2 if not installed (recommended)
- `"embedBootstrapper"` - Includes WebView2 installer (~1.8MB larger)
- `"offlineInstaller"` - Bundles full WebView2 (~150MB larger)
- `"fixedRuntime"` - Uses specific WebView2 version
- `"skip"` - Assumes WebView2 is pre-installed

### Install Mode

```json
"nsis": {
  "installMode": "perUser"
}
```

Options:
- `"perUser"` - Installs for current user (no admin required)
- `"perMachine"` - Installs for all users (requires admin)
- `"both"` - User chooses during installation

### Customization

#### Add License Agreement

1. Create `LICENSE.txt` or `LICENSE.rtf`
2. Update config:

```json
"windows": {
  "nsis": {
    "license": "./LICENSE.txt"
  },
  "wix": {
    "license": "./LICENSE.rtf"
  }
}
```

#### Custom Installer Images

**For NSIS:**
```json
"nsis": {
  "headerImage": "./assets/installer-header.bmp",  // 150x57 pixels
  "sidebarImage": "./assets/installer-sidebar.bmp", // 164x314 pixels
  "installerIcon": "./assets/installer-icon.ico"
}
```

**For WiX:**
```json
"wix": {
  "bannerPath": "./assets/wix-banner.bmp",      // 493x58 pixels
  "dialogImagePath": "./assets/wix-dialog.bmp"  // 493x312 pixels
}
```

## Signing the Installer (Optional)

For distribution, you should sign your installer with a code signing certificate.

### 1. Obtain a Code Signing Certificate

Purchase from:
- DigiCert
- Sectigo
- GlobalSign
- SSL.com

### 2. Sign with SignTool

```powershell
# Install Windows SDK for SignTool
winget install Microsoft.WindowsSDK

# Sign the installer
signtool sign /f "path\to\certificate.pfx" /p "password" /tr http://timestamp.digicert.com /td sha256 /fd sha256 "path\to\XML Navigator_1.0.0_x64-setup.exe"
```

### 3. Verify Signature

```powershell
signtool verify /pa "path\to\XML Navigator_1.0.0_x64-setup.exe"
```

## Cross-Compilation (Build Windows Installer on macOS/Linux)

### On macOS

```bash
# Install Windows cross-compilation tools
brew install mingw-w64

# Add Windows target
rustup target add x86_64-pc-windows-msvc

# Note: Full cross-compilation for Windows installers requires Wine and WiX
# It's recommended to build on Windows or use CI/CD
```

### Using GitHub Actions (Recommended)

See `GITHUB_ACTIONS.md` for setting up automated Windows builds.

## Testing the Installer

1. **Test in a clean Windows VM** to ensure all dependencies are properly bundled
2. **Test both installation modes** (per-user and per-machine)
3. **Test upgrade scenario** - Install v1.0.0, then install v1.0.1
4. **Test uninstallation** - Ensure all files and registry entries are removed
5. **Check Start Menu** - Verify shortcuts are created correctly
6. **Test auto-updates** - If updater is configured

## Troubleshooting

### "VCRUNTIME140.dll was not found"
- Install Visual C++ Redistributable
- Or embed it in installer (increases size)

### "WebView2 not found"
- Change `webviewInstallMode` to `"embedBootstrapper"` or `"offlineInstaller"`

### Build fails with linker errors
- Ensure Visual Studio Build Tools are installed with C++ workload
- Restart terminal after installation

### MSI build fails - "WiX not found"
- WiX is bundled with Tauri, but ensure PATH is correct
- Try building only NSIS: `npm run tauri build -- --bundles nsis`

## File Locations

After build, find installers in:
```
src-tauri/target/release/bundle/
├── msi/
│   └── XML Navigator_1.0.0_x64_en-US.msi
└── nsis/
    └── XML Navigator_1.0.0_x64-setup.exe
```

## Recommended Distribution

1. **For Microsoft Store**: Use MSIX format (configure in `tauri.conf.json`)
2. **For Direct Download**: Provide both MSI and NSIS, let users choose
3. **For Enterprise**: Recommend MSI
4. **For Consumers**: Recommend NSIS (smaller, faster)

## Additional Resources

- [Tauri Windows Bundling](https://v2.tauri.app/distribute/windows-installer/)
- [WiX Toolset Documentation](https://wixtoolset.org/)
- [NSIS Documentation](https://nsis.sourceforge.io/Docs/)
- [Code Signing Best Practices](https://learn.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)

