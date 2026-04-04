use windows::core::{GUID, HSTRING};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eCapture, DEVICE_STATE_ACTIVE};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::System::Registry::{RegOpenKeyExW, RegQueryValueExW, HKEY, KEY_READ, HKEY_LOCAL_MACHINE, REG_VALUE_TYPE};

const CLSID_MMDEVICE_ENUMERATOR: GUID = GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);

#[derive(Debug)]
pub struct AudioError(String);

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Audio error: {}", self.0)
    }
}

impl std::error::Error for AudioError {}

impl From<windows::core::Error> for AudioError {
    fn from(e: windows::core::Error) -> Self {
        AudioError(format!("COM error: {}", e))
    }
}

pub fn get_microphones() -> Result<Vec<crate::MicrophoneInfo>, AudioError> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;

        let enumerator: windows::Win32::Media::Audio::IMMDeviceEnumerator = CoCreateInstance(
            &CLSID_MMDEVICE_ENUMERATOR,
            None,
            CLSCTX_ALL,
        )?;

        let device_collection = enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)?;
        let count = device_collection.GetCount()?;

        let mut microphones = Vec::new();

        for i in 0..count {
            let device = device_collection.Item(i)?;

            // Получаем ID устройства
            let id_wstr = device.GetId()?;
            let id = id_wstr
                .to_string()
                .unwrap_or_else(|_| String::from("unknown"));
            let _ = CoTaskMemFree(Some(id_wstr.as_ptr().cast()));

            // Получаем имя устройства
            let name = get_device_name(&device)
                .unwrap_or_else(|_| format!("Устройство {}", i + 1));

            microphones.push(crate::MicrophoneInfo {
                device_id: id,
                name,
            });
        }

        CoUninitialize();
        Ok(microphones)
    }
}

fn get_device_name(device: &windows::Win32::Media::Audio::IMMDevice) -> Result<String, AudioError> {
    unsafe {
        // Получаем ID устройства для использования в реестре
        let id_wstr = device.GetId()?;
        let id = id_wstr.to_string().unwrap_or_else(|_| String::from("unknown"));
        let _ = CoTaskMemFree(Some(id_wstr.as_ptr().cast()));

        // Пробуем получить FriendlyName из реестра
        let reg_path = format!(r"SYSTEM\CurrentControlSet\Enum\SWD\MMDEVAPI\{}", id);
        let wide_path = HSTRING::from(&reg_path);
        let mut hkey: HKEY = HKEY::default();

        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, &wide_path, None, KEY_READ, &mut hkey).is_ok() && !hkey.is_invalid() {
            // Пробуем получить "FriendlyName"
            if let Ok(name) = read_registry_string(hkey, "FriendlyName") {
                return Ok(name);
            }
        }

        // Если реестр не дал результата, возвращаем дефолтное имя
        Ok(String::from("Unknown Microphone"))
    }
}

unsafe fn read_registry_string(hkey: HKEY, value_name: &str) -> Result<String, AudioError> {
    let wide_name = HSTRING::from(value_name);
    let mut data_type = REG_VALUE_TYPE(0);
    let mut data_size: u32 = 512;
    let mut data: Vec<u8> = vec![0; 512];

    let result = RegQueryValueExW(
        hkey,
        &wide_name,
        None,
        Some(&mut data_type),
        Some(data.as_mut_ptr()),
        Some(&mut data_size),
    );

    if result.is_ok() && (data_type == REG_VALUE_TYPE(1) || data_type == REG_VALUE_TYPE(2)) {
        let wide_data: Vec<u16> = data[..data_size as usize]
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .take_while(|&c| c != 0)
            .collect();

        String::from_utf16(&wide_data)
            .map_err(|_| AudioError("Invalid UTF-16".into()))
    } else {
        Err(AudioError(format!("Registry error: {:?}", result)))
    }
}

pub fn set_microphone_volume(device_id: &str, level: u32) -> Result<(), AudioError> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;

        let enumerator: windows::Win32::Media::Audio::IMMDeviceEnumerator = CoCreateInstance(
            &CLSID_MMDEVICE_ENUMERATOR,
            None,
            CLSCTX_ALL,
        )?;

        let device = enumerator.GetDevice(&HSTRING::from(device_id))?;

        // Активируем устройство и получаем интерфейс громкости
        let audio_endpoint_volume: IAudioEndpointVolume =
            device.Activate(CLSCTX_ALL, None)?;

        // Устанавливаем громкость (0.0 - 1.0)
        let normalized_level = (level as f32) / 100.0;
        audio_endpoint_volume.SetMasterVolumeLevelScalar(normalized_level, &GUID::zeroed())?;

        CoUninitialize();
        Ok(())
    }
}
