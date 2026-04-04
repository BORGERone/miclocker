# MicLocker

Лёгкая утилита для Windows, которая фиксирует громкость микрофона на заданном уровне. Предотвращает случайные или нежелательные изменения громкости.

## Возможности

- Выбор микрофона из доступных аудиоустройств
- Установка уровня громкости (0–100%)
- Автоматическое поддержание громкости каждую секунду
- Чистый минималистичный интерфейс
- Автозапуск при старте системы (опционально)

## Быстрый запуск

### Разработка

```bash
# Установка зависимостей
npm install

# Запуск в режиме разработки
npm run tauri dev
```

### Сборка

```bash
npm run tauri build
```

После сборки исполняемые файлы находятся в:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## Технологии

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: крейт `windows` для доступа к Core Audio API

## Требования

- Windows 10/11
- Node.js 18+
- Rust toolchain (для сборки из исходников)

## Лицензия

MIT
