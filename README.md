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
