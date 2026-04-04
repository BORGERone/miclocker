# MicLocker

Lekki program dla systemu Windows, który blokuje głośność mikrofonu na stałym poziomie. Zapobiega przypadkowym lub niechcianym zmianom głośności.

## Funkcje

- Wybór mikrofonu spośród dostępnych urządzeń wejścia audio
- Ustawianie poziomu głośności (0–100%)
- Automatyczne utrzymanie głośności co sekundę
- Czysty, minimalistyczny interfejs
- Automatyczne uruchamianie przy starcie systemu (opcjonalnie)

## Szybki start

### Rozwój

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Budowanie

```bash
npm run tauri build
```

Po zbudowaniu pliki wykonywalne znajdziesz w:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Samodzielny EXE**: `src-tauri/target/release/miclocker.exe`

## Stos technologiczny

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: crate `windows` dla dostępu do Core Audio API

## Wymagania

- Windows 10/11
- Node.js 18+
- Toolchain Rust (do budowania ze źródeł)

## Licencja

MIT
