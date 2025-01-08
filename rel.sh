#!/bin/bash

set -e

#######################################
# Variables
#######################################
APP_NAME="projectm_sdl"
BUNDLE_ID="projectM"
DEVELOPER_ID="Developer ID Application: Mischa Spiegelmock (5926VBQM6Y)"
TEAM_ID="5926VBQM6Y"
KEYCHAIN_PROFILE="projectm"

BUILD_DIR="target"
OUTPUT_DIR="${PWD}/dist"

# Paths for universal binary
UNIVERSAL_BINARY="${OUTPUT_DIR}/${APP_NAME}"

# .app bundle paths
APP_BUNDLE_NAME="${APP_NAME}.app"
APP_BUNDLE_PATH="${OUTPUT_DIR}/${APP_BUNDLE_NAME}"
APP_EXECUTABLE_PATH="${APP_BUNDLE_PATH}/Contents/MacOS"
INFO_PLIST_PATH="${APP_BUNDLE_PATH}/Contents/Info.plist"
RESOURCES_PATH="${APP_BUNDLE_PATH}/Contents/Resources"

# Entitlements file (if sandboxing is needed)
ENTITLEMENTS_FILE="${OUTPUT_DIR}/entitlements.plist"

# Zip paths
PRE_NOTARIZATION_ZIP="${OUTPUT_DIR}/${APP_NAME}-pre-notarization.zip"
FINAL_ZIP="${OUTPUT_DIR}/${APP_NAME}.zip"

#######################################
# 1) Build Rust Binaries (x86_64 + arm64)
#######################################
echo "==> Building for x86_64"
cargo build --release --target x86_64-apple-darwin

echo "==> Building for arm64"
cargo build --release --target aarch64-apple-darwin

#######################################
# 2) Create Universal Binary
#######################################
mkdir -p "${OUTPUT_DIR}"
echo "==> Creating universal binary"
lipo -create -output "${UNIVERSAL_BINARY}" \
  "${BUILD_DIR}/x86_64-apple-darwin/release/${APP_NAME}" \
  "${BUILD_DIR}/aarch64-apple-darwin/release/${APP_NAME}"

#######################################
# 3) Create .app Bundle Structure
#######################################
echo "==> Creating .app bundle structure"
rm -rf "${APP_BUNDLE_PATH}" || true
mkdir -p "${APP_EXECUTABLE_PATH}"
mkdir -p "${RESOURCES_PATH}"

# Move the universal binary into MacOS/
mv "${UNIVERSAL_BINARY}" "${APP_EXECUTABLE_PATH}/${APP_NAME}"

#######################################
# 4) Create Info.plist with Microphone Access
#######################################
echo "==> Creating Info.plist"
cat > "${INFO_PLIST_PATH}" <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>NSMicrophoneUsageDescription</key>
    <string>This app requires microphone access for audio input.</string>
</dict>
</plist>
EOL

#######################################
# 5) (Optional) Create Entitlements File for Sandboxing
#######################################
echo "==> Creating entitlements file for sandboxing (optional)"
cat > "${ENTITLEMENTS_FILE}" <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
    <key>com.apple.security.device.audio-input</key>
    <true/>
</dict>
</plist>
EOL

#######################################
# 6) Clone and Copy Presets/Textures
#######################################
echo "==> Cloning preset repositories"
TEMP_DIR="$(mktemp -d)"
pushd "$TEMP_DIR" >/dev/null

# cream-of-the-crop
git clone git@github.com:projectM-visualizer/presets-cream-of-the-crop.git
mkdir -p "${RESOURCES_PATH}/presets"
cp -R presets-cream-of-the-crop/"." "${RESOURCES_PATH}/presets/"

# milkdrop-texture-pack
git clone git@github.com:projectM-visualizer/presets-milkdrop-texture-pack.git
mkdir -p "${RESOURCES_PATH}/textures"
cp -R presets-milkdrop-texture-pack/textures/"." "${RESOURCES_PATH}/textures/"

popd >/dev/null
rm -rf "$TEMP_DIR"

#######################################
# 7) Sign the .app Bundle with Entitlements
#######################################
echo "==> Signing the .app with hardened runtime and entitlements"
codesign --deep --verbose --force --options runtime \
  --entitlements "${ENTITLEMENTS_FILE}" \
  --sign "${DEVELOPER_ID}" "${APP_BUNDLE_PATH}"

#######################################
# 8) Zip the Signed .app for Notarization
#######################################
echo "==> Creating zip for notarization"
rm -f "${PRE_NOTARIZATION_ZIP}"
ditto -c -k --sequesterRsrc --keepParent \
  "${APP_BUNDLE_PATH}" \
  "${PRE_NOTARIZATION_ZIP}"

#######################################
# 9) Submit the Zip File for Notarization
#######################################
echo "==> Submitting for notarization"
xcrun notarytool submit "${PRE_NOTARIZATION_ZIP}" \
  --keychain-profile "${KEYCHAIN_PROFILE}" \
  --team-id "${TEAM_ID}" \
  --wait

#######################################
# 10) Staple the Now-Notarized .app
#######################################
echo "==> Stapling notarization ticket to .app"
xcrun stapler staple "${APP_BUNDLE_PATH}"

#######################################
# 11) (Optional) Create Final Zip with Stapled .app
#######################################
echo "==> Creating final zip of stapled .app"
rm -f "${FINAL_ZIP}"
ditto -c -k --sequesterRsrc --keepParent \
  "${APP_BUNDLE_PATH}" \
  "${FINAL_ZIP}"

#######################################
# 12) Verify with Gatekeeper
#######################################
echo "==> Verifying with spctl"
spctl --assess --verbose=4 "${APP_BUNDLE_PATH}"

rm "${PRE_NOTARIZATION_ZIP}"
rm "${ENTITLEMENTS_FILE}"

echo "✅ Build, sign, notarize, staple, and package completed successfully!"