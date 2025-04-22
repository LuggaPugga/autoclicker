import { app, shell, BrowserWindow, ipcMain, globalShortcut } from 'electron'
import { join } from 'path'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'
import icon from '../../resources/icon.png?asset'
import { AutoClicker } from './autoClicker'

let mainWindow: BrowserWindow | null = null
let autoClicker: AutoClicker | null = null
let leftClickShortcut: string | null = null
let rightClickShortcut: string | null = null
let isListening: boolean = false

function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 450,
    height: process.platform === 'win32' ? 660 : 620,
    fullscreenable: false,
    resizable: false,
    show: false,
    title: 'AutoClicker',
    autoHideMenuBar: true,
    ...(process.platform === 'linux' ? { icon } : {}),
    webPreferences: {
      preload: join(__dirname, '../preload/index.mjs'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false,
    },
  })

  if (!mainWindow) {
    console.error('Failed to create main window')
    return
  }

  autoClicker = new AutoClicker(mainWindow)

  mainWindow.on('ready-to-show', () => {
    if (mainWindow) mainWindow.show()
  })

  mainWindow.webContents.setWindowOpenHandler(details => {
    shell.openExternal(details.url)
    return { action: 'deny' }
  })

  if (is.dev && process.env['ELECTRON_RENDERER_URL']) {
    mainWindow.loadURL(process.env['ELECTRON_RENDERER_URL'])
  } else {
    mainWindow.loadFile(join(__dirname, '../renderer/index.html'))
  }
}

function toggleListening(newState: boolean): void {
  if (!autoClicker) return

  isListening = newState
  autoClicker.setListening(newState)

  if (!newState) {
    // If turning off listening, unregister hotkeys
    if (leftClickShortcut) {
      globalShortcut.unregister(leftClickShortcut)
    }
    if (rightClickShortcut) {
      globalShortcut.unregister(rightClickShortcut)
    }
  } else {
    // When turning on listening, register stored hotkeys
    if (leftClickShortcut) {
      globalShortcut.register(leftClickShortcut, () => {
        if (!autoClicker) return
        if (autoClicker.getCurrentButton() === 'left' && autoClicker.isRunning()) {
          autoClicker.stopAutoClicker()
        } else {
          autoClicker.startAutoClicker('left')
        }
      })
    }
    if (rightClickShortcut) {
      globalShortcut.register(rightClickShortcut, () => {
        if (!autoClicker) return
        if (autoClicker.getCurrentButton() === 'right' && autoClicker.isRunning()) {
          autoClicker.stopAutoClicker()
        } else {
          autoClicker.startAutoClicker('right')
        }
      })
    }
  }

  if (mainWindow && !mainWindow.isDestroyed()) {
    mainWindow.webContents.send('hotkey-state-update', {
      leftActive: autoClicker.getCurrentButton() === 'left' && autoClicker.isRunning(),
      rightActive: autoClicker.getCurrentButton() === 'right' && autoClicker.isRunning(),
    })
  }
}

function registerGlobalShortcut(accelerator: string, mouseButton: 'left' | 'right'): boolean {
  try {
    if (!autoClicker) return false

    const existingShortcut = mouseButton === 'left' ? leftClickShortcut : rightClickShortcut
    if (existingShortcut && isListening) {
      globalShortcut.unregister(existingShortcut)
    }

    if (mouseButton === 'left') {
      leftClickShortcut = accelerator
    } else {
      rightClickShortcut = accelerator
    }

    if (isListening) {
      return globalShortcut.register(accelerator, () => {
        if (!autoClicker) return

        if (autoClicker.getCurrentButton() === mouseButton && autoClicker.isRunning()) {
          autoClicker.stopAutoClicker()
        } else {
          autoClicker.startAutoClicker(mouseButton)
        }
      })
    }

    return true
  } catch (error) {
    console.error(`Failed to register ${mouseButton} click shortcut:`, error)
    return false
  }
}

// IPC handlers
ipcMain.handle('toggle-auto-clicker', (_, newState: boolean) => {
  toggleListening(newState)
  return true
})

ipcMain.handle('set-click-interval', (_, interval: number) => {
  if (!autoClicker) return false
  autoClicker.setClickInterval(interval)
  return true
})

ipcMain.handle('set-randomize', (_, randomize: boolean) => {
  if (!autoClicker) return false
  autoClicker.setRandomize(randomize)
  return true
})

ipcMain.handle('set-mouse-button', (_, button: 'left' | 'right') => {
  if (!autoClicker) return false
  autoClicker.setMouseButton(button)
  return true
})

ipcMain.handle('register-hotkey', (_, type: 'left' | 'right', hotkey: string) => {
  return registerGlobalShortcut(hotkey, type)
})

// For testing hotkey activation
ipcMain.handle('test-hotkey-activation', (_, type: 'left' | 'right') => {
  if (!autoClicker || !mainWindow || mainWindow.isDestroyed()) return false

  autoClicker.setMouseButton(type)
  const isRunning = autoClicker.isRunning()
  if (isRunning) {
    autoClicker.stopAutoClicker()
  } else {
    autoClicker.startAutoClicker(type)
  }
  return true
})

app.whenReady().then(() => {
  electronApp.setAppUserModelId('com.electron')

  app.on('browser-window-created', (_, window) => {
    optimizer.watchWindowShortcuts(window)
  })

  ipcMain.on('ping', () => console.log('pong'))

  createWindow()

  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('will-quit', () => {
  // Unregister all shortcuts when app is about to quit
  globalShortcut.unregisterAll()

  // Stop auto-clicker if running
  if (autoClicker) {
    autoClicker.setListening(false)
  }
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

// In this file you can include the rest of your app"s specific main process
// code. You can also put them in separate files and require them here.
