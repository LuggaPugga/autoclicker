import "./globals.css"
import { MousePointerClick, Play, Square } from "lucide-solid"
import { ThemeToggle } from "@/components/theme-toggle"
import { useTempStore } from "@/lib/temp-store"
import { ThemeProvider } from "@/lib/theme-provider"
import { HotkeyControl } from "./components/hotkey-control"
import { SpeedControl } from "./components/speed-control"
import { Button } from "./components/ui/button"

function App() {
  const tempStore = useTempStore()

  return (
    <ThemeProvider>
      <div class="flex flex-col h-screen bg-background">
        <header class="border-b border-border/50 px-4 py-3">
          <div class="container mx-auto flex items-center justify-between">
            <div class="flex items-center gap-2">
              <MousePointerClick class="h-5 w-5 text-cyan-400" />
              <h1 class="font-semibold text-foreground">AutoClicker</h1>
            </div>
            <div class="flex items-center gap-4">
              <div class="flex items-center gap-2">
                <div
                  class={`h-2 w-2 rounded-full ${
                    tempStore.isRunning ? "bg-green-400 animate-pulse" : "bg-gray-400"
                  }`}
                ></div>
                <span class="text-xs font-medium text-muted-foreground">
                  {tempStore.isRunning ? "Running" : "Not Running"}
                </span>
              </div>
              <ThemeToggle />
            </div>
          </div>
        </header>

        <main class="flex p-4 flex-col gap-4">
          <SpeedControl />
          <HotkeyControl isListening={tempStore.isRunning} />
        </main>

        <div class="p-4 flex flex-1 items-end">
          <Button
            class={`w-full ${tempStore.isRunning ? "bg-red-600 hover:bg-red-700" : ""}`}
            onClick={tempStore.toggleIsRunning}
          >
            {tempStore.isRunning ? (
              <>
                <Square class="h-4 w-4 mr-2" />
                Stop Listening for Hotkey
              </>
            ) : (
              <>
                <Play class="h-4 w-4 mr-2" />
                Start Listening for Hotkey
              </>
            )}
          </Button>
        </div>
      </div>
    </ThemeProvider>
  )
}

export default App
