import { create } from "zustand"
import { createTauriStore } from "@tauri-store/zustand"

interface AutoclickerState {
  clickSpeed: number
  holdMode: boolean
  hotkeyLeft: string
  hotkeyRight: string
  setClickSpeed: (speed: number) => void
  toggleHoldMode: () => void
  setHotkeyLeft: (hotkey: string) => void
  setHotkeyRight: (hotkey: string) => void
  [key: string]: any
}

export const createAutoclickerStore = () =>
  create<AutoclickerState>((set, _get) => {
    return {
      clickSpeed: 100.0,
      holdMode: false,
      hotkeyLeft: "F5",
      hotkeyRight: "F6",
      toggleHoldMode: () => set((state) => ({ holdMode: !state.holdMode })),
      setClickSpeed: (speed: number) => set({ clickSpeed: speed }),
      setHotkeyLeft: (hotkey: string) => set({ hotkeyLeft: hotkey }),
      setHotkeyRight: (hotkey: string) => set({ hotkeyRight: hotkey }),
    }
  })

export const useAutoclickerStore = createAutoclickerStore()
export const tauriHandler = createTauriStore("autoclicker", useAutoclickerStore, {
  syncStrategy: "immediate",
  saveStrategy: "debounce",
})
;(async () => {
  await tauriHandler.start()
})()
