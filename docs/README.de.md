# MicLocker

Ein leichtes Windows-Programm, das die Mikrofonlautstärke auf einem festen Wert hält. Verhindert versehentliche oder unerwünschte Lautstärkeänderungen.

## Funktionen

- Mikrofon aus verfügbaren Audioeingabegeräten auswählen
- Lautstärkepegel einstellen (0–100%)
- Automatische Lautstärkeregulierung jede Sekunde
- Saubere, minimalistische Oberfläche
- Autostart beim Systemstart (optional)

## Schnellstart

### Entwicklung

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Kompilieren

```bash
npm run tauri build
```

Nach dem Kompilieren finden Sie die ausführbaren Dateien in:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Eigenständiges EXE**: `src-tauri/target/release/miclocker.exe`

## Technologie-Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: `windows`-Crate für Core Audio API-Zugriff

## Anforderungen

- Windows 10/11
- Node.js 18+
- Rust-Toolchain (zum Kompilieren aus dem Quellcode)

## Lizenz

MIT
