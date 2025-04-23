import React, { useState, useEffect, useCallback } from 'react'
import { Button } from './ui/button'
import { Label } from './ui/label'
import { MousePointer } from 'lucide-react'

type HotkeyType = 'left' | 'right'

interface HotkeyControlProps {
  className?: string
  onHotkeyChange?: (type: HotkeyType, hotkey: string) => void
  isListening?: boolean
}

const STORAGE_KEYS = {
  LEFT_HOTKEY: 'autoclicker-leftHotkey',
  RIGHT_HOTKEY: 'autoclicker-rightHotkey',
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
    const baseClasses = 'h-7 min-w-[80px] text-xs transition-colors'

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
      variant={isRecording ? 'default' : isActive ? 'default' : 'outline'}
      size="sm"
      onClick={() => onRecordClick(type)}
      className={getButtonStyle()}
    >
      {isRecording ? recordingText : hotkey}
    </Button>
  )
}

export function HotkeyControl({
  className = '',
  onHotkeyChange,
  isListening = false,
}: HotkeyControlProps): React.ReactElement {
  const [hotkeys, setHotkeys] = useState({
    left: localStorage.getItem(STORAGE_KEYS.LEFT_HOTKEY) || 'F6',
    right: localStorage.getItem(STORAGE_KEYS.RIGHT_HOTKEY) || 'F7',
  })

  const [recording, setRecording] = useState<HotkeyType | null>(null)
  const [registrationStatus, setRegistrationStatus] = useState<Record<HotkeyType, boolean>>({
    left: false,
    right: false,
  })
  const [currentModifiers, setCurrentModifiers] = useState<string[]>([])
  const [currentKey, setCurrentKey] = useState<string>('')
  const [activeHotkeys, setActiveHotkeys] = useState<Record<HotkeyType, boolean>>({
    left: false,
    right: false,
  })

  const registerGlobalHotkey = useCallback(
    async (type: HotkeyType, hotkey: string): Promise<void> => {
      try {
        const success = await window.api.registerHotkey(type, hotkey)
        setRegistrationStatus((prev) => ({ ...prev, [type]: success }))
        localStorage.setItem(
          type === 'left' ? STORAGE_KEYS.LEFT_HOTKEY : STORAGE_KEYS.RIGHT_HOTKEY,
          hotkey,
        )

        if (onHotkeyChange) {
          onHotkeyChange(type, hotkey)
        }
      } catch (error) {
        console.error(`Failed to register ${type} hotkey:`, error)
        setRegistrationStatus((prev) => ({ ...prev, [type]: false }))
      }
    },
    [onHotkeyChange],
  )

  useEffect(() => {
    const removeListener = window.api.onHotkeyActivated(
      (state: { leftActive: boolean; rightActive: boolean }) => {
        setActiveHotkeys({
          left: state.leftActive,
          right: state.rightActive,
        })
      },
    )

    return () => removeListener()
  }, [])

  useEffect(() => {
    registerGlobalHotkey('left', hotkeys.left)
  }, [hotkeys.left, registerGlobalHotkey])

  useEffect(() => {
    registerGlobalHotkey('right', hotkeys.right)
  }, [hotkeys.right, registerGlobalHotkey])

  const handleStartRecording = useCallback((type: HotkeyType): void => {
    setRecording(type)
    setCurrentModifiers([])
    setCurrentKey('')
  }, [])

  const handleKeyDown = useCallback(
    (e: KeyboardEvent): void => {
      if (!recording) return
      e.preventDefault()

      if (e.key === 'Escape') {
        setRecording(null)
        setCurrentModifiers([])
        setCurrentKey('')
        return
      }

      if (['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) {
        setCurrentModifiers((prev) => {
          const modifier = e.key === 'Control' ? 'Ctrl' : e.key
          return prev.includes(modifier) ? prev : [...prev, modifier]
        })
        return
      }

      setCurrentKey(e.key === ' ' ? 'Space' : e.key)
    },
    [recording],
  )

  const handleKeyUp = useCallback(
    (e: KeyboardEvent): void => {
      if (!recording) return

      if (!['Control', 'Alt', 'Shift', 'Meta', 'Escape', ' '].includes(e.key) && currentKey) {
        const fullKey = [...currentModifiers, currentKey].join('+')

        setHotkeys((prev) => ({
          ...prev,
          [recording]: fullKey,
        }))

        setRecording(null)
        setCurrentModifiers([])
        setCurrentKey('')
      }

      if (['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) {
        setCurrentModifiers((prev) =>
          prev.filter((m) => m !== (e.key === 'Control' ? 'Ctrl' : e.key)),
        )
      }
    },
    [recording, currentModifiers, currentKey],
  )

  useEffect(() => {
    if (recording) {
      window.addEventListener('keydown', handleKeyDown)
      window.addEventListener('keyup', handleKeyUp)
    }

    return () => {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('keyup', handleKeyUp)
    }
  }, [recording, handleKeyDown, handleKeyUp])

  const getRecordingText = (): string => {
    if (!recording) return ''

    if (currentModifiers.length === 0 && !currentKey) {
      return 'Press key...'
    }

    const parts = [...currentModifiers]
    if (currentKey) parts.push(currentKey)
    return parts.join('+')
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
                activeHotkeys.left ? 'bg-green-500' : 'bg-gray-300'
              }`}
            />
            <Label className="text-xs font-medium text-muted-foreground">Left click</Label>
          </div>

          <HotkeyButton
            type="left"
            hotkey={hotkeys.left}
            isRecording={recording === 'left'}
            isActive={activeHotkeys.left}
            isRegistered={registrationStatus.left}
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
                activeHotkeys.right ? 'bg-green-500' : 'bg-gray-300'
              }`}
            />
            <Label className="text-xs font-medium text-muted-foreground">Right click</Label>
          </div>

          <HotkeyButton
            type="right"
            hotkey={hotkeys.right}
            isRecording={recording === 'right'}
            isActive={activeHotkeys.right}
            isRegistered={registrationStatus.right}
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
