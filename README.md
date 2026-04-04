<p align="center">
  <a href="docs/README.ru.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/20px-Flag_of_Russia.svg.png" alt="Russian"/> Русский</a> &nbsp;
  <a href="README.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/20px-Flag_of_the_United_States.svg.png" alt="English"/> English</a> &nbsp;
  <a href="docs/README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/20px-Flag_of_France.svg.png" alt="French"/> Français</a> &nbsp;
  <a href="docs/README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/20px-Flag_of_Spain_%28civil%29.svg.png" alt="Spanish"/> Español</a> &nbsp;
  <a href="docs/README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/20px-Flag_of_Germany.svg.png" alt="German"/> Deutsch</a> &nbsp;
  <a href="docs/README.it.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/20px-Flag_of_Italy.svg.png" alt="Italian"/> Italiano</a>
</p>
<p align="center">
  <a href="docs/README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/20px-Flag_of_Poland.svg.png" alt="Polish"/> Polski</a> &nbsp;
  <a href="docs/README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/20px-Flag_of_Brazil.svg.png" alt="Portuguese"/> Português</a> &nbsp;
  <a href="docs/README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/20px-Flag_of_Japan.svg.png" alt="Japanese"/> 日本語</a> &nbsp;
  <a href="docs/README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/20px-Flag_of_South_Korea.svg.png" alt="Korean"/> 한국어</a> &nbsp;
  <a href="docs/README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/20px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese"/> 中文</a> &nbsp;
  <a href="docs/README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/20px-Flag_of_India.svg.png" alt="Hindi"/> हिन्दी</a>
</p>

---

# MicLocker

A lightweight Windows utility that locks your microphone volume at a fixed level. Prevents accidental or unwanted volume changes.

## Features

- Select microphone from available audio input devices
- Set volume level (0–100%)
- Automatic volume maintenance every second
- Clean, minimal interface
- Auto-start on system boot (optional)

## Quick Start

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

After building, you'll find the executables in:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: `windows` crate for Core Audio API access

## Requirements

- Windows 10/11
- Node.js 18+
- Rust toolchain (for building from source)

## License

MIT
