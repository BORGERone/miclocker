#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use audio::{get_microphones as get_mics, set_microphone_volume as set_vol};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use windows::Win32::System::Threading::Sleep;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MicrophoneInfo {
    device_id: String,
    name: String,
}

static SELECTED_DEVICE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static TARGET_VOLUME: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(100));
static IS_MAINTAINING: AtomicBool = AtomicBool::new(false);
static STOP_FLAG: AtomicBool = AtomicBool::new(false);
static AUTO_START: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(Debug, Serialize, Deserialize)]
struct AppSettings {
    selected_device: Option<String>,
    volume: u32,
    auto_start: bool,
}

const STORE_FILE: &str = "settings.json";

fn start_maintain_thread() {
    IS_MAINTAINING.store(true, Ordering::SeqCst);
    STOP_FLAG.store(false, Ordering::SeqCst);

    std::thread::spawn(|| {
        while !STOP_FLAG.load(Ordering::SeqCst) {
            if let Some(ref dev_id) = *SELECTED_DEVICE.lock() {
                let vol = *TARGET_VOLUME.lock();
                let _ = set_vol(dev_id, vol);
            }
            unsafe { Sleep(1000); }
        }
        IS_MAINTAINING.store(false, Ordering::SeqCst);
    });
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle) -> Result<(), String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    let settings = AppSettings {
        selected_device: SELECTED_DEVICE.lock().clone(),
        volume: *TARGET_VOLUME.lock(),
        auto_start: *AUTO_START.lock(),
    };
    store.set("settings", serde_json::to_value(&settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())
}

#[tauri::command]
fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    match store.get("settings") {
        Some(val) => {
            let settings: AppSettings = serde_json::from_value(val).map_err(|e| e.to_string())?;
            *SELECTED_DEVICE.lock() = settings.selected_device.clone();
            *TARGET_VOLUME.lock() = settings.volume;
            *AUTO_START.lock() = settings.auto_start;
            Ok(settings)
        }
        None => Ok(AppSettings {
            selected_device: None,
            volume: 100,
            auto_start: false,
        }),
    }
}

#[tauri::command]
fn get_microphones() -> Result<Vec<MicrophoneInfo>, String> {
    get_mics().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_microphone_volume(device_id: String, level: u32, app: tauri::AppHandle) -> Result<(), String> {
    set_vol(&device_id, level).map_err(|e| e.to_string())?;
    *TARGET_VOLUME.lock() = level;
    let _ = save_settings(app);
    Ok(())
}

#[tauri::command]
fn start_auto_maintain(device_id: String, level: u32, app: tauri::AppHandle) -> Result<(), String> {
    if IS_MAINTAINING.load(Ordering::SeqCst) {
        return Ok(());
    }

    *SELECTED_DEVICE.lock() = Some(device_id.clone());
    *TARGET_VOLUME.lock() = level;
    let _ = save_settings(app.clone());

    set_vol(&device_id, level).map_err(|e| e.to_string())?;
    start_maintain_thread();

    Ok(())
}

#[tauri::command]
fn stop_auto_maintain() -> Result<(), String> {
    STOP_FLAG.store(true, Ordering::SeqCst);
    std::thread::sleep(Duration::from_millis(1100));
    IS_MAINTAINING.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn is_maintaining() -> bool {
    IS_MAINTAINING.load(Ordering::SeqCst)
}

#[tauri::command]
fn select_microphone(device_id: String, app: tauri::AppHandle) -> Result<(), String> {
    *SELECTED_DEVICE.lock() = Some(device_id);
    save_settings(app)
}

#[tauri::command]
fn set_auto_start(enabled: bool, app: tauri::AppHandle) -> Result<(), String> {
    *AUTO_START.lock() = enabled;
    save_settings(app)
}

#[tauri::command]
fn get_auto_start() -> bool {
    *AUTO_START.lock()
}

#[tauri::command]
fn show_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        // Позиционируем окно в правом нижнем углу
        unsafe {
            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            let win_w: i32 = 380;
            let win_h: i32 = 300;
            let x = screen_w - win_w - 20;
            let y = screen_h - win_h - 60;
            let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
        }
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

#[tauri::command]
fn toggle_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            unsafe {
                let screen_w = GetSystemMetrics(SM_CXSCREEN);
                let screen_h = GetSystemMetrics(SM_CYSCREEN);
                let win_w: i32 = 380;
                let win_h: i32 = 300;
                let x = screen_w - win_w - 20;
                let y = screen_h - win_h - 60;
                let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
            }
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
    Ok(())
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    STOP_FLAG.store(true, Ordering::SeqCst);
    std::thread::sleep(Duration::from_millis(200));
    app.exit(0);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Загружаем настройки
            let mut auto_starting = false;
            if let Ok(store) = app.store(STORE_FILE) {
                if let Some(val) = store.get("settings") {
                    if let Ok(settings) = serde_json::from_value::<AppSettings>(val) {
                        *SELECTED_DEVICE.lock() = settings.selected_device.clone();
                        *TARGET_VOLUME.lock() = settings.volume;
                        *AUTO_START.lock() = settings.auto_start;

                        if settings.auto_start {
                            if let Some(ref dev_id) = settings.selected_device {
                                let dev_id = dev_id.clone();
                                let vol = settings.volume;
                                let _ = set_vol(&dev_id, vol);
                                start_maintain_thread();
                                auto_starting = true;
                            }
                        }
                    }
                }
            }

            // Создаём меню для трея
            let toggle_item = MenuItem::with_id(app, "toggle", "Открыть", true, None::<&str>)?;
            let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Закрыть", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&toggle_item, &separator, &quit_item])?;

            // Создаём иконку в трее
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("MicLocker")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "toggle" => {
                        let _ = show_window(app.clone());
                    }
                    "quit" => {
                        quit_app(app.clone());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event {
                        let app = tray.app_handle().clone();
                        let _ = show_window(app);
                    }
                })
                .build(app)?;

            // Если автозапуск — показываем уведомление в трее, но окно не показываем
            if auto_starting {
                // Окно скрыто, но поддержание запущено
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_microphones,
            set_microphone_volume,
            start_auto_maintain,
            stop_auto_maintain,
            is_maintaining,
            load_settings,
            save_settings,
            select_microphone,
            set_auto_start,
            get_auto_start,
            show_window,
            hide_window,
            toggle_window,
            quit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
