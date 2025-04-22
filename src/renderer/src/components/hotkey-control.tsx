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

const DEFAULT_HOTKEYS = {
  left: 'F6',
  right: 'F7',
}

export function HotkeyControl({
  className = '',
  onHotkeyChange,
  isListening = false,
}: HotkeyControlProps): React.ReactElement {
  const [leftHotkey, setLeftHotkey] = useState<string>(() => {
    const savedHotkey = localStorage.getItem(STORAGE_KEYS.LEFT_HOTKEY)
    return savedHotkey || DEFAULT_HOTKEYS.left
  })

  const [rightHotkey, setRightHotkey] = useState<string>(() => {
    const savedHotkey = localStorage.getItem(STORAGE_KEYS.RIGHT_HOTKEY)
    return savedHotkey || DEFAULT_HOTKEYS.right
  })

  const [recording, setRecording] = useState<HotkeyType | null>(null)
  const [registrationStatus, setRegistrationStatus] = useState<Record<HotkeyType, boolean>>({
    left: false,
    right: false,
  })

  const [activeHotkeys, setActiveHotkeys] = useState<Record<HotkeyType, boolean>>({
    left: false,
    right: false,
  })

  useEffect(() => {
    const removeListener = window.api.onHotkeyActivated(
      (state: { leftActive: boolean; rightActive: boolean }) => {
        setActiveHotkeys({
          left: state.leftActive,
          right: state.rightActive,
        })
      }
    )

    return () => {
      removeListener()
    }
  }, [])

  const registerGlobalHotkey = useCallback(
    async (type: HotkeyType, hotkey: string): Promise<void> => {
      try {
        const success = await window.api.registerHotkey(type, hotkey)
        setRegistrationStatus(prev => ({ ...prev, [type]: success }))
        localStorage.setItem(
          type === 'left' ? STORAGE_KEYS.LEFT_HOTKEY : STORAGE_KEYS.RIGHT_HOTKEY,
          hotkey
        )

        if (onHotkeyChange) {
          onHotkeyChange(type, hotkey)
        }
      } catch (error) {
        console.error(`Failed to register ${type} hotkey:`, error)
        setRegistrationStatus(prev => ({ ...prev, [type]: false }))
      }
    },
    [onHotkeyChange]
  )

  useEffect(() => {
    if (leftHotkey) {
      registerGlobalHotkey('left', leftHotkey)
    }
  }, [leftHotkey, registerGlobalHotkey])

  useEffect(() => {
    if (rightHotkey) {
      registerGlobalHotkey('right', rightHotkey)
    }
  }, [rightHotkey, registerGlobalHotkey])

  const handleStartRecording = useCallback((type: HotkeyType): void => {
    setRecording(type)
  }, [])

  const handleKeyDown = useCallback(
    (e: KeyboardEvent): void => {
      if (!recording) return

      e.preventDefault()

      let keyName = e.key

      if (e.key === ' ') keyName = 'Space'
      if (e.key === 'Control') keyName = 'Ctrl'
      if (e.key === 'Meta') keyName = 'Meta'
      if (e.key === 'Escape') {
        setRecording(null)
        return
      }

      const modifiers: string[] = []
      if (e.ctrlKey && e.key !== 'Control') modifiers.push('Ctrl')
      if (e.altKey && e.key !== 'Alt') modifiers.push('Alt')
      if (e.shiftKey && e.key !== 'Shift') modifiers.push('Shift')
      if (e.metaKey && e.key !== 'Meta') modifiers.push('Meta')

      const fullKey = [...modifiers, keyName].join('+')

      if (recording === 'left') {
        setLeftHotkey(fullKey)
      } else {
        setRightHotkey(fullKey)
      }

      setRecording(null)
    },
    [recording]
  )

  const handleMouseDown = useCallback(
    (e: MouseEvent): void => {
      if (!recording) return

      e.preventDefault()

      if (e.button >= 0 && e.button <= 4) {
        const buttonNames = ['Mouse1', 'Mouse3', 'Mouse2', 'Mouse4', 'Mouse5']
        const buttonName = buttonNames[e.button]

        const modifiers: string[] = []
        if (e.ctrlKey) modifiers.push('Ctrl')
        if (e.altKey) modifiers.push('Alt')
        if (e.shiftKey) modifiers.push('Shift')
        if (e.metaKey) modifiers.push('Meta')

        const fullKey = [...modifiers, buttonName].join('+')

        if (recording === 'left') {
          setLeftHotkey(fullKey)
        } else {
          setRightHotkey(fullKey)
        }

        setRecording(null)
      }
    },
    [recording]
  )

  useEffect(() => {
    if (recording) {
      window.addEventListener('keydown', handleKeyDown)
      window.addEventListener('mousedown', handleMouseDown)
    }

    return (): void => {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('mousedown', handleMouseDown)
    }
  }, [recording, handleKeyDown, handleMouseDown])

  const getButtonStyle = (type: HotkeyType): string => {
    const isRecording = recording === type
    const isActive = activeHotkeys[type]
    const isRegistered = registrationStatus[type]
    const hotkey = type === 'left' ? leftHotkey : rightHotkey

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

          <div className="flex items-center gap-2">
            <Button
              variant={
                recording === 'left' ? 'default' : activeHotkeys.left ? 'default' : 'outline'
              }
              size="sm"
              onClick={() => handleStartRecording('left')}
              className={getButtonStyle('left')}
            >
              {recording === 'left' ? 'Press key...' : leftHotkey}
            </Button>
          </div>
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

          <div className="flex items-center gap-2">
            <Button
              variant={
                recording === 'right' ? 'default' : activeHotkeys.right ? 'default' : 'outline'
              }
              size="sm"
              onClick={() => handleStartRecording('right')}
              className={getButtonStyle('right')}
            >
              {recording === 'right' ? 'Press key...' : rightHotkey}
            </Button>
          </div>
        </div>
      </div>

      <p className="text-xs text-muted-foreground">
        Press a key or mouse button. ESC to cancel. Hotkeys work globally.
      </p>
    </div>
  )
}
