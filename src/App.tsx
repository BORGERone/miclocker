import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

interface Microphone {
  device_id: string
  name: string
}

interface AppSettings {
  selected_device: string | null
  volume: number
  auto_start: boolean
}

function App() {
  const [microphones, setMicrophones] = useState<Microphone[]>([])
  const [selectedMic, setSelectedMic] = useState<string>('')
  const [volume, setVolume] = useState<number>(100)
  const [isMaintaining, setIsMaintaining] = useState<boolean>(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string>('')
  const [settingsLoaded, setSettingsLoaded] = useState<boolean>(false)
  const [autoStart, setAutoStart] = useState<boolean>(false)

  useEffect(() => {
    loadMicrophones()
    loadSettings()
  }, [])

  async function loadSettings() {
    try {
      const settings = await invoke<AppSettings>('load_settings')
      setVolume(settings.volume)
      if (settings.selected_device) {
        setSelectedMic(settings.selected_device)
      }
      setAutoStart(settings.auto_start)
      setSettingsLoaded(true)

      if (settings.auto_start && settings.selected_device) {
        await new Promise(resolve => setTimeout(resolve, 300))
        const maintaining = await invoke<boolean>('is_maintaining')
        if (maintaining) {
          setIsMaintaining(true)
        }
      }
    } catch (e) {
      setSettingsLoaded(true)
    }
  }

  async function loadMicrophones() {
    try {
      const mics = await invoke<Microphone[]>('get_microphones')
      setMicrophones(mics)
      if (mics.length > 0 && !selectedMic && settingsLoaded) {
        setSelectedMic(mics[0].device_id)
      }
    } catch (e) {
      setError(`Ошибка загрузки микрофонов: ${e}`)
    }
  }

  async function handleStartMaintain() {
    if (!selectedMic) return
    try {
      setError(null)
      setStatus('Запуск...')
      await invoke('start_auto_maintain', { deviceId: selectedMic, level: volume })
      setIsMaintaining(true)
      setStatus('')
    } catch (e) {
      setError(`Ошибка запуска: ${e}`)
      setStatus('')
    }
  }

  async function handleStopMaintain() {
    try {
      setError(null)
      setStatus('Остановка...')
      await invoke('stop_auto_maintain')
      setIsMaintaining(false)
      setStatus('')
    } catch (e) {
      setError(`Ошибка остановки: ${e}`)
      setStatus('')
    }
  }

  async function handleVolumeChange(e: React.ChangeEvent<HTMLInputElement>) {
    const newVolume = parseInt(e.target.value)
    setVolume(newVolume)
    if (isMaintaining && selectedMic) {
      try {
        await invoke('set_microphone_volume', { deviceId: selectedMic, level: newVolume })
      } catch (e) {
        setError(`Ошибка установки громкости: ${e}`)
      }
    }
  }

  async function handleMicChange(e: React.ChangeEvent<HTMLSelectElement>) {
    const newMic = e.target.value
    setSelectedMic(newMic)
    try {
      await invoke('select_microphone', { deviceId: newMic })
    } catch (e) {
      setError(`Ошибка сохранения микрофона: ${e}`)
    }
    if (isMaintaining) {
      try {
        await invoke('set_microphone_volume', { deviceId: newMic, level: volume })
      } catch (e) {
        setError(`Ошибка переключения микрофона: ${e}`)
      }
    }
  }

  async function handleAutoStartChange(e: React.ChangeEvent<HTMLInputElement>) {
    const enabled = e.target.checked
    setAutoStart(enabled)
    try {
      await invoke('set_auto_start', { enabled })
    } catch (e) {
      setError(`Ошибка сохранения настройки автозапуска: ${e}`)
    }
  }

  return (
    <div className="app">
      {/* Drag region — вся верхняя полоска */}
      <div className="drag-region" data-tauri-drag-region />

      <div className="content">
        <div className="form-group">
          <div className="label-row">
            <label htmlFor="microphone">Микрофон</label>
            <div className="status-bar">
              <div className={`status-dot ${isMaintaining ? 'active' : ''}`}></div>
              <span>{isMaintaining ? 'Активно' : 'Неактивно'}</span>
            </div>
          </div>
          <select
            id="microphone"
            value={selectedMic}
            onChange={handleMicChange}
            disabled={isMaintaining}
          >
            {microphones.length === 0 && (
              <option value="">Нет доступных микрофонов</option>
            )}
            {microphones.map(mic => (
              <option key={mic.device_id} value={mic.device_id}>
                {mic.name}
              </option>
            ))}
          </select>
        </div>

        <div className="form-group">
          <div className="volume-header">
            <label htmlFor="volume">Громкость</label>
            <span className="volume-value">{volume}%</span>
          </div>
          <div className="slider-container">
            <input
              type="range"
              id="volume"
              min="0"
              max="100"
              value={volume}
              onChange={handleVolumeChange}
              disabled={isMaintaining}
            />
          </div>
        </div>

        {error && (
          <div className="error">
            {error}
          </div>
        )}

        {status && <div className="status-message">{status}</div>}

        <button
          className={`btn ${isMaintaining ? 'btn-stop' : 'btn-start'}`}
          onClick={isMaintaining ? handleStopMaintain : handleStartMaintain}
        >
          {isMaintaining ? 'Остановить' : 'Запустить'}
        </button>

        <div className="form-group">
          <label className="toggle-label">
            <span className="toggle-text">
              Запускать при старте
            </span>
            <label className="switch">
              <input
                type="checkbox"
                checked={autoStart}
                onChange={handleAutoStartChange}
              />
              <span className="slider"></span>
            </label>
          </label>
        </div>
      </div>
    </div>
  )
}

export default App
