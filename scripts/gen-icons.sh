#!/usr/bin/env bash
# Regenerate every app-icon raster from the SVGs under resource/icons/, and copy the committed
# platform assets into platform/. Edit an SVG (day-icon.svg / macos / android_*), then run this.
#
# Requires: rsvg-convert (librsvg) and magick (ImageMagick). See resource/icons/README.md.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
icons="$here/resource/icons"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

r() { rsvg-convert -w "$2" -h "$2" -o "$3" "$1"; }  # svg  size  outfile

master="$icons/day-icon.svg"

# --- generic png/ (full-bleed) ---
for sz in 16 32 64 128 256 512 1024; do r "$master" "$sz" "$icons/png/day-icon-$sz.png"; done

# --- iOS (full-bleed 1024, alpha stripped so App Store validation accepts it) ---
r "$master" 1024 "$tmp/ios.png"
magick "$tmp/ios.png" -background '#2F80F0' -alpha remove -alpha off "$icons/ios/AppIcon-1024.png"

# --- macOS (squircle with transparent margin) ---
for sz in 16 32 128 256 512 1024; do r "$icons/macos/day-icon-macos.svg" "$sz" "$icons/macos/day-icon-macos-$sz.png"; done

# --- Linux (full-bleed) ---
for sz in 48 128 256 512; do r "$master" "$sz" "$icons/linux/day-icon-$sz.png"; done

# --- Windows (.ico multi-size + a 256 png) ---
r "$master" 256 "$icons/windows/day-icon-256.png"
for sz in 16 32 48 256; do r "$master" "$sz" "$tmp/w$sz.png"; done
magick "$tmp/w16.png" "$tmp/w32.png" "$tmp/w48.png" "$tmp/w256.png" "$icons/windows/day.ico"

# --- Android (adaptive foreground/background, legacy, play-store) ---
r "$icons/android/ic_launcher_foreground.svg" 432 "$icons/android/ic_launcher_foreground.png"
r "$icons/android/ic_launcher_background.svg" 432 "$icons/android/ic_launcher_background.png"
r "$master" 192 "$icons/android/ic_launcher-legacy-192.png"
r "$master" 512 "$icons/android/play-store-512.png"

# --- Copy the committed platform assets that builds consume ---
cp "$icons/ios/AppIcon-1024.png" "$here/platform/ios/Assets.xcassets/AppIcon.appiconset/AppIcon-1024.png"
ar="$here/platform/android/app/src/main/res/mipmap-xxxhdpi"
cp "$icons/android/ic_launcher-legacy-192.png" "$ar/ic_launcher.png"
cp "$icons/android/ic_launcher_foreground.png" "$ar/ic_launcher_foreground.png"
cp "$icons/android/ic_launcher_background.png" "$ar/ic_launcher_background.png"
cp "$icons/png/day-icon-512.png" "$here/platform/ohos/entry/src/main/resources/base/media/startIcon.png"
cp "$icons/png/day-icon-512.png" "$here/platform/ohos/AppScope/resources/base/media/startIcon.png"

echo "icons regenerated from $icons and staged into platform/"
