import React, { useState, useEffect } from 'react'
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
  const [mode, setMode] = useState<SpeedMode>(
    () => (localStorage.getItem(STORAGE_KEYS.MODE) as SpeedMode) || 'cps'
  )

  const [value, setValue] = useState(() => {
    const savedValue = localStorage.getItem(STORAGE_KEYS.VALUE)
    return savedValue ? parseInt(savedValue) : mode === 'cps' ? 5 : 200
  })

  const [randomize, setRandomize] = useState(
    () => localStorage.getItem(STORAGE_KEYS.RANDOMIZE) === 'true'
  )

  useEffect(() => {
    localStorage.setItem(STORAGE_KEYS.MODE, mode)
    localStorage.setItem(STORAGE_KEYS.VALUE, value.toString())
    localStorage.setItem(STORAGE_KEYS.RANDOMIZE, randomize.toString())

    const interval = mode === 'cps' ? Math.round(1000 / value) : value
    window.api.setClickInterval(interval)
    window.api.setRandomize(randomize)
  }, [value, mode, randomize])

  function handleModeToggle(newMode: SpeedMode): void {
    if (newMode === mode) return

    const newValue =
      newMode === 'cps' ? Math.max(1, Math.round(1000 / value)) : Math.round(1000 / value)

    setValue(newValue)
    setMode(newMode)
  }

  const min = mode === 'cps' ? 1 : 50
  const max = mode === 'cps' ? 50 : 1000
  const step = mode === 'cps' ? 1 : 10

  return (
    <div className="w-full max-w-md space-y-4">
      <div className="p-4 rounded-lg border border-border/50 bg-card space-y-6">
        <div className="flex justify-center space-x-2">
          <Button
            variant={mode === 'cps' ? 'default' : 'outline'}
            className="flex-1"
            onClick={() => handleModeToggle('cps')}
          >
            <Zap className="h-4 w-4 mr-2" />
            Clicks per second
          </Button>
          <Button
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
              onValueChange={v => setValue(v[0])}
              className="my-6"
            />
          </div>

          <div className="flex items-center gap-2">
            <Input
              id="speed-input"
              type="number"
              value={value}
              onChange={e => {
                const numValue = parseInt(e.target.value)
                if (!isNaN(numValue) && numValue !== 0) {
                  setValue(numValue)
                } else {
                  setValue(1)
                }
              }}
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
