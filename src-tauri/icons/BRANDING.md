# Installer Branding Assets - TODO

## Required Files

### Installer Header Image
**File**: `src-tauri/icons/installer-header.bmp`
**Dimensions**: 150px × 57px
**Format**: BMP (24-bit or 32-bit)

**Design Specifications**:
- Background: Dark gradient (#0A0A0F to #1A1A2E)
- Logo: "Limbo Voice" text with microphone icon
- Color Accent: Neon cyan (#00D9FF) with glow effect
- Style: Modern, minimal, professional

### App Icon (Already exists)
The following icon files should exist in `src-tauri/icons/`:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

## Creating the Header Image

You can create this using:
1. **Photoshop/GIMP**: Design and export as BMP
2. **Figma**: Design and export as PNG, convert to BMP
3. **Online Tool**: Use a gradient generator + add text

### Quick DIY Method
1. Create 150×57px canvas
2. Fill with dark gradient
3. Add "LIMBO VOICE" text (white, bold)
4. Add microphone SVG icon (cyan glow)
5. Export as BMP

## Alternative: Skip Header Image

If you want to build the installer NOW without custom branding:
1. Comment out the `headerImage` line in `tauri.conf.json`
2. NSIS will use default Windows installer header
3. Add custom header later for polish

## Note

The installer will work perfectly fine without the custom header image. The license agreement, metadata, and all other professional features are already configured!
