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
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="Korean" width="30"/> <b>한국어</b> &nbsp;&nbsp;
  <a href="README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese" width="30"/> 中文</a> &nbsp;&nbsp;
  <a href="README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="Hindi" width="30"/> हिन्दी</a>
</p>

<p align="center">
  <img width="125" height="125" alt="MickLock1" src="https://github.com/user-attachments/assets/ef7235ba-f842-4f05-89c0-b04d3f216d9a" />
</p>

---

# MicLocker

MicLocker는 Windows용 경량 데스크톱 애플리케이션으로, 특정 오디오 장치를 마이크로 선택하고 볼륨 레벨을 설정할 수 있습니다. 애플리케이션은 지속적인 루프에서 선택된 볼륨 레벨을 자동으로 모니터링하고 유지합니다.

## 미리보기

<p align="center">
  <img src="preview.png" alt="MicLocker" width="600">
</p>

## 기능

- 사용 가능한 오디오 입력 장치에서 마이크 선택
- 볼륨 레벨 설정 (0–100%)
- 매초 볼륨 자동 유지
- 깔끔하고 미니멀한 인터페이스

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

빌드 후 다음 위치에서 실행 파일을 찾을 수 있습니다:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## 기술 스택

- **프론트엔드**: React + TypeScript + Vite
- **백엔드**: Rust + Tauri v2
- **Windows API**: Core Audio API 접근을 위한 `windows` 크레이트

## 요구 사항

- Windows 10/11
- Node.js 18+
- Rust 툴체인 (소스에서 빌드할 경우)

## 라이선스

MIT
