# Limbo Voice 🎙️

**Universal Voice Dictation for Windows**

> Press `Alt + Space` anywhere to start dictating. Works in every app.

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey)

---

## ✨ Features

- **🌍 Universal**: Works in Notepad, VS Code, Chrome, Slack, Discord - literally any app
- **⚡ Fast**: Local AI processing with Whisper Large-V3 Turbo
- **🔒 Privacy-First**: 100% offline, zero data collection
- **💰 Free Forever**: No subscriptions, no API costs
- **🎨 Beautiful**: Premium glassmorphism UI with smooth animations
- **📦 Portable**: Single .exe, share with anyone

---

## 📥 Installation

### Quick Install
1. Download `Limbo-Voice-Setup.exe`
2. Run the installer
3. Accept the license agreement
4. Press `Alt + Space` to start dictating!

### System Requirements
- Windows 10/11 (64-bit)
- Microphone access
- ~500MB disk space (includes AI model)

---

## 🚀 Usage

1. **Activate**: Press `Alt + Space` in any application
2. **Speak**: A glowing microphone appears bottom-right
3. **Stop**: Press `Alt + Space` again or just stop talking
4. **Text Appears**: Transcribed text types into your active window

---

## 🎨 What Makes It Beautiful

- **Transparent Overlay**: Minimal, non-intrusive design
- **Neon Glow Effects**: Cyan accents inspired by modern UI
- **Smooth Animations**: Pulsating rings, slide-in transitions
- **Glassmorphism**: Blur effects with semi-transparent panels
- **Smart Auto-Hide**: Disappears when idle

---

## 🔒 Privacy Guarantee

✅ **No Cloud** - Everything runs locally  
✅ **No Tracking** - Zero analytics or telemetry  
✅ **No Data Collection** - Your voice never leaves your device  
✅ **Open Source AI** - Uses Whisper (MIT license)  

---

## 🛠️ Building from Source

### Prerequisites
- Node.js 16+
- Rust 1.70+
- Visual Studio C++ Build Tools

### Build Steps
```bash
git clone https://github.com/limbo-voice/limbo-voice
cd limbo-voice
npm install
npm run tauri build
```

Output: `src-tauri/target/release/bundle/nsis/Limbo Voice_1.0.0_x64-setup.exe`

---

## 📁 Project Structure

```
limbo-voice/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── App.tsx            # Main app
│   └── App.css            # Styles
├── src-tauri/             # Rust backend
│   ├── src/lib.rs         # Core logic
│   └── Cargo.toml         # Dependencies
├── LICENSE.txt            # License agreement
└── README.md              # This file
```

---

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines first.

---

## 📄 License

MIT License - see [LICENSE.txt](LICENSE.txt)

---

## 💡 Inspiration

Inspired by [Glaido](https://glaido.com/) (macOS only). We wanted a free, privacy-first alternative for Windows.

---

## 🙏 Credits

Built with:
- [Tauri](https://tauri.app/) - Desktop framework
- [React](https://react.dev/) - UI library
- [Whisper](https://github.com/openai/whisper) - Speech-to-text AI
- [Rust](https://www.rust-lang.org/) - Systems programming

---

**Made with ❤️ for the Windows community**

Press `Alt + Space` and start talking! 🎙️
