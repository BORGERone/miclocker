# MicLocker

가벼운 Windows 유틸리티로 마이크 음량을 고정 수준으로 유지합니다. 실수나 원치 않는 음량 변경을 방지합니다.

## 기능

- 사용 가능한 오디오 입력 장치에서 마이크 선택
- 음량 레벨 설정 (0–100%)
- 1초마다 자동 음량 유지
- 깔끔하고 미니멀한 인터페이스
- 시스템 부팅 시 자동 시작 (선택 사항)

## 빠른 시작

### 개발

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### 빌드

```bash
npm run tauri build
```

빌드 후 실행 파일이 다음 위치에 생성됩니다:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **독립 실행형 EXE**: `src-tauri/target/release/miclocker.exe`

## 기술 스택

- **프론트엔드**: React + TypeScript + Vite
- **백엔드**: Rust + Tauri v2
- **Windows API**: Core Audio API 접근을 위한 `windows` 크레이트

## 시스템 요구 사항

- Windows 10/11
- Node.js 18+
- Rust 툴체인 (소스에서 빌드하는 경우)

## 라이선스

MIT
