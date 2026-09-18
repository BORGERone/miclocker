<p align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/1280px-Flag_of_Russia.svg.png" alt="Русский" width="30"/> <b>Русский</b> &nbsp;&nbsp;
  <a href="../README.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/1200px-Flag_of_the_United_States.svg.png" alt="English" width="30"/> English</a> &nbsp;&nbsp;
  <a href="README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/40px-Flag_of_France.svg.png" alt="Français" width="30"/> Français</a> &nbsp;&nbsp;
  <a href="README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/1280px-Flag_of_Spain_%28civil%29.svg.png?_=20110426012613" alt="Español" width="30"/> Español</a> &nbsp;&nbsp;
  <a href="README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/1200px-Flag_of_Germany.svg.png" alt="Deutsch" width="30"/> Deutsch</a> &nbsp;&nbsp;
  <a href="README.it.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/1200px-Flag_of_Italy.svg.png" alt="Italiano" width="30"/> Italiano</a>
</p>
<p align="center">
  <a href="README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/500px-Flag_of_Poland.svg.png" alt="Polski" width="30"/> Polski</a> &nbsp;&nbsp;
  <a href="README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/1200px-Flag_of_Brazil.svg.png" alt="Português" width="30"/> Português</a> &nbsp;&nbsp;
  <a href="README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/1200px-Flag_of_Japan.svg.png" alt="日本語" width="30"/> 日本語</a> &nbsp;&nbsp;
  <a href="README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="한국어" width="30"/> 한국어</a> &nbsp;&nbsp;
  <a href="README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="中文" width="30"/> 中文</a> &nbsp;&nbsp;
  <a href="README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="हिन्दी" width="30"/> हिन्दी</a>
</p>

<p align="center">
  <img width="125" height="125" alt="MicLocker" src="https://github.com/user-attachments/assets/ef7235ba-f842-4f05-89c0-b04d3f216d9a" />
</p>

<h1 align="center">MicLocker</h1>

<p align="center">
  <b>Зафиксируйте громкость микрофона — и она останется такой.</b><br/>
  <sub>Мини-утилита для трея Windows, которая переустанавливает выбранный уровень громкости микрофона раз в секунду через Windows Core Audio API.</sub>
</p>

<p align="center">
  <a href="https://github.com/BORGERone/miclocker/releases/latest"><img src="https://img.shields.io/badge/version-1.0.0-00c46a?style=flat-square" alt="Версия"></a>
  <a href="https://github.com/BORGERone/miclocker/releases/latest"><img src="https://img.shields.io/badge/platform-Windows%2010%2F11%20x64-0078D6?style=flat-square&logo=windows11&logoColor=white" alt="Платформа"></a>
  <a href="#сборка-из-исходников"><img src="https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB?style=flat-square&logo=tauri&logoColor=white" alt="Tauri v2"></a>
  <a href="https://github.com/BORGERone/miclocker/stargazers"><img src="https://img.shields.io/github/stars/BORGERone/miclocker?style=flat-square&logo=github" alt="Звёзды"></a>
  <a href="https://github.com/BORGERone/miclocker/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-00c46a?style=flat-square" alt="Лицензия MIT"></a>
</p>

---

## Скриншот

<p align="center">
  <img src="preview.png" alt="Главное окно MicLocker" width="600">
</p>

MicLocker живёт в области уведомлений. Выберите устройство ввода, задайте уровень и нажмите **Запустить** — приложение удерживает этот уровень и бесшумно возвращает его, как только что-то попытается его изменить.

## Зачем это нужно?

Голосовые чаты, игры и программы для конференций любят «помогать» с уровнем микрофона: автоматическая регулировка усиления, нормализация или сброс на 100% после выхода устройства из сна. MicLocker хранит одну истину — **уровень, который вы задали**, — и проверяет его каждую секунду.

- **Блокировка уровня** — любое отклонение исправляется на следующем такте (≤ 1 секунды).
- **Переживает переподключение** — если устройство пропало и вернулось, уровень будет выставлен снова.
- **Нативный и компактный** — бинарник на Rust с интерфейсом WebView2: без Electron, без фоновых служб, без аудиодрайверов и без прав администратора.

## Возможности

| Возможность | Описание |
|---|---|
| **Выбор устройства** | Показывает все активные устройства захвата звука в системе |
| **Фиксация уровня (0–100%)** | Проверка раз в секунду через `IAudioEndpointVolume` |
| **Интерфейс без рамки** | Окно 380 × 300, скрывается при потере фокуса |
| **Работа в трее** | Левый клик по значку возвращает окно, правый — открывает меню |
| **Режим автозапуска** | Запоминает устройство и уровень и включает удержание сразу при старте MicLocker |
| **12 языков интерфейса** | Определяются из системной локали |
| **Настройки сохраняются** | Устройство, уровень и автозапуск переживают перезапуск |
| **Портативно или с установкой** | Один `.exe`, а также установщики MSI и NSIS |
| **Офлайн** | Ни телеметрии, ни аккаунтов, ни сетевых запросов (кроме кнопки GitHub) |

