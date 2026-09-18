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
  <a href="README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese" width="30"/> 中文</a> &nbsp;&nbsp;
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="Hindi" width="30"/> <b>हिन्दी</b>
</p>

<p align="center">
  <img width="125" height="125" alt="MickLock1" src="https://github.com/user-attachments/assets/ef7235ba-f842-4f05-89c0-b04d3f216d9a" />
</p>

---

# MicLocker

MicLocker Windows के लिए एक हल्की डेस्कटॉप एप्लिकेशन है जो आपको माइक्रोफोन के रूप में एक विशिष्ट ऑडियो डिवाइस चुनने और उसके वॉल्यूम स्तर को सेट करने देती है। एप्लिकेशन एक निरंतर लूप में चुने गए वॉल्यूम स्तर को स्वचालित रूप से मॉनिटर और बनाए रखती है।

## स्क्रीनशॉट

<p align="center">
  <img src="preview.png" alt="MicLocker" width="600">
</p>

## विशेषताएं

- उपलब्ध ऑडियो इनपुट डिवाइस से माइक्रोफोन चुनें
- वॉल्यूम स्तर सेट करें (0–100%)
- हर सेकंड स्वचालित वॉल्यूम रखरखाव
- साफ, न्यूनतम इंटरफ़ेस

## त्वरित प्रारंभ

### विकास

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### बिल्ड

```bash
npm run tauri build
```

बिल्ड करने के बाद, आपको निम्नलिखित स्थानों पर एक्जीक्यूटेबल्स मिलेंगे:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## तकनीकी स्टैक

- **फ्रंटएंड**: React + TypeScript + Vite
- **बैकएंड**: Rust + Tauri v2
- **Windows API**: Core Audio API एक्सेस के लिए `windows` क्रेट

## आवश्यकताएं

- Windows 10/11
- Node.js 18+
- Rust टूलचेन (सोर्स से बिल्ड करने के लिए)

## लाइसेंस

MIT
