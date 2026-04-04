# MicLocker

एक हल्की Windows उपयोगिता जो आपके माइक्रोफ़ोन वॉल्यूम को एक निश्चित स्तर पर लॉक करती है। आकस्मिक या अवांछित वॉल्यूम परिवर्तनों को रोकती है।

## सुविधाएँ

- उपलब्ध ऑडियो इनपुट उपकरणों से माइक्रोफ़ोन चुनें
- वॉल्यूम स्तर सेट करें (0–100%)
- हर सेकंड स्वचालित वॉल्यूम रखरखाव
- साफ, न्यूनतम इंटरफ़ेस
- सिस्टम बूट पर ऑटो-स्टार्ट (वैकल्पिक)

## त्वरित प्रारंभ

### डेवलपमेंट

```bash
# निर्भरताएँ इंस्टॉल करें
npm install

# डेवलपमेंट मोड में चलाएँ
npm run tauri dev
```

### बिल्ड

```bash
npm run tauri build
```

बिल्ड के बाद, एक्जीक्यूटेबल्स यहाँ मिलेंगे:

- **MSI**: `src-tauri/target/release/bundle/msi/MicLocker_*_x64_en-US.msi`
- **NSIS**: `src-tauri/target/release/bundle/nsis/MicLocker_*_x64-setup.exe`
- **Standalone EXE**: `src-tauri/target/release/miclocker.exe`

## टेक स्टैक

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Windows API**: Core Audio API एक्सेस के लिए `windows` crate

## आवश्यकताएँ

- Windows 10/11
- Node.js 18+
- Rust toolchain (सोर्स से बिल्ड करने के लिए)

## लाइसेंस

MIT
