<p align="center">
  <a href="docs/README.ru.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/1280px-Flag_of_Russia.svg.png" alt="Русский" width="30"/> Русский</a> &nbsp;&nbsp;
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/1200px-Flag_of_the_United_States.svg.png" alt="English" width="30"/> <b>English</b> &nbsp;&nbsp;
  <a href="docs/README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/40px-Flag_of_France.svg.png" alt="Français" width="30"/> Français</a> &nbsp;&nbsp;
  <a href="docs/README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/1280px-Flag_of_Spain_%28civil%29.svg.png?_=20110426012613" alt="Español" width="30"/> Español</a> &nbsp;&nbsp;
  <a href="docs/README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/1200px-Flag_of_Germany.svg.png" alt="Deutsch" width="30"/> Deutsch</a> &nbsp;&nbsp;
  <a href="docs/README.it.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/1200px-Flag_of_Italy.svg.png" alt="Italiano" width="30"/> Italiano</a>
</p>
<p align="center">
  <a href="docs/README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/500px-Flag_of_Poland.svg.png" alt="Polski" width="30"/> Polski</a> &nbsp;&nbsp;
  <a href="docs/README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/1200px-Flag_of_Brazil.svg.png" alt="Português" width="30"/> Português</a> &nbsp;&nbsp;
  <a href="docs/README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/1200px-Flag_of_Japan.svg.png" alt="日本語" width="30"/> 日本語</a> &nbsp;&nbsp;
  <a href="docs/README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="한국어" width="30"/> 한국어</a> &nbsp;&nbsp;
  <a href="docs/README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="中文" width="30"/> 中文</a> &nbsp;&nbsp;
  <a href="docs/README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="हिन्दी" width="30"/> हिन्दी</a>
</p>

<p align="center">
  <img width="125" height="125" alt="MicLocker" src="https://github.com/user-attachments/assets/ef7235ba-f842-4f05-89c0-b04d3f216d9a" />
</p>

<h1 align="center">MicLocker</h1>

<p align="center">
  <b>Pin your microphone level — and keep it there.</b><br/>
  <sub>A tiny Windows tray utility that re-applies your microphone volume once per second through the Windows Core Audio API.</sub>
</p>

<p align="center">
  <a href="https://github.com/BORGERone/miclocker/releases/latest"><img src="https://img.shields.io/badge/version-1.0.0-00c46a?style=flat-square" alt="Version"></a>
  <a href="https://github.com/BORGERone/miclocker/releases/latest"><img src="https://img.shields.io/badge/platform-Windows%2010%2F11%20x64-0078D6?style=flat-square&logo=windows11&logoColor=white" alt="Platform"></a>
  <a href="#build-from-source"><img src="https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB?style=flat-square&logo=tauri&logoColor=white" alt="Tauri v2"></a>
  <a href="https://github.com/BORGERone/miclocker/stargazers"><img src="https://img.shields.io/github/stars/BORGERone/miclocker?style=flat-square&logo=github" alt="Stars"></a>
  <a href="https://github.com/BORGERone/miclocker/network/members"><img src="https://img.shields.io/github/forks/BORGERone/miclocker?style=flat-square&logo=github" alt="Forks"></a>
  <a href="https://github.com/BORGERone/miclocker/issues"><img src="https://img.shields.io/github/issues/BORGERone/miclocker?style=flat-square&logo=github" alt="Issues"></a>
  <a href="https://github.com/BORGERone/miclocker/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-00c46a?style=flat-square" alt="MIT License"></a>
</p>

---

## Preview

<p align="center">
  <img src="docs/preview.png" alt="MicLocker main window" width="600">
</p>

MicLocker lives in the notification area. Pick an input device, choose a level, hit **Start** — the app locks that level and quietly restores it the moment anything tries to change it.

## Why MicLocker?

Voice chats, games and conferencing apps love to "help" with your input level: auto-gain, normalization, or a full 100% reset after the device wakes up from sleep. MicLocker keeps a single truth — **the level you set** — and enforces it every second.

- 🔒 **Lock the level** — any drift is corrected on the next tick (≤ 1 second).
- 🎮 **No more blown-out voice chat** — nobody gets a 100% mic in their ears after a game update resets your slider.
- 🔌 **Survives reconnects** — if the device disappears and comes back, the level is re-applied automatically.
- 🪶 **Featherweight** — a native Rust binary with a WebView2 UI; no Electron, no background services, no drivers, no admin rights.

## Features

