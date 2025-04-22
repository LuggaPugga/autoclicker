import { ElectronAPI } from '@electron-toolkit/preload'

interface CustomElectronAPI {
  toggleAutoClicker: (state: boolean) => Promise<void>
  setClickInterval: (interval: number) => Promise<void>
  setRandomize: (state: boolean) => Promise<void>
  registerHotkey: (type: 'left' | 'right', hotkey: string) => Promise<boolean>
  onHotkeyActivated: (
    callback: (state: { leftActive: boolean; rightActive: boolean }) => void
  ) => () => void
  testHotkeyActivation: (type: 'left' | 'right') => Promise<boolean>
}

declare global {
  interface Window {
    electron: ElectronAPI
    api: CustomElectronAPI
  }
}
