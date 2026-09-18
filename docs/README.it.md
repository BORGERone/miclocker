<p align="center">
  <a href="README.ru.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/1280px-Flag_of_Russia.svg.png" alt="Russian" width="30"/> Русский</a> &nbsp;&nbsp;
  <a href="../README.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/1200px-Flag_of_the_United_States.svg.png" alt="English" width="30"/> English</a> &nbsp;&nbsp;
  <a href="README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/40px-Flag_of_France.svg.png" alt="French" width="30"/> Français</a> &nbsp;&nbsp;
  <a href="README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/1280px-Flag_of_Spain_%28civil%29.svg.png?_=20110426012613" alt="Spanish" width="30"/> Español</a> &nbsp;&nbsp;
  <a href="README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/1200px-Flag_of_Germany.svg.png" alt="German" width="30"/> Deutsch</a> &nbsp;&nbsp;
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/1200px-Flag_of_Italy.svg.png" alt="Italian" width="30"/> <b>Italiano</b>
</p>
<p align="center">
  <a href="README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/500px-Flag_of_Poland.svg.png" alt="Polish" width="30"/> Polski</a> &nbsp;&nbsp;
  <a href="README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/1200px-Flag_of_Brazil.svg.png" alt="Portuguese" width="30"/> Português</a> &nbsp;&nbsp;
  <a href="README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/1200px-Flag_of_Japan.svg.png" alt="Japanese" width="30"/> 日本語</a> &nbsp;&nbsp;
  <a href="README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="Korean" width="30"/> 한국어</a> &nbsp;&nbsp;
  <a href="README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese" width="30"/> 中文</a> &nbsp;&nbsp;
  <a href="README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="Hindi" width="30"/> हिन्दी</a>
</p>

<p align="center">
  <img width="125" height="125" alt="MickLock1" src="https://github.com/user-attachments/assets/ef7235ba-f842-4f05-89c0-b04d3f216d9a" />
</p>

---

# MicLocker

MicLocker è un'applicazione desktop leggera per Windows che consente di selezionare un dispositivo audio specifico come microfono e impostarne il livello del volume. L'applicazione monitora e mantiene automaticamente il livello di volume scelto in un ciclo continuo.

## Anteprima

<p align="center">
  <img src="preview.png" alt="MicLocker" width="600">
</p>

## Funzionalità

- Selezionare il microfono dai dispositivi di input audio disponibili
- Impostare il livello del volume (0–100 %)
- Mantenimento automatico del volume ogni secondo
- Interfaccia pulita e minimalista

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
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## Stack tecnologico

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **API Windows**: Crate `windows` per l'accesso a Core Audio API

## Requisiti

- Windows 10/11
- Node.js 18+
- Toolchain Rust (per la compilazione dal codice sorgente)

## Licenza

MIT
