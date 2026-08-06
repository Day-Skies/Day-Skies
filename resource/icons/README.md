# Day Skies — "Sun & Cloud" icon exports

Master art: `day-icon.svg` — a warm sun peeking behind a soft cumulus cloud on a day-sky gradient.
Sky `#2F80F0 → #8FC4FF`, sun `#FDBE4A` (amber, echoing the Day sunrise mark), cloud
`#FFFFFF → #E4EFFB`. Full-bleed square; each platform applies its own mask/shape.

Every raster below is generated from the SVGs by `scripts/gen-icons.sh` (rsvg-convert + magick),
which also copies the committed platform assets into `platform/`. Edit an SVG, re-run the script.

## iOS (`ios/`)
- `AppIcon-1024.png` — full-bleed square, **opaque** (no alpha, for App Store validation). The
  Xcode AppIcon single-size slot; iOS applies its own corner mask.
- `day-icon-ios.svg` — the master art, for re-export.

## Android (`android/`)
- `ic_launcher_foreground.svg/png` (432×432) — the sun+cloud motif inside the 66dp safe zone,
  transparent background.
- `ic_launcher_background.svg/png` (432×432) — the sky gradient.
- `ic_launcher-legacy-192.png` — legacy launcher fallback (full-bleed, opaque).
- `play-store-512.png` — Play listing icon (full-bleed, opaque).

## macOS (`macos/`)
- `day-icon-macos.svg` + `day-icon-macos-{16,32,128,256,512,1024}.png` — the art in Apple's rounded
  body with a transparent margin (824 pt art on a 1024 canvas). `day pack -p macos-appkit` builds
  `AppIcon.icns` from these via `sips`/`iconutil`; the desktop dock icon is loaded from the largest.

## Windows (`windows/`)
- `day.ico` — multi-size (256/48/32/16, PNG-compressed).
- `day-icon-256.png`.

## Linux (`linux/`)
- `day-icon-{512,256,128,48}.png` — install under `hicolor/<size>x<size>/apps/`; the root
  `day-icon.svg` serves `hicolor/scalable/apps/`.

## Web / general (`png/`)
- `day-icon-{1024,512,256,128,64,32,16}.png` — favicons, PWA manifest, etc.
