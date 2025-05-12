import { useState } from "react"
import "./globals.css"
import { ThemeProvider } from "@/lib/theme-provider"
import { MousePointerClick } from "lucide-react"
import { ThemeToggle } from "@/components/theme-toggle"

function App() {
  const [isRunning, setIsRunning] = useState(false)

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
      </div>
    </ThemeProvider>
  )
}

export default App
