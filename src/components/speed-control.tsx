import { ShuffleIcon, Timer, Zap } from "lucide-solid"
import { createSignal } from "solid-js"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Switch } from "@/components/ui/switch"
import { useAutoclickerStore } from "@/lib/autoclicker-store"

type SpeedMode = "cps" | "ms"

export function SpeedControl() {
  const store = useAutoclickerStore()
  const [mode, setMode] = createSignal<SpeedMode>("cps")

  function getDisplayValue() {
    return mode() === "cps" ? 1000 / store.clickSpeed : store.clickSpeed
  }

  function toMilliseconds(value: number) {
    return mode() === "cps" ? 1000 / value : value
  }

  function handleModeToggle(newMode: SpeedMode): void {
    if (newMode === mode()) return
    setMode(newMode)
  }

  const min = mode() === "cps" ? 1 : 1
  const max = mode() === "cps" ? 50 : 1000
  const step = mode() === "cps" ? 1 : 1

  return (
    <div class="w-full max-w-md space-y-4">
      <div class="p-4 rounded-lg border border-border/50 bg-card space-y-6">
        <div class="flex justify-center space-x-2">
          <Button
            variant={mode() === "cps" ? "default" : "outline"}
            class="flex-1"
            onClick={() => handleModeToggle("cps")}
          >
            <Zap class="h-4 w-4 mr-2" />
            Clicks per second
          </Button>
          <Button
            variant={mode() === "ms" ? "default" : "outline"}
            class="flex-1"
            onClick={() => handleModeToggle("ms")}
          >
            <Timer class="h-4 w-4 mr-2" />
            Milliseconds
          </Button>
        </div>

        <div class="space-y-4">
          <div class="space-y-2">
            <div class="flex justify-between items-center">
              <Label for="speed-input" class="text-sm font-medium">
                {mode() === "cps" ? "Clicks per second" : "Milliseconds between clicks"}
              </Label>
              <div class="bg-muted/30 rounded px-2 py-1 text-sm font-mono text-muted-foreground">
                {mode() === "cps"
                  ? `${(1000 / store.clickSpeed).toFixed(1)} CPS (${store.clickSpeed.toFixed(1)}ms)`
                  : `${store.clickSpeed.toFixed(1)}ms (${(1000 / store.clickSpeed).toFixed(1)} CPS)`}
              </div>
            </div>

            <Slider
              value={[getDisplayValue()]}
              minValue={min}
              maxValue={max}
              step={step}
              onChange={(values) => {
                const value = Array.isArray(values) ? values[0] : values
                store.setClickSpeed(mode() === "cps" ? toMilliseconds(value) : value)
              }}
              class="my-6"
            />
          </div>

          <div class="flex items-center gap-2">
            <Input
              id="speed-input"
              type="number"
              value={getDisplayValue()}
              onInput={(e) => {
                const numValue = Number.parseFloat(e.currentTarget.value)
                if (!Number.isNaN(numValue) && numValue > 0) {
                  store.setClickSpeed(toMilliseconds(numValue))
                } else {
                  store.setClickSpeed(mode() === "cps" ? toMilliseconds(1.0) : 1.0)
                }
              }}
              class="w-24"
            />
            <span class="text-sm text-muted-foreground">
              {mode() === "cps" ? "clicks per second" : "milliseconds"}
            </span>
          </div>
        </div>
      </div>

      <div class="p-3 rounded-md border border-border/30 bg-background/50">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <ShuffleIcon class="h-4 w-4 text-muted-foreground" />
            <Label for="hold-mode-switch" class="text-sm font-medium">
              Hold to Click Mode
            </Label>
          </div>

          <Switch
            id="hold-mode-switch"
            checked={store.holdMode}
            onCueChange={store.toggleHoldMode}
          />
        </div>
        <p class="mt-1 text-xs text-muted-foreground">
          When enabled, clicking only occurs while holding down the hotkey
        </p>
      </div>
    </div>
  )
}
