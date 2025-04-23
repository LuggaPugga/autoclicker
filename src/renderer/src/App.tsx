import { MousePointerClick, Play, Square } from 'lucide-react'
import { Button } from './components/ui/button'
import { ThemeProvider } from './lib/theme-provider'
import { ThemeToggle } from './components/theme-toggle'
import { SpeedControl } from './components/speed-control'
import { HotkeyControl } from './components/hotkey-control'
import { useState, useCallback } from 'react'

const STORAGE_KEYS = {
  LEFT_HOTKEY: 'autoclicker-leftHotkey',
  RIGHT_HOTKEY: 'autoclicker-rightHotkey',
}

function App(): React.ReactElement {
  const [isRunning, setIsRunning] = useState(false)

  const toggleAutoClicker = useCallback(async (): Promise<void> => {
    const newState = !isRunning
    setIsRunning(newState)
    await window.api.toggleAutoClicker(newState)
  }, [isRunning])

  const handleHotkeyChange = useCallback((type: 'left' | 'right', hotkey: string): void => {
    if (type === 'left') {
      localStorage.setItem(STORAGE_KEYS.LEFT_HOTKEY, hotkey)
    } else {
      localStorage.setItem(STORAGE_KEYS.RIGHT_HOTKEY, hotkey)
    }
  }, [])

  return (
    <ThemeProvider defaultTheme="system" storageKey="autoclicker-theme">
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
                  className={`h-2 w-2 rounded-full ${isRunning ? 'bg-green-400 animate-pulse' : 'bg-gray-400'}`}
                ></div>
                <span className="text-xs font-medium text-muted-foreground">
                  {isRunning ? 'Running' : 'Not Running'}
                </span>
              </div>
              <ThemeToggle />
            </div>
          </div>
        </header>

        <main className="flex p-4 flex-col gap-4">
          <SpeedControl />
          <HotkeyControl onHotkeyChange={handleHotkeyChange} isListening={isRunning} />
        </main>

        <div className="p-4 flex flex-1 items-end">
          <Button
            className={`w-full ${isRunning ? 'bg-red-600 hover:bg-red-700' : ''}`}
            onClick={toggleAutoClicker}
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
