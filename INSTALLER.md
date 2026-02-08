# Limbo Voice Installer Configuration

This document explains the professional installer setup for Limbo Voice.

## Installer Features

### NSIS Installer (.exe)
- **License Agreement**: Shows LICENSE.txt with "I Accept" checkbox
- **Installation Mode**: Per-user (no admin required)
- **Compression**: LZMA (maximum compression)
- **Start Menu**: Creates "Limbo Voice" folder
- **Uninstaller**: Clean removal, preserves user preferences option

### MSI Installer (Enterprise)
- **Windows Installer**: For corporate deployments
- **Group Policy**: Can be deployed via GPO
- **License**: Embedded in installer

## Window Configuration

The main app window is configured as:
- **Transparent & Frameless**: Modern, minimal UI
- **Always on Top**: Visible when recording
- **Hidden by Default**: Only shows when activated
- **Skip Taskbar**: Doesn't clutter taskbar
- **Non-resizable**: Fixed, optimized size

## File Structure

```
Limbo Voice/
├── Limbo Voice.exe          # Main application
├── resources/
│   └── whisper-model.bin   # AI model (bundled)
├── LICENSE.txt             # License agreement
└── uninstall.exe          # Clean uninstaller
```

## Installation Paths

- **Program Files**: `%LOCALAPPDATA%\Programs\Limbo Voice`
- **App Data**: `%APPDATA%\Limbo Voice` (settings, cache)
- **Start Menu**: `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Limbo Voice`

## Installer Behavior

1. **Welcome Screen**: Branded introduction
2. **License Agreement**: Must accept to continue
3. **Installation Path**: Default with browse option
4. **Options**:
   - ☑ Create Desktop Shortcut
   - ☑ Add to Start Menu
   - ☐ Run on Windows Startup (optional)
5. **Installation**: Progress bar with status
6. **Completion**: Option to launch immediately

## Uninstaller Features

- **Clean Removal**: Removes all app files
- **Preserve Option**: Asks to keep user data
- **No Dark Patterns**: Simple, respectful process
- **Registry Cleanup**: Removes all registry entries

## Custom Branding (TODO)

To complete the installer branding:
1. Create `icons/installer-header.bmp` (150x57px)
2. Add Limbo Voice logo and gradient background
3. Use neon cyan (#00D9FF) color scheme

## Build Commands

```bash
# Development build
npm run tauri dev

# Production installers
npm run tauri build

# Output locations
src-tauri/target/release/bundle/nsis/Limbo Voice_1.0.0_x64-setup.exe
src-tauri/target/release/bundle/msi/Limbo Voice_1.0.0_x64_en-US.msi
```