## Скачать

Последняя сборка — на странице [**Releases**](https://github.com/BORGERone/miclocker/releases/latest):

| Файл | Для чего | Примечание |
|---|---|---|
| `miclocker.exe` | Портативный запуск | Просто запустите — без установки и прав администратора |
| `MicLocker_1.0.0_x64_en-US.msi` | Развёртывание через MSI | Собирается командой `npm run tauri build` |
| `MicLocker_1.0.0_x64-setup.exe` | Обычная установка | Установщик NSIS с деинсталлятором, собирается командой `npm run tauri build` |

К релизу приложен портативный `miclocker.exe`, а установщики создаются при локальной сборке (см. [Сборка из исходников](#сборка-из-исходников)).

> [!NOTE]
> Сборки **не подписаны**, поэтому SmartScreen может показать предупреждение «Windows защитила ваш компьютер». Нажмите **Подробнее → Выполнить в любом случае** либо соберите приложение из исходников.

## Как пользоваться

1. **Запустите `miclocker.exe`.** Окно появляется скрытым — ищите значок MicLocker в системном трее.
2. **Щёлкните по значку в трее**, чтобы открыть окно (оно размещается в правом нижнем углу экрана).
3. **Выберите микрофон** в списке.
4. **Задайте громкость**, которую нужно удерживать (0–100%).
5. Нажмите **Запустить.** Индикатор станет зелёным и покажет *Активно*.

С этого момента уровень переустанавливается каждую секунду. Нажмите **Остановить**, чтобы снять блокировку.

| Элемент | Что делает |
|---|---|
| **Микрофон** | Устройство захвата, которое нужно удерживать (недоступно, пока блокировка активна) |
| **Громкость** | Целевой уровень в процентах; при активной блокировке применяется сразу при перетаскивании |
| **Запустить / Остановить** | Включает и выключает цикл удержания |
| **Запускать при старте** | Запоминает устройство и уровень и включает блокировку сразу при запуске MicLocker |
| **GitHub** | Открывает этот репозиторий в браузере |

**Поведение окна:** окно без рамки и прозрачное, закрывается, как только теряет фокус (настройки и уровень продолжают действовать), и в любой момент открывается заново из трея. Правый клик по значку → *Закрыть* — полный выход из приложения.

**Автозапуск вместе с Windows:** включите **Запускать при старте** и добавьте ярлык `miclocker.exe` в папку автозагрузки (нажмите <kbd>Win</kbd>+<kbd>R</kbd> и введите `shell:startup`). Тогда приложение будет запускаться в трее с уже включённой блокировкой.

## Как это работает

```
   Интерфейс React + TypeScript  (src/App.tsx)
                │  Tauri invoke()
                ▼
   Бэкенд на Rust  (src-tauri/src/main.rs)
                │  каждые 1000 мс
                ▼
   Windows Core Audio  (src-tauri/src/audio.rs)
                │  IAudioEndpointVolume::SetMasterVolumeLevelScalar()
                ▼
        уровень входа  ──►  возвращается к заданному вами значению
```

- Устройства перечисляются через `IMMDeviceEnumerator::EnumAudioEndpoints(eCapture, …)`.
- Понятные имена устройств берутся из реестра (`SYSTEM\CurrentControlSet\Enum\SWD\MMDEVAPI\…`).
- Отдельный поток просыпается раз в секунду и заново выставляет целевой уровень; флаг остановки корректно завершает его.
- Настройки хранит `tauri-plugin-store` в файле `%APPDATA%\com.miclocker.app\settings.json`.

Никаких хуков, драйверов и виртуальных устройств — используется тот же публичный интерфейс Core Audio, что и у системного ползунка громкости.

## Сборка из исходников

### Что потребуется

| Требование | Версия | Для чего |
|---|---|---|
| Windows | 10 / 11 (x64) | Запуск и сборка |
| [Node.js](https://nodejs.org/) | 18 или новее | Сборка фронтенда |
| [Rust](https://rustup.rs/) | stable, 1.77+ | Сборка бэкенда |
| [MSVC C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) | — | Компиляция крейта `windows` |
| WebView2 Runtime | — | Уже есть в Windows 11 и актуальной Windows 10 |

### Быстрый старт

```bash
# 1. Клонируем репозиторий
git clone https://github.com/BORGERone/miclocker.git
cd miclocker

# 2. Устанавливаем зависимости фронтенда
npm install

# 3. Запускаем в режиме разработки (с горячей перезагрузкой)
npm run tauri dev
```

### Релизная сборка

```bash
npm run tauri build
```

**Скрипты для Windows в один клик** (достаточно запустить `.bat`):

| Скрипт | Назначение |
|---|---|
| `run.bat` | Запуск приложения в режиме разработки |
| `dev.bat` | То же, что `run.bat`, с заголовком в консоли |
| `build.bat` | Релизная сборка; по завершении печатает пути к артефактам |

Артефакты сборки:

| Артефакт | Путь |
|---|---|
| Портативный исполняемый файл | `src-tauri/target/release/miclocker.exe` |
| Установщик MSI | `src-tauri/target/release/bundle/msi/MicLocker_1.0.0_x64_en-US.msi` |
| Установщик NSIS | `src-tauri/target/release/bundle/nsis/MicLocker_1.0.0_x64-setup.exe` |

## Структура проекта

```
miclocker/
├── src/                        # Фронтенд на React + TypeScript
│   ├── App.tsx                 # Главное окно: выбор устройства, уровень, переключатели
│   ├── App.css                 # Тема в стиле «стекла»
│   ├── main.tsx                # Точка входа, автозакрытие при потере фокуса
│   └── i18n/                   # 12 языков интерфейса (en, ru, de, zh, …)
├── src-tauri/                  # Бэкенд на Rust (Tauri v2)
│   ├── src/main.rs             # Команды, значок в трее, цикл удержания раз в секунду
│   ├── src/audio.rs            # Core Audio: перечисление устройств, установка громкости
│   ├── capabilities/           # Набор разрешений Tauri
│   └── tauri.conf.json         # Окно (380×300, без рамки, прозрачное) и конфигурация сборки
├── docs/                       # Переводы README + скриншот
├── build.bat · dev.bat · run.bat
└── package.json
```

## Технологии

| Уровень | Технология |
|---|---|
| Интерфейс | React 18, TypeScript, Vite 5 |
| Локализация | i18next + react-i18next (12 языков) |
| Оболочка / IPC | Tauri v2 (значок в трее, управление окном, команды `invoke`) |
| Ядро | Rust (поток удержания уровня раз в секунду) |
| Аудио | Крейт `windows` — Windows Core Audio API (`IAudioEndpointVolume`) |
| Хранение настроек | `tauri-plugin-store` → `settings.json` |

## Решение проблем

| Симптом | Что делать |
|---|---|
| *Нет доступных микрофонов* | Разрешите доступ к микрофону в **Параметры → Конфиденциальность → Микрофон** и убедитесь, что устройство включено в **Параметры → Система → Звук → Ввод**. Затем откройте приложение заново. |
| Уровень возвращается к прежнему значению | MicLocker больше его не удерживает — откройте окно из трея и снова нажмите **Запустить** (индикатор должен стать зелёным). |
| После запуска ничего не появляется | Так и задумано: окно стартует скрытым в трее. Щёлкните по значку в трее или правый клик → *Открыть*. |
| «Windows защитила ваш компьютер» | Бинарник не подписан — нажмите **Подробнее → Выполнить в любом случае** или соберите приложение сами. |
| Антивирус ругается на портативный `.exe` | Частое ложное срабатывание для неподписанных однфайловых сборок на Rust. Используйте установщик или соберите из исходников. |
| Окно закрывается при клике в стороне | Это ожидаемое поведение — вернуть окно можно кликом по значку в трее. Блокировка громкости продолжает работать. |

## Языки интерфейса

В приложении 12 локалей, язык определяется по системному: `en`, `ru`, `de`, `es`, `fr`, `it`, `pl`, `pt`, `ja`, `ko`, `zh`, `hi`.

**Добавить язык** — три шага:

1. Скопируйте `src/i18n/en.json` в `src/i18n/xx.json` и переведите значения.
2. Импортируйте и зарегистрируйте файл в `src/i18n/index.ts`.
3. (Необязательно) Добавьте перевод README в папку `docs/` и ссылку на него в списке языков вверху.

## Известные ограничения

- **Только Windows** — приложение построено непосредственно на Windows Core Audio API.
- **Одно устройство за раз** — MicLocker удерживает уровень одного устройства ввода за сессию.
- **Отклонение до ~1 секунды** — проверка идёт раз в секунду, поэтому изменение, сделанное между тактами, будет исправлено на следующем.
- **Приложение должно быть запущено**, иначе блокировка не действует; выход из трея снимает её.
- Контекстное меню в трее пока только на русском; само окно следует за языком системы.

## Участие в разработке

Issues и pull request'ы приветствуются — сообщения об ошибках, новые языки интерфейса и переводы README одинаково полезны. Пожалуйста, делайте изменения небольшими и по существу и описывайте, как вы проверяли их в Windows.

## Лицензия

Проект распространяется по [лицензии MIT](https://github.com/BORGERone/miclocker/blob/main/LICENSE).

---

<p align="center">
  <sub>Если MicLocker оказался полезен, звезда на репозитории поможет другим его найти.</sub>
</p>
