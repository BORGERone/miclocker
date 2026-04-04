# MicLocker

Un utilitaire Windows léger qui verrouille le volume de votre microphone à un niveau fixe. Empêche les modifications de volume accidentelles ou indésirables.

## Fonctionnalités

- Sélection du microphone parmi les périphériques d'entrée audio disponibles
- Réglage du niveau de volume (0–100%)
- Maintien automatique du volume chaque seconde
- Interface propre et minimaliste
- Démarrage automatique au lancement du système (optionnel)

## Démarrage rapide

### Développement

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Compilation

```bash
npm run tauri build
```

Après la compilation, vous trouverez les exécutables dans :

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **EXE autonome**: `src-tauri/target/release/miclocker.exe`

## Stack technique

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: crate `windows` pour l'accès à l'API Core Audio

## Prérequis

- Windows 10/11
- Node.js 18+
- Toolchain Rust (pour la compilation depuis les sources)

## Licence

MIT
