# Limbo Voice - Python Edition

A lightweight, standalone voice dictation tool for Windows.

## 🚀 Quick Start

### Option 1: Build the .exe yourself
1. **Double-click** `build_exe.bat`
2. Wait 2-3 minutes for the build to complete
3. Run `dist\LimboVoice.exe`

### Option 2: Run from Python directly
```bash
pip install -r requirements.txt
python limbo_voice.py
```

## 🎯 How to Use

1. Launch **LimboVoice.exe**
2. Press **Alt+Space** anywhere to start dictating
3. Speak clearly into your microphone
4. Release **Alt+Space** (or press again) to stop
5. Your speech will be typed automatically!

## ✨ Features

- ✅ **Global Hotkey**: Alt+Space works in any app
- ✅ **Beautiful UI**: Glassmorphic overlay with status indicator
- ✅ **Free**: Uses Google's free speech recognition
- ✅ **Lightweight**: Single .exe, no installation needed
- ✅ **Privacy**: Audio processed locally, sent only to Google for transcription

## 🛠️ Technical Details

- **Voice Recognition**: Google Speech Recognition API (free tier)
- **Audio Capture**: PyAudio (16kHz, mono)
- **Keyboard Simulation**: pynput
- **Hotkey Detection**: keyboard library
- **UI**: Tkinter (built into Python)

## 📝 Notes

- Requires an internet connection for speech recognition
- Works on Windows 10/11
- Microphone access permission required
