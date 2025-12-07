import { createSignal, onCleanup } from "solid-js"
import { Store } from "tauri-store"

interface TempState {
  isRunning: boolean
  hotkeyLeftActive: boolean
  hotkeyRightActive: boolean
  [key: string]: unknown
}

const store = new Store<TempState>("temp", {
  isRunning: false,
  hotkeyLeftActive: false,
  hotkeyRightActive: false,
})

const initialState: TempState = {
  isRunning: false,
  hotkeyLeftActive: false,
  hotkeyRightActive: false,
}

const [state, setState] = createSignal<TempState>(initialState)

store.start().then(() => {
  const currentState = {
    isRunning: (store.get("isRunning") as boolean) ?? false,
    hotkeyLeftActive: (store.get("hotkeyLeftActive") as boolean) ?? false,
    hotkeyRightActive: (store.get("hotkeyRightActive") as boolean) ?? false,
  }
  setState(currentState)
})

const unsubscribe = store.subscribe(() => {
  const currentState = {
    isRunning: (store.get("isRunning") as boolean) ?? false,
    hotkeyLeftActive: (store.get("hotkeyLeftActive") as boolean) ?? false,
    hotkeyRightActive: (store.get("hotkeyRightActive") as boolean) ?? false,
  }
  setState(currentState)
})

onCleanup(() => {
  unsubscribe()
})

export function useTempStore() {
  const currentState = state

  return {
    get isRunning() {
      return currentState().isRunning
    },
    get hotkeyLeftActive() {
      return currentState().hotkeyLeftActive
    },
    get hotkeyRightActive() {
      return currentState().hotkeyRightActive
    },
    toggleIsRunning: () => {
      const current = store.get("isRunning")
      store.set("isRunning", !current)
    },
  }
}
