import { MousePointer } from "lucide-solid"
import { createEffect, createSignal, onCleanup } from "solid-js"
import { useAutoclickerStore } from "@/lib/autoclicker-store"
import { useTempStore } from "@/lib/temp-store"
import { Button } from "./ui/button"
import { Label } from "./ui/label"

type HotkeyType = "left" | "right"

interface HotkeyControlProps {
  class?: string
  isListening?: boolean
}

interface HotkeyButtonProps {
  type: HotkeyType
  hotkey: string
  isRecording: boolean
  isActive: boolean
  isRegistered: boolean
  isListening: boolean
  recordingText: string
  onRecordClick: (type: HotkeyType) => void
}

function HotkeyButton(props: HotkeyButtonProps) {
  const getButtonStyle = (): string => {
    const baseClasses = "h-7 min-w-[80px] text-xs transition-colors"

    if (props.isRecording) {
      return `${baseClasses} bg-cyan-600 hover:bg-cyan-700`
    } else if (!props.isRegistered && props.hotkey && props.isListening) {
      return `${baseClasses} bg-red-200 text-red-800 border-red-300`
    } else if (props.isActive) {
      return `${baseClasses} bg-green-600 hover:bg-green-700 text-white`
    }

    return baseClasses
  }

  return (
    <Button
      variant={props.isRecording ? "default" : props.isActive ? "default" : "outline"}
      size="sm"
      onClick={() => props.onRecordClick(props.type)}
      class={getButtonStyle()}
    >
      {props.isRecording ? props.recordingText : props.hotkey}
    </Button>
  )
}

export function HotkeyControl(props: HotkeyControlProps) {
  const autoclickerStore = useAutoclickerStore()
  const tempStore = useTempStore()

  const [recording, setRecording] = createSignal<HotkeyType | null>(null)
  const [currentModifiers, setCurrentModifiers] = createSignal<string[]>([])
  const [currentKey, setCurrentKey] = createSignal<string>("")

  const handleStartRecording = (type: HotkeyType): void => {
    setRecording(type)
    setCurrentModifiers([])
    setCurrentKey("")
  }

  const handleInputDown = (e: KeyboardEvent | MouseEvent): void => {
    if (!recording()) return
    e.preventDefault()

    if (e instanceof KeyboardEvent) {
      if (e.key === "Escape") {
        setRecording(null)
        setCurrentModifiers([])
        setCurrentKey("")
        return
      }

      if (["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
        setCurrentModifiers((prev) => {
          const modifier = e.key === "Control" ? "Ctrl" : e.key
          return prev.includes(modifier) ? prev : [...prev, modifier]
        })
        return
      }

      setCurrentKey(e.key === " " ? "Space" : e.key)
    } else if (e instanceof MouseEvent) {
      if (e.button > 2) {
        setCurrentKey(`MouseButton${e.button + 1}`)
      }
    }
  }

  const handleInputUp = (e: KeyboardEvent | MouseEvent): void => {
    if (!recording()) return

    if (e instanceof KeyboardEvent) {
      if (!["Control", "Alt", "Shift", "Meta", "Escape", " "].includes(e.key) && currentKey()) {
        const fullKey = [...currentModifiers(), currentKey()].join("+")

        if (recording() === "left") {
          autoclickerStore.setHotkeyLeft(fullKey)
        } else if (recording() === "right") {
          autoclickerStore.setHotkeyRight(fullKey)
        }

        setRecording(null)
        setCurrentModifiers([])
        setCurrentKey("")
      }

      if (["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
        setCurrentModifiers((prev) =>
          prev.filter((m) => m !== (e.key === "Control" ? "Ctrl" : e.key)),
        )
      }
    } else if (e instanceof MouseEvent) {
      e.preventDefault()
      if (currentKey() && e.button > 2) {
        const fullKey = currentKey()

        if (recording() === "left") {
          autoclickerStore.setHotkeyLeft(fullKey)
        } else if (recording() === "right") {
          autoclickerStore.setHotkeyRight(fullKey)
        }

        setRecording(null)
        setCurrentModifiers([])
        setCurrentKey("")
      }
    }
  }

  createEffect(() => {
    const handleContextMenu = (e: Event) => e.preventDefault()

    if (recording()) {
      window.addEventListener("keydown", handleInputDown as EventListener)
      window.addEventListener("keyup", handleInputUp as EventListener)
      window.addEventListener("mousedown", handleInputDown as EventListener)
      window.addEventListener("mouseup", handleInputUp as EventListener)
      window.addEventListener("contextmenu", handleContextMenu)

      onCleanup(() => {
        window.removeEventListener("keydown", handleInputDown as EventListener)
        window.removeEventListener("keyup", handleInputUp as EventListener)
        window.removeEventListener("mousedown", handleInputDown as EventListener)
        window.removeEventListener("mouseup", handleInputUp as EventListener)
        window.removeEventListener("contextmenu", handleContextMenu)
      })
    }
  })

  const getRecordingText = (): string => {
    if (!recording()) return ""

    const key = currentKey()
    if (key.startsWith("MouseButton")) {
      if (key === "MouseButton4") return "Browser Back"
      if (key === "MouseButton5") return "Browser Forward"

      const buttonNumber = key.replace("MouseButton", "")
      return `Mouse Button ${buttonNumber}`
    }

    if (currentModifiers().length === 0 && !key) {
      return "Press key..."
    }

    const parts = [...currentModifiers()]
    if (key) parts.push(key)
    return parts.join("+")
  }

  return (
    <div
      class={`w-full max-w-md rounded-md border border-border/30 bg-transparent p-3 space-y-3 ${props.class || ""}`}
    >
      <div class="space-y-2">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-2">
            <MousePointer class="h-4 w-4 text-muted-foreground" />
            <Label for="hotkeys" class="text-sm font-medium">
              Hotkeys (Global)
            </Label>
          </div>
        </div>

        <div class="flex items-center justify-between min-h-[28px]">
          <div class="flex items-center gap-2">
            <div
              class={`w-2 h-2 rounded-full transition-colors ${
                tempStore.hotkeyLeftActive ? "bg-green-500" : "bg-gray-300"
              }`}
            />
            <Label class="text-xs font-medium text-muted-foreground">Left click</Label>
          </div>

          <HotkeyButton
            type="left"
            hotkey={autoclickerStore.hotkeyLeft}
            isRecording={recording() === "left"}
            isActive={tempStore.hotkeyLeftActive}
            isRegistered={!!autoclickerStore.hotkeyLeft}
            isListening={props.isListening ?? false}
            recordingText={getRecordingText()}
            onRecordClick={handleStartRecording}
          />
        </div>

        <div class="flex items-center justify-between min-h-[28px]">
          <div class="flex items-center gap-2">
            <div
              class={`w-2 h-2 rounded-full transition-colors ${
                tempStore.hotkeyRightActive ? "bg-green-500" : "bg-gray-300"
              }`}
            />
            <Label class="text-xs font-medium text-muted-foreground">Right click</Label>
          </div>

          <HotkeyButton
            type="right"
            hotkey={autoclickerStore.hotkeyRight}
            isRecording={recording() === "right"}
            isActive={tempStore.hotkeyRightActive}
            isRegistered={!!autoclickerStore.hotkeyRight}
            isListening={props.isListening ?? false}
            recordingText={getRecordingText()}
            onRecordClick={handleStartRecording}
          />
        </div>
      </div>

      <p class="text-xs text-muted-foreground">
        Press a key. ESC to cancel. Hotkeys work globally.
      </p>
    </div>
  )
}
