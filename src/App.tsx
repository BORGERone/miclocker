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

        <div className="footer">
          <span className="version">v1.0.0</span>
          <a href="https://github.com/BORGERone/miclocker" target="_blank" rel="noopener noreferrer" className="github-link">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
            GitHub
          </a>
        </div>
      </div>
    </div>
  )
}

export default App