| Feature | Description |
|---|---|
| 🎙️ **Device picker** | Lists every active capture endpoint on the system |
| 🎚️ **Level lock (0–100%)** | Re-applied once per second through `IAudioEndpointVolume` |
| 🖥️ **Frameless glass UI** | A 380 × 300 window that fades away as soon as it loses focus |
| 🧲 **Tray resident** | Left-click the tray icon to bring the window back, right-click for the menu |
| 🚀 **Launch-at-startup mode** | Remembers device + level and starts maintaining as soon as MicLocker starts |
| 🌍 **12 UI languages** | Picked automatically from the system locale |
| 💾 **Settings persisted** | Device, level and auto-start survive restarts |
| 📦 **Portable or installed** | One `.exe`, plus MSI and NSIS installers |
| 🕵️ **Fully offline** | No telemetry, no accounts, no network calls (except the GitHub button) |

## Download

Grab the latest build from the [**Releases**](https://github.com/BORGERone/miclocker/releases/latest) page:

| File | Best for | Notes |
|---|---|---|
| `miclocker.exe` | Portable use | Just run it — no install, no admin rights |
| `MicLocker_1.0.0_x64_en-US.msi` | Managed / MSI deployment | Produced by `npm run tauri build` |
| `MicLocker_1.0.0_x64-setup.exe` | Regular desktop install | NSIS installer with uninstaller, produced by `npm run tauri build` |

The portable `miclocker.exe` is attached to the release; the installers are generated by a local build (see [Build from source](#build-from-source)).

> [!NOTE]
> The builds are **not code-signed**, so Windows SmartScreen may show a "Windows protected your PC" prompt. Click **More info → Run anyway**, or build the app yourself from source.

## Usage

1. **Run `miclocker.exe`.** The window starts hidden — look for the MicLocker icon in the system tray.
2. **Click the tray icon** to open the window (it is placed in the bottom-right corner of the screen).
3. **Select your microphone** from the dropdown.
4. **Set the volume** you want to lock (0–100%).
5. Press **Start.** The status dot turns green and shows *Active*.

From that point on the level is re-applied every second. Press **Stop** to release the lock.

| Control | What it does |
|---|---|
| **Microphone** | The capture endpoint to lock (disabled while the lock is active) |
| **Volume** | Target level in percent; dragging it while active applies immediately |
| **Start / Stop** | Toggles the maintenance loop |
| **Launch at startup** | Keeps your device + level and turns the lock on automatically when MicLocker starts |
| **GitHub** | Opens this repository in your browser |

**Window behaviour:** the window is frameless and transparent, closes itself as soon as it loses focus (your settings stay applied) and can be reopened from the tray icon at any time. Right-click the tray icon → *Закрыть* (quit) to exit completely.

> [!TIP]
> To make MicLocker start with Windows, enable **Launch at startup** here, then add a shortcut to `miclocker.exe` into `shell:startup` (press <kbd>Win</kbd>+<kbd>R</kbd>, type `shell:startup`). The app will start in the tray with the lock already engaged.

## How it works

```
   React + TypeScript UI  (src/App.tsx)
                │  Tauri invoke()
                ▼
   Rust backend  (src-tauri/src/main.rs)
                │  every 1000 ms
                ▼
   Windows Core Audio  (src-tauri/src/audio.rs)
                │  IAudioEndpointVolume::SetMasterVolumeLevelScalar()
                ▼
        🎙️  input level  ──►  forced back to the value you chose
```

- Devices are enumerated with `IMMDeviceEnumerator::EnumAudioEndpoints(eCapture, …)`.
- Device friendly names come from the registry (`SYSTEM\CurrentControlSet\Enum\SWD\MMDEVAPI\…`).
- A dedicated thread wakes up once per second and re-applies the target level; a stop flag shuts it down cleanly.
- Settings are stored by `tauri-plugin-store` in `%APPDATA%\com.miclocker.app\settings.json`.

No hooks, no audio drivers, no virtual devices — only the same public Core Audio interface the Windows volume slider uses.

## Build from source

### Prerequisites

| Requirement | Version | Needed for |
|---|---|---|
| Windows | 10 / 11 (x64) | Running and building |
| [Node.js](https://nodejs.org/) | 18 or newer | Frontend build |
| [Rust](https://rustup.rs/) | stable, 1.77+ | Backend build |
| [MSVC C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) | — | Compiling the `windows` crate |
| WebView2 Runtime | — | Preinstalled on Windows 11 and current Windows 10 |

### Quick start

```bash
# 1. Clone the repository
git clone https://github.com/BORGERone/miclocker.git
cd miclocker

# 2. Install frontend dependencies
npm install

# 3. Run in development mode (hot reload)
npm run tauri dev
```

### Release build

```bash
npm run tauri build
```

**One-click Windows scripts** (just run the `.bat` file):

| Script | Purpose |
|---|---|
| `run.bat` | Run the app in development mode |
| `dev.bat` | Same as `run.bat`, with console banner |
| `build.bat` | Release build; prints the artifact paths when finished |

Build artifacts:

| Artifact | Path |
|---|---|
| Portable executable | `src-tauri/target/release/miclocker.exe` |
| MSI installer | `src-tauri/target/release/bundle/msi/MicLocker_1.0.0_x64_en-US.msi` |
| NSIS installer | `src-tauri/target/release/bundle/nsis/MicLocker_1.0.0_x64-setup.exe` |

## Project structure

```
miclocker/
├── src/                        # React + TypeScript frontend
│   ├── App.tsx                 # Main window: device picker, level, toggles
│   ├── App.css                 # Glassmorphism theme
│   ├── main.tsx                # Entry point, auto-hide on focus loss
│   └── i18n/                   # 12 UI locales (en, ru, de, zh, …)
├── src-tauri/                  # Rust backend (Tauri v2)
│   ├── src/main.rs             # Commands, tray icon, 1 Hz maintenance loop
│   ├── src/audio.rs            # Core Audio: enumerate devices, set master volume
│   ├── capabilities/           # Tauri permission set
│   └── tauri.conf.json         # Window (380×300, frameless, transparent) & bundle config
├── docs/                       # Translated READMEs + preview image
├── build.bat · dev.bat · run.bat
└── package.json
```

## Tech stack

| Layer | Technology |
|---|---|
| UI | React 18, TypeScript, Vite 5 |
| Localisation | i18next + react-i18next (12 languages) |
| Shell / IPC | Tauri v2 (tray icon, window management, `invoke` commands) |
| Core | Rust (a 1 Hz maintenance thread) |
| Audio | `windows` crate — Windows Core Audio API (`IAudioEndpointVolume`) |
| Persistence | `tauri-plugin-store` → `settings.json` |

## Troubleshooting

| Symptom | Fix |
|---|---|
| *No microphones available* | Enable microphone access in **Settings → Privacy → Microphone**, and make sure the device is enabled in **Settings → System → Sound → Input**. Then reopen the app. |
| The level snaps back to the old value | MicLocker isn't maintaining it anymore — open the tray icon and press **Start** again (the dot must be green). |
| Nothing appears after launching | By design: the window starts hidden in the tray. Left-click the tray icon, or right-click → *Открыть*. |
| "Windows protected your PC" | The binary is unsigned — **More info → Run anyway**, or build it yourself. |
| Antivirus flags the portable `.exe` | Common false positive for unsigned single-file Rust binaries. Use the installer, or compile from source. |
| The window closes when I click elsewhere | Intended behaviour — click the tray icon to bring it back. Your volume lock keeps running. |

## Interface languages

The UI ships with 12 locales and follows the system language:

<p align="center">
  🇬🇧 English · 🇷🇺 Русский · 🇩🇪 Deutsch · 🇪🇸 Español · 🇫🇷 Français · 🇮🇹 Italiano<br/>
  🇵🇱 Polski · 🇵🇹 Português · 🇯🇵 日本語 · 🇰🇷 한국어 · 🇨🇳 中文 · 🇮🇳 हिन्दी
</p>

**Adding a language** is a three-step job:

1. Copy `src/i18n/en.json` to `src/i18n/xx.json` and translate the values.
2. Import and register it in `src/i18n/index.ts`.
3. (Optional) Add a translated README under `docs/` and link it in the language list at the top.

## Known limitations

- **Windows only** — the app is built directly on the Windows Core Audio API.
- **One endpoint at a time** — MicLocker locks a single input device per session.
- **Up to ~1 second of drift** — enforcement happens on a 1-second tick, so a change made in between is corrected on the next pass.
- **The app must be running** for the lock to hold; quitting from the tray releases it.
- The tray context menu is currently Russian-only; the main window follows your system language.

## Contributing

Issues and pull requests are welcome — bug reports, new UI languages and README translations are all appreciated. Please keep changes small and focused, and describe how you tested them on Windows.

## License

Released under the [MIT License](https://github.com/BORGERone/miclocker/blob/main/LICENSE).

---

<p align="center">
  <sub>If MicLocker keeps your microphone in check, a ⭐ on the repository helps other people find it.</sub>
</p>
