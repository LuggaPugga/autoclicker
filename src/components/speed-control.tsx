import React, { useState } from "react"
import { Slider } from "@/components/ui/slider"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"
import { Switch } from "@/components/ui/switch"
import { Timer, Zap, Shuffle } from "lucide-react"
import { useAutoclickerStore } from "@/lib/autoclicker-store"

type SpeedMode = "cps" | "ms"

export function SpeedControl(): React.ReactElement {
  const { clickSpeed, setClickSpeed, holdMode, toggleHoldMode } = useAutoclickerStore()
  const [mode, setMode] = useState<SpeedMode>("cps")

  function getDisplayValue() {
    return mode === "cps" ? 1000 / clickSpeed : clickSpeed
  }

  function toMilliseconds(value: number) {
    return mode === "cps" ? 1000 / value : value
  }

  function handleModeToggle(newMode: SpeedMode): void {
    if (newMode === mode) return
    setMode(newMode)
  }

  const min = mode === "cps" ? 1 : 1
  const max = mode === "cps" ? 50 : 1000
  const step = mode === "cps" ? 1 : 1

  return (
    <div className="w-full max-w-md space-y-4">
      <div className="p-4 rounded-lg border border-border/50 bg-card space-y-6">
        <div className="flex justify-center space-x-2">
          <Button
            variant={mode === "cps" ? "default" : "outline"}
            className="flex-1"
            onClick={() => handleModeToggle("cps")}
          >
            <Zap className="h-4 w-4 mr-2" />
            Clicks per second
          </Button>
          <Button
            variant={mode === "ms" ? "default" : "outline"}
            className="flex-1"
            onClick={() => handleModeToggle("ms")}
          >
            <Timer className="h-4 w-4 mr-2" />
            Milliseconds
          </Button>
        </div>

        <div className="space-y-4">
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <Label htmlFor="speed-input" className="text-sm font-medium">
                {mode === "cps" ? "Clicks per second" : "Milliseconds between clicks"}
              </Label>
              <div className="bg-muted/30 rounded px-2 py-1 text-sm font-mono text-muted-foreground">
                {mode === "cps"
                  ? `${(1000 / clickSpeed).toFixed(1)} CPS (${clickSpeed.toFixed(1)}ms)`
                  : `${clickSpeed.toFixed(1)}ms (${(1000 / clickSpeed).toFixed(1)} CPS)`}
              </div>
            </div>

            <Slider
              value={[getDisplayValue()]}
              min={min}
              max={max}
              step={step}
              onValueChange={(v) => {
                const ms = toMilliseconds(v[0])
                setClickSpeed(mode === "cps" ? ms : toMilliseconds(ms))
              }}
              className="my-6"
            />
          </div>

          <div className="flex items-center gap-2">
            <Input
              id="speed-input"
              type="number"
              value={getDisplayValue()}
              onChange={(e) => {
                const numValue = parseFloat(e.target.value)
                if (!isNaN(numValue) && numValue > 0) {
                  setClickSpeed(toMilliseconds(numValue))
                } else {
                  setClickSpeed(mode === "cps" ? toMilliseconds(1.0) : 1.0)
                }
              }}
              className="w-24"
            />
            <span className="text-sm text-muted-foreground">
              {mode === "cps" ? "clicks per second" : "milliseconds"}
            </span>
          </div>
        </div>
      </div>

      <div className="p-3 rounded-md border border-border/30 bg-background/50">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Shuffle className="h-4 w-4 text-muted-foreground" />
            <Label htmlFor="hold-mode-switch" className="text-sm font-medium">
              Hold to Click Mode
            </Label>
          </div>

          <Switch id="hold-mode-switch" checked={holdMode} onCheckedChange={toggleHoldMode} />
        </div>
        <p className="mt-1 text-xs text-muted-foreground">
          When enabled, clicking only occurs while holding down the hotkey
        </p>
      </div>
    </div>
  )
}
