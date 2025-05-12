import "./globals.css"
import { ThemeProvider } from "@/lib/theme-provider"
import { MousePointerClick, Square, Play } from "lucide-react"
import { ThemeToggle } from "@/components/theme-toggle"
import { Button } from "./components/ui/button"
import { SpeedControl } from "./components/speed-control"
import { useTempStore } from "@/lib/temp-store"
import { HotkeyControl } from "./components/hotkey-control"
function App() {
  const { isRunning, toggleIsRunning } = useTempStore()

  return (
    <ThemeProvider>
      <div className="flex flex-col h-screen bg-background">
        <header className="border-b border-border/50 px-4 py-3">
          <div className="container mx-auto flex items-center justify-between">
            <div className="flex items-center gap-2">
              <MousePointerClick className="h-5 w-5 text-cyan-400" />
              <h1 className="font-semibold text-foreground">AutoClicker</h1>
            </div>
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-2">
                <div
                  className={`h-2 w-2 rounded-full ${
                    isRunning ? "bg-green-400 animate-pulse" : "bg-gray-400"
                  }`}
                ></div>
                <span className="text-xs font-medium text-muted-foreground">
                  {isRunning ? "Running" : "Not Running"}
                </span>
              </div>
              <ThemeToggle />
            </div>
          </div>
        </header>

        <main className="flex p-4 flex-col gap-4">
          <SpeedControl />
          <HotkeyControl />
        </main>

        <div className="p-4 flex flex-1 items-end">
          <Button
            className={`w-full ${isRunning ? "bg-red-600 hover:bg-red-700" : ""}`}
            onClick={toggleIsRunning}
          >
            {isRunning ? (
              <>
                <Square className="h-4 w-4 mr-2" />
                Stop Listening for Hotkey
              </>
            ) : (
              <>
                <Play className="h-4 w-4 mr-2" />
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
