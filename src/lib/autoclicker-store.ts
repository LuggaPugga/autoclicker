import { createSignal, onCleanup } from "solid-js"
import { Store } from "tauri-store"

interface AutoclickerState {
  clickSpeed: number
  holdMode: boolean
  hotkeyLeft: string
  hotkeyRight: string
  [key: string]: unknown
}

const store = new Store<AutoclickerState>("autoclicker", {
  clickSpeed: 100.0,
  holdMode: false,
  hotkeyLeft: "F5",
  hotkeyRight: "F6",
})

const initialState: AutoclickerState = {
  clickSpeed: 100.0,
  holdMode: false,
  hotkeyLeft: "F5",
  hotkeyRight: "F6",
}

const [state, setState] = createSignal<AutoclickerState>(initialState)

store.start().then(() => {
  const currentState = {
    clickSpeed: (store.get("clickSpeed") as number) ?? 100.0,
    holdMode: (store.get("holdMode") as boolean) ?? false,
    hotkeyLeft: (store.get("hotkeyLeft") as string) ?? "F5",
    hotkeyRight: (store.get("hotkeyRight") as string) ?? "F6",
  }
  setState(currentState)
})

const unsubscribe = store.subscribe(() => {
  const currentState = {
    clickSpeed: (store.get("clickSpeed") as number) ?? 100.0,
    holdMode: (store.get("holdMode") as boolean) ?? false,
    hotkeyLeft: (store.get("hotkeyLeft") as string) ?? "F5",
    hotkeyRight: (store.get("hotkeyRight") as string) ?? "F6",
  }
  setState(currentState)
})

onCleanup(() => {
  unsubscribe()
})

export function useAutoclickerStore() {
  const currentState = state

  return {
    get clickSpeed() {
      return currentState().clickSpeed
    },
    get holdMode() {
      return currentState().holdMode
    },
    get hotkeyLeft() {
      return currentState().hotkeyLeft
    },
    get hotkeyRight() {
      return currentState().hotkeyRight
    },
    setClickSpeed: (speed: number) => {
      store.set("clickSpeed", speed)
      store.save()
    },
    toggleHoldMode: () => {
      const current = store.get("holdMode")
      store.set("holdMode", !current)
      store.save()
    },
    setHotkeyLeft: (hotkey: string) => {
      store.set("hotkeyLeft", hotkey)
      store.save()
    },
    setHotkeyRight: (hotkey: string) => {
      store.set("hotkeyRight", hotkey)
      store.save()
    },
  }
}
