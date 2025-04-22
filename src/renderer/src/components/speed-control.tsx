import React, { useState, useEffect, useCallback } from 'react'
import { Slider } from './ui/slider'
import { Input } from './ui/input'
import { Label } from './ui/label'
import { Button } from './ui/button'
import { Switch } from './ui/switch'
import { Timer, Zap, Shuffle } from 'lucide-react'

type SpeedMode = 'cps' | 'ms'

const STORAGE_KEYS = {
  MODE: 'autoclicker-mode',
  VALUE: 'autoclicker-value',
  RANDOMIZE: 'autoclicker-randomize',
}

export function SpeedControl(): React.ReactElement {
  const [mode, setMode] = useState<SpeedMode>(() => {
    const savedMode = localStorage.getItem(STORAGE_KEYS.MODE)
    return (savedMode as SpeedMode) || 'cps'
  })

  const [value, setValue] = useState(() => {
    const savedValue = localStorage.getItem(STORAGE_KEYS.VALUE)
    if (savedValue) {
      return parseInt(savedValue)
    }
    return mode === 'cps' ? 5 : 200
  })

  const [randomize, setRandomize] = useState(() => {
    const savedRandomize = localStorage.getItem(STORAGE_KEYS.RANDOMIZE)
    return savedRandomize === 'true'
  })

  useEffect(() => {
    localStorage.setItem(STORAGE_KEYS.MODE, mode)
  }, [mode])

  useEffect(() => {
    localStorage.setItem(STORAGE_KEYS.VALUE, value.toString())
  }, [value])

  useEffect(() => {
    localStorage.setItem(STORAGE_KEYS.RANDOMIZE, randomize.toString())
  }, [randomize])

  useEffect(() => {
    const updateMainProcess = async (): Promise<void> => {
      const interval = mode === 'cps' ? Math.round(1000 / value) : value
      await window.api.setClickInterval(interval)
    }

    updateMainProcess()
  }, [value, mode])

  useEffect(() => {
    const updateRandomize = async (): Promise<void> => {
      await window.api.setRandomize(randomize)
    }

    updateRandomize()
  }, [randomize])

  const handleModeToggle = useCallback(
    (newMode: SpeedMode): void => {
      if (newMode === mode) return

      if (newMode === 'cps') {
        setValue(Math.max(1, Math.round(1000 / value)))
      } else {
        setValue(Math.round(1000 / value))
      }

      setMode(newMode)
    },
    [mode, value]
  )

  const handleSliderChange = useCallback((newValue: number[]): void => {
    setValue(newValue[0])
  }, [])

  const handleInputChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>): void => {
      const numValue = parseInt(e.target.value)
      if (isNaN(numValue)) return

      if (mode === 'cps') {
        setValue(Math.max(1, numValue))
      } else {
        setValue(Math.max(50, numValue))
      }
    },
    [mode]
  )

  const min = mode === 'cps' ? 1 : 50
  const max = mode === 'cps' ? 50 : 1000
  const step = mode === 'cps' ? 1 : 10

  return (
    <div className="w-full max-w-md space-y-4">
      <div className="p-4 rounded-lg border border-border/50 bg-card space-y-6">
        <div className="flex justify-center space-x-2">
          <Button
            type="button"
            variant={mode === 'cps' ? 'default' : 'outline'}
            className="flex-1"
            onClick={() => handleModeToggle('cps')}
          >
            <Zap className="h-4 w-4 mr-2" />
            Clicks per second
          </Button>
          <Button
            type="button"
            variant={mode === 'ms' ? 'default' : 'outline'}
            className="flex-1"
            onClick={() => handleModeToggle('ms')}
          >
            <Timer className="h-4 w-4 mr-2" />
            Milliseconds
          </Button>
        </div>

        <div className="space-y-4">
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <Label htmlFor="speed-input" className="text-sm font-medium">
                {mode === 'cps' ? 'Clicks per second' : 'Milliseconds between clicks'}
              </Label>
              <div className="bg-muted/30 rounded px-2 py-1 text-sm font-mono text-muted-foreground">
                {mode === 'cps'
                  ? `${value} CPS (${Math.round(1000 / value)}ms)`
                  : `${value}ms (${(1000 / value).toFixed(1)} CPS)`}
              </div>
            </div>

            <Slider
              value={[value]}
              min={min}
              max={max}
              step={step}
              onValueChange={handleSliderChange}
              className="my-6"
            />
          </div>

          <div className="flex items-center gap-2">
            <Label htmlFor="speed-input" className="sr-only">
              {mode === 'cps' ? 'Clicks per second' : 'Milliseconds between clicks'}
            </Label>
            <Input
              id="speed-input"
              type="number"
              value={value}
              onChange={handleInputChange}
              min={min}
              step={step}
              className="w-24"
            />
            <span className="text-sm text-muted-foreground">
              {mode === 'cps' ? 'clicks per second' : 'milliseconds'}
            </span>
          </div>
        </div>
      </div>

      <div className="p-3 rounded-md border border-border/30 bg-background/50">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Shuffle className="h-4 w-4 text-muted-foreground" />
            <Label htmlFor="randomize-switch" className="text-sm font-medium">
              Randomize timing
            </Label>
          </div>

          <Switch id="randomize-switch" checked={randomize} onCheckedChange={setRandomize} />
        </div>
        <p className="mt-1 text-xs text-muted-foreground">
          Add slight random variations to timing to avoid detection
        </p>
      </div>
    </div>
  )
}
