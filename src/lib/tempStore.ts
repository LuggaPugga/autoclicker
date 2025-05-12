import { create } from "zustand"
import { createTauriStore } from "@tauri-store/zustand"

interface TempStoreState {
  isRunning: boolean
  hotkeyLeftActive: boolean
  hotkeyRightActive: boolean
  toggleIsRunning: () => void
  [key: string]: any
}

export const createTempStore = () =>
  create<TempStoreState>((set) => ({
    isRunning: false,
    hotkeyLeftActive: false,
    hotkeyRightActive: false,
    toggleIsRunning: () => set((state) => ({ isRunning: !state.isRunning })),
  }))

export const useTempStore = createTempStore()
export const tauriHandler = createTauriStore("temp", useTempStore, {
  syncStrategy: "immediate",
  saveOnChange: false,
  saveOnExit: false,
})
;(async () => {
  await tauriHandler.start()
})()
