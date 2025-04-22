import { BrowserWindow } from 'electron'
import robot from '@jitsi/robotjs'

export class AutoClicker {
  private isListening = false
  private isAutoClickerRunning = false
  private currentMouseButton: 'left' | 'right' = 'left'
  private clickInterval = 200
  private useRandomize = false
  private clickLoopTimeout: NodeJS.Timeout | null = null
  private mainWindow: BrowserWindow | null = null

  constructor(mainWindow: BrowserWindow) {
    this.mainWindow = mainWindow
  }

  setMainWindow(window: BrowserWindow): void {
    this.mainWindow = window
  }

  private updateUIState(): void {
    if (this.mainWindow && !this.mainWindow.isDestroyed()) {
      this.mainWindow.webContents.send('hotkey-state-update', {
        leftActive: this.currentMouseButton === 'left' && this.isAutoClickerRunning,
        rightActive: this.currentMouseButton === 'right' && this.isAutoClickerRunning,
      })
    }
  }

  private simulateMouseClick(): void {
    if (!this.isAutoClickerRunning) return
    robot.mouseClick(this.currentMouseButton)
  }

  private startAutoClickerLoop(): void {
    if (this.clickLoopTimeout) clearTimeout(this.clickLoopTimeout)

    const autoClickLoop = (): void => {
      if (!this.isAutoClickerRunning) return

      this.simulateMouseClick()

      let nextInterval = this.clickInterval
      if (this.useRandomize) {
        const variation = this.clickInterval * 0.1
        nextInterval = this.clickInterval + Math.floor(Math.random() * variation * 2) - variation
      }

      this.clickLoopTimeout = setTimeout(autoClickLoop, nextInterval)
    }

    autoClickLoop()
  }

  startAutoClicker(mouseButton: 'left' | 'right'): void {
    if (!this.isListening) return

    this.currentMouseButton = mouseButton
    this.isAutoClickerRunning = true
    this.startAutoClickerLoop()
    this.updateUIState()
  }

  stopAutoClicker(): void {
    if (this.clickLoopTimeout) {
      clearTimeout(this.clickLoopTimeout)
      this.clickLoopTimeout = null
    }

    this.isAutoClickerRunning = false
    this.updateUIState()
  }

  setClickInterval(interval: number): void {
    this.clickInterval = interval
  }

  setRandomize(randomize: boolean): void {
    this.useRandomize = randomize
  }

  setMouseButton(button: 'left' | 'right'): void {
    this.currentMouseButton = button
    this.updateUIState()
  }

  isRunning(): boolean {
    return this.isAutoClickerRunning
  }

  getCurrentButton(): 'left' | 'right' {
    return this.currentMouseButton
  }

  setListening(state: boolean): void {
    this.isListening = state
    if (!state) {
      this.stopAutoClicker()
    }
  }
}
