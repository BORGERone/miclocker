import React from 'react'
import ReactDOM from 'react-dom/client'
import { invoke } from '@tauri-apps/api/core'
import App from './App'
import './index.css'
import './i18n'

// Отключаем контекстное меню браузера
document.addEventListener('contextmenu', (e) => e.preventDefault())

// Сворачиваем окно при потере фокуса (клик вне окна)
window.addEventListener('blur', () => {
  // Небольшая задержка чтобы окно успело потерять фокус
  setTimeout(() => {
    invoke('hide_window').catch(() => { })
  }, 100)
})

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
