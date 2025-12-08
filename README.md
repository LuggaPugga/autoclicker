# AutoClicker

![Autoclicker Screenshot](.github/banner.png)

> [!NOTE]
> Mouse button hotkeys are currently only supported on Windows. Keyboard hotkeys work on all platforms.

## About

I created this autoclicker because I couldn't find any open-source autoclickers that had all the features I wanted. This project is designed to be simple, powerful, and cross-platform, with a modern UI and essential features.

## Features

- **Hotkeys:** Easily start and stop clicking with customizable keyboard shortcuts.
- **CPS (Clicks Per Second):** Set and monitor your desired click speed.

## Tech Stack

- **GPUI** – High-performance GPU-accelerated UI framework from Zed
- **gpui-component** – UI component library for GPUI
- **Enigo** – Rust library for input simulation
- **evdev** – Linux input event handling
- **device_query** – Input detection for Windows and macOS

## Compatibility

- **Windows**
- **Linux** (X11)
- **Mac** (untested)
