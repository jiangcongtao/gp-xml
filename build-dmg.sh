#!/bin/bash

# Build DMG for XML Navigator
# This script works around the bundle_dmg.sh issue on macOS Sequoia

set -e  # Exit on error

echo "🔨 Building XML Navigator..."
echo ""

# Step 1: Build the .app bundle
echo "📦 Step 1: Building .app bundle..."
npm run tauri build -- --bundles app

if [ ! -d "src-tauri/target/release/bundle/macos/XML Navigator.app" ]; then
    echo "❌ Error: App bundle was not created"
    exit 1
fi

echo "✅ App bundle created successfully"
echo ""

# Step 2: Create DMG
echo "💿 Step 2: Creating DMG..."

DMG_NAME="XML Navigator_1.0.0_x64.dmg"
DMG_PATH="src-tauri/target/release/bundle/dmg"
APP_PATH="src-tauri/target/release/bundle/macos/XML Navigator.app"

# Create dmg directory if it doesn't exist
mkdir -p "$DMG_PATH"

# Remove existing DMG if it exists
if [ -f "$DMG_PATH/$DMG_NAME" ]; then
    echo "🗑️  Removing existing DMG..."
    rm "$DMG_PATH/$DMG_NAME"
fi

# Create DMG using hdiutil
echo "🎨 Creating disk image..."
hdiutil create -volname "XML Navigator" \
  -srcfolder "$APP_PATH" \
  -ov -format UDZO \
  "$DMG_PATH/$DMG_NAME"

if [ -f "$DMG_PATH/$DMG_NAME" ]; then
    echo ""
    echo "✅ DMG created successfully!"
    echo "📍 Location: $DMG_PATH/$DMG_NAME"
    echo ""
    
    # Get file size
    SIZE=$(du -h "$DMG_PATH/$DMG_NAME" | cut -f1)
    echo "📊 Size: $SIZE"
    
    # Open the folder containing the DMG
    echo ""
    read -p "Open DMG folder? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        open "$DMG_PATH"
    fi
else
    echo "❌ Error: DMG was not created"
    exit 1
fi

