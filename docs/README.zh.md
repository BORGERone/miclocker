# MicLocker

一款轻量级 Windows 实用工具，可将麦克风音量锁定在固定水平。防止意外或不需要的音量更改。

## 功能

- 从可用音频输入设备中选择麦克风
- 设置音量级别（0–100%）
- 每秒自动维持音量
- 简洁、极简的界面
- 系统启动时自动启动（可选）

## 快速入门

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
- **独立 EXE**: `src-tauri/target/release/miclocker.exe`

## 技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri v2
- **Windows API**: 使用 `windows` crate 访问核心音频 API

## 系统要求

- Windows 10/11
- Node.js 18+
- Rust 工具链（从源代码构建时需要）

## 许可证

MIT
