import { contextBridge, ipcRenderer } from 'electron'
import { electronAPI } from '@electron-toolkit/preload'
const api = {
  toggleAutoClicker: (isRunning: boolean): Promise<void> =>
    ipcRenderer.invoke('toggle-auto-clicker', isRunning),
  setClickInterval: (interval: number): Promise<void> =>
    ipcRenderer.invoke('set-click-interval', interval),
  setRandomize: (randomize: boolean): Promise<void> =>
    ipcRenderer.invoke('set-randomize', randomize),
  registerHotkey: (type: 'left' | 'right', hotkey: string): Promise<boolean> =>
    ipcRenderer.invoke('register-hotkey', type, hotkey),
  onHotkeyActivated: (
    callback: (state: { leftActive: boolean; rightActive: boolean }) => void,
  ): (() => void) => {
    const listener = (_: unknown, state: { leftActive: boolean; rightActive: boolean }): void =>
      callback(state)
    ipcRenderer.on('hotkey-state-update', listener)
    return () => ipcRenderer.removeListener('hotkey-state-update', listener)
  },
  testHotkeyActivation: (type: 'left' | 'right'): Promise<boolean> =>
    ipcRenderer.invoke('test-hotkey-activation', type),
}

if (process.contextIsolated) {
  try {
    console.log('context isolated')
    contextBridge.exposeInMainWorld('electron', electronAPI)
    contextBridge.exposeInMainWorld('api', api)
  } catch (error) {
    console.error(error)
  }
} else {
  console.log('not context isolated')
  // @ts-ignore (define in dts)
  window.electron = electronAPI
  // @ts-ignore (define in dts)
  window.api = api
}
