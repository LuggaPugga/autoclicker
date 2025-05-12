import { invoke } from "@tauri-apps/api/core"
import { create } from "zustand"
import { createTauriStore } from "@tauri-store/zustand"

interface AutoclickerState {
  isRunning: boolean
  clickSpeed: number
  randomize: boolean
  hotkeyLeft: string
  hotkeyRight: string
  hotkeyLeftActive: boolean
  hotkeyRightActive: boolean
  toggleIsRunning: () => void
  setClickSpeed: (speed: number) => void
  toggleRandomize: () => void
  setHotkeyLeft: (hotkey: string) => void
  setHotkeyRight: (hotkey: string) => void
  [key: string]: any
}

export const createAutoclickerStore = () =>
  create<AutoclickerState>((set, _get) => {
    return {
      isRunning: false,
      clickSpeed: 100.0,
      randomize: false,
      hotkeyLeft: "F5",
      hotkeyRight: "F6",
      hotkeyLeftActive: false,
      hotkeyRightActive: false,
      toggleIsRunning: () =>
        set((state) => {
          invoke("toggle_is_running")
          return {
            isRunning: !state.isRunning,
          }
        }),
      toggleRandomize: () => set((state) => ({ randomize: !state.randomize })),
      setClickSpeed: (speed: number) => {
        invoke("set_click_speed", { speed: speed })
        set({
          clickSpeed: speed,
        })
      },
      setHotkeyLeft: (hotkey: string) => {
        invoke("set_hotkey_left", { hotkey: hotkey })
        set({
          hotkeyLeft: hotkey,
        })
      },
      setHotkeyRight: (hotkey: string) => {
        invoke("set_hotkey_right", { hotkey: hotkey })
        set({
          hotkeyRight: hotkey,
        })
      },
    }
  })

export const useAutoclickerStore = createAutoclickerStore()
export const tauriHandler = createTauriStore("autoclicker", useAutoclickerStore, {
  syncStrategy: "immediate",
})
await tauriHandler.start()
