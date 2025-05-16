import React, { useState, useEffect, useCallback } from "react"
import { Button } from "./ui/button"
import { Label } from "./ui/label"
import { MousePointer } from "lucide-react"
import { useTempStore } from "@/lib/temp-store"
import { useAutoclickerStore } from "@/lib/autoclicker-store"

type HotkeyType = "left" | "right"

interface HotkeyControlProps {
  className?: string
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

function HotkeyButton({
  type,
  hotkey,
  isRecording,
  isActive,
  isRegistered,
  isListening,
  recordingText,
  onRecordClick,
}: HotkeyButtonProps) {
  const getButtonStyle = (): string => {
    const baseClasses = "h-7 min-w-[80px] text-xs transition-colors"

    if (isRecording) {
      return `${baseClasses} bg-cyan-600 hover:bg-cyan-700`
    } else if (!isRegistered && hotkey && isListening) {
      return `${baseClasses} bg-red-200 text-red-800 border-red-300`
    } else if (isActive) {
      return `${baseClasses} bg-green-600 hover:bg-green-700 text-white`
    }

    return baseClasses
  }

  return (
    <Button
      variant={isRecording ? "default" : isActive ? "default" : "outline"}
      size="sm"
      onClick={() => onRecordClick(type)}
      className={getButtonStyle()}
    >
      {isRecording ? recordingText : hotkey}
    </Button>
  )
}

export function HotkeyControl({
  className = "",
  isListening = false,
}: HotkeyControlProps): React.ReactElement {
  const { hotkeyLeft, hotkeyRight, setHotkeyLeft, setHotkeyRight } = useAutoclickerStore()
  const { hotkeyLeftActive, hotkeyRightActive } = useTempStore()

  useEffect(() => {
    console.log("hotkeyLeftActive", hotkeyLeftActive)
    console.log("hotkeyRightActive", hotkeyRightActive)
  }, [hotkeyLeftActive, hotkeyRightActive])

  const [recording, setRecording] = useState<HotkeyType | null>(null)
  const [currentModifiers, setCurrentModifiers] = useState<string[]>([])
  const [currentKey, setCurrentKey] = useState<string>("")

  const handleStartRecording = useCallback((type: HotkeyType): void => {
    setRecording(type)
    setCurrentModifiers([])
    setCurrentKey("")
  }, [])

  const handleInputDown = useCallback(
    (e: KeyboardEvent | MouseEvent): void => {
      if (!recording) return
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
    },
    [recording],
  )

  const handleInputUp = useCallback(
    (e: KeyboardEvent | MouseEvent): void => {
      if (!recording) return

      if (e instanceof KeyboardEvent) {
        if (!["Control", "Alt", "Shift", "Meta", "Escape", " "].includes(e.key) && currentKey) {
          const fullKey = [...currentModifiers, currentKey].join("+")

          if (recording === "left") {
            setHotkeyLeft(fullKey)
          } else if (recording === "right") {
            setHotkeyRight(fullKey)
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
        if (currentKey && e.button > 2) {
          const fullKey = currentKey

          if (recording === "left") {
            setHotkeyLeft(fullKey)
          } else if (recording === "right") {
            setHotkeyRight(fullKey)
          }

          setRecording(null)
          setCurrentModifiers([])
          setCurrentKey("")
        }
      }
    },
    [recording, currentModifiers, currentKey, setHotkeyLeft, setHotkeyRight],
  )

  useEffect(() => {
    if (recording) {
      window.addEventListener("keydown", handleInputDown as EventListener)
      window.addEventListener("keyup", handleInputUp as EventListener)
      window.addEventListener("mousedown", handleInputDown as EventListener)
      window.addEventListener("mouseup", handleInputUp as EventListener)
      window.addEventListener("contextmenu", (e) => e.preventDefault())
    }

    return () => {
      window.removeEventListener("keydown", handleInputDown as EventListener)
      window.removeEventListener("keyup", handleInputUp as EventListener)
      window.removeEventListener("mousedown", handleInputDown as EventListener)
      window.removeEventListener("mouseup", handleInputUp as EventListener)
      window.removeEventListener("contextmenu", (e) => e.preventDefault())
    }
  }, [recording, handleInputDown, handleInputUp])

  const getRecordingText = (): string => {
    if (!recording) return ""

    if (currentKey.startsWith("MouseButton")) {
      if (currentKey === "MouseButton4") return "Browser Back"
      if (currentKey === "MouseButton5") return "Browser Forward"

      const buttonNumber = currentKey.replace("MouseButton", "")
      return `Mouse Button ${buttonNumber}`
    }

    if (currentModifiers.length === 0 && !currentKey) {
      return "Press key..."
    }

    const parts = [...currentModifiers]
    if (currentKey) parts.push(currentKey)
    return parts.join("+")
  }

  return (
    <div
      className={`w-full max-w-md rounded-md border border-border/30 bg-transparent p-3 space-y-3 ${className}`}
    >
      <div className="space-y-2">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <MousePointer className="h-4 w-4 text-muted-foreground" />
            <Label htmlFor="hotkeys" className="text-sm font-medium">
              Hotkeys (Global)
            </Label>
          </div>
        </div>

        {/* Left Click Hotkey */}
        <div className="flex items-center justify-between min-h-[28px]">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full transition-colors ${
                hotkeyLeftActive ? "bg-green-500" : "bg-gray-300"
              }`}
            />
            <Label className="text-xs font-medium text-muted-foreground">Left click</Label>
          </div>

          <HotkeyButton
            type="left"
            hotkey={hotkeyLeft}
            isRecording={recording === "left"}
            isActive={hotkeyLeftActive}
            isRegistered={!!hotkeyLeft}
            isListening={isListening}
            recordingText={getRecordingText()}
            onRecordClick={handleStartRecording}
          />
        </div>

        {/* Right Click Hotkey */}
        <div className="flex items-center justify-between min-h-[28px]">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full transition-colors ${
                hotkeyRightActive ? "bg-green-500" : "bg-gray-300"
              }`}
            />
            <Label className="text-xs font-medium text-muted-foreground">Right click</Label>
          </div>

          <HotkeyButton
            type="right"
            hotkey={hotkeyRight}
            isRecording={recording === "right"}
            isActive={hotkeyRightActive}
            isRegistered={!!hotkeyRight}
            isListening={isListening}
            recordingText={getRecordingText()}
            onRecordClick={handleStartRecording}
          />
        </div>
      </div>

      <p className="text-xs text-muted-foreground">
        Press a key. ESC to cancel. Hotkeys work globally.
      </p>
    </div>
  )
}
