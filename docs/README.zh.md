<p align="center">
  <a href="README.ru.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/1280px-Flag_of_Russia.svg.png" alt="Russian" width="30"/> Русский</a> &nbsp;&nbsp;
  <a href="../README.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/1200px-Flag_of_the_United_States.svg.png" alt="English" width="30"/> English</a> &nbsp;&nbsp;
  <a href="README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/40px-Flag_of_France.svg.png" alt="French" width="30"/> Français</a> &nbsp;&nbsp;
  <a href="README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/1280px-Flag_of_Spain_%28civil%29.svg.png?_=20110426012613" alt="Spanish" width="30"/> Español</a> &nbsp;&nbsp;
  <a href="README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/1200px-Flag_of_Germany.svg.png" alt="German" width="30"/> Deutsch</a> &nbsp;&nbsp;
  <a href="README.it.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/1200px-Flag_of_Italy.svg.png" alt="Italian" width="30"/> Italiano</a>
</p>
<p align="center">
  <a href="README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/500px-Flag_of_Poland.svg.png" alt="Polish" width="30"/> Polski</a> &nbsp;&nbsp;
  <a href="README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/1200px-Flag_of_Brazil.svg.png" alt="Portuguese" width="30"/> Português</a> &nbsp;&nbsp;
  <a href="README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/1200px-Flag_of_Japan.svg.png" alt="Japanese" width="30"/> 日本語</a> &nbsp;&nbsp;
  <a href="README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="Korean" width="30"/> 한국어</a> &nbsp;&nbsp;
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese" width="30"/> <b>中文</b> &nbsp;&nbsp;
  <a href="README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="Hindi" width="30"/> हिन्दी</a>
</p>

<p align="center">
  <img src="hero.png" alt="MicLocker 横幅" width="880">
</p>

---

# MicLocker

MicLocker 是一款适用于 Windows 的轻量级桌面应用程序，可选择特定的音频设备作为麦克风并设置音量级别。应用程序会在持续循环中自动监控并维持所选音量级别。

## 预览

<p align="center">
  <img src="preview.png" alt="MicLocker" width="520">
</p>

## 功能

- 从可用的音频输入设备中选择麦克风
- 设置音量级别（0–100%）
- 每秒自动维持音量
- 简洁的最小化界面

## 快速开始

### 开发

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### 构建

```bash
npm run tauri build
```

构建完成后，可执行文件位于：

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## 技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri v2
- **Windows API**: 使用 `windows` crate 访问 Core Audio API

## 系统要求

- Windows 10/11
- Node.js 18+
- Rust 工具链（从源码构建时需要）

## 许可证

MIT
