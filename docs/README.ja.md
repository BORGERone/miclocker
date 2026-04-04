# MicLocker

軽量な Windows ユーティリティで、マイク音量を一定レベルに固定します。意図しないまたは不要な音量変更を防止します。

## 機能

- 使用可能なオーディオ入力デバイスからマイクを選択
- 音量レベルの設定（0–100%）
- 1秒ごとに音量を自動維持
- クリーンでミニマルなインターフェース
- システム起動時の自動起動（オプション）

## クイックスタート

### 開発

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### ビルド

```bash
npm run tauri build
```

ビルド後、実行ファイルは以下の場所に出力されます：

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **スタンドアロン EXE**: `src-tauri/target/release/miclocker.exe`

## 技術スタック

- **フロントエンド**: React + TypeScript + Vite
- **バックエンド**: Rust + Tauri v2
- **Windows API**: Core Audio API アクセス用 `windows` クレート

## 動作環境

- Windows 10/11
- Node.js 18+
- Rust ツールチェーン（ソースからビルドする場合）

## ライセンス

MIT
