# MicLocker

Un'utilità leggera per Windows che blocca il volume del microfono a un livello fisso. Previene modifiche accidentali o indesiderate del volume.

## Caratteristiche

- Selezione del microfono dai dispositivi di input audio disponibili
- Impostazione del livello del volume (0–100%)
- Mantenimento automatico del volume ogni secondo
- Interfaccia pulita e minimalista
- Avvio automatico all'accensione del sistema (opzionale)

## Avvio rapido

### Sviluppo

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Compilazione

```bash
npm run tauri build
```

Dopo la compilazione, troverai gli eseguibili in:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **EXE autonomo**: `src-tauri/target/release/miclocker.exe`

## Stack tecnologico

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: crate `windows` per l'accesso alla Core Audio API

## Requisiti

- Windows 10/11
- Node.js 18+
- Toolchain Rust (per la compilazione dai sorgenti)

## Licenza

MIT
