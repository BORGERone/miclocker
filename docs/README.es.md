# MicLocker

Una utilidad ligera de Windows que fija el volumen del micrófono en un nivel constante. Evita cambios de volumen accidentales o no deseados.

## Características

- Selecciona el micrófono entre los dispositivos de entrada de audio disponibles
- Establece el nivel de volumen (0–100%)
- Mantenimiento automático del volumen cada segundo
- Interfaz limpia y minimalista
- Inicio automático con el sistema (opcional)

## Inicio rápido

### Desarrollo

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Compilación

```bash
npm run tauri build
```

Después de compilar, encontrarás los ejecutables en:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **EXE independiente**: `src-tauri/target/release/miclocker.exe`

## Tecnologías

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: crate `windows` para acceso a Core Audio API

## Requisitos

- Windows 10/11
- Node.js 18+
- Toolchain de Rust (para compilar desde el código fuente)

## Licencia

MIT
