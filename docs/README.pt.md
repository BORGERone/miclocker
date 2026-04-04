# MicLocker

Um utilitário leve para Windows que trava o volume do microfone em um nível fixo. Evita alterações acidentais ou indesejadas de volume.

## Recursos

- Selecione o microfone entre os dispositivos de entrada de áudio disponíveis
- Defina o nível de volume (0–100%)
- Manutenção automática do volume a cada segundo
- Interface limpa e minimalista
- Inicialização automática com o sistema (opcional)

## Início rápido

### Desenvolvimento

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Compilação

```bash
npm run tauri build
```

Após a compilação, os executáveis estarão em:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **EXE independente**: `src-tauri/target/release/miclocker.exe`

## Stack de tecnologias

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: crate `windows` para acesso à Core Audio API

## Requisitos

- Windows 10/11
- Node.js 18+
- Toolchain Rust (para compilar a partir do código-fonte)

## Licença

MIT
