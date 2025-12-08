use crate::state::AppState;
use std::sync::{atomic::Ordering, Arc};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use device_query::{DeviceQuery, DeviceState, Keycode};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    collections::HashSet,
    str::FromStr,
    thread,
    time::Duration,
};

#[cfg(target_os = "linux")]
use evdev::{Device, KeyCode};
#[cfg(target_os = "linux")]
use nix::fcntl::{fcntl, FcntlArg, OFlag};
#[cfg(target_os = "linux")]
use std::{
    collections::HashSet,
    fs,
    os::fd::AsRawFd,
    str::FromStr,
    thread,
    time::Duration,
};

#[cfg(target_os = "linux")]
fn key_to_evdev(s: &str) -> Option<KeyCode> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let upper = s.to_uppercase();

    if let Ok(key) = KeyCode::from_str(&format!("KEY_{}", upper)) {
        return Some(key);
    }

    let evdev_name = match upper.as_str() {
        "CTRL" | "CONTROL" => "KEY_LEFTCTRL",
        "SHIFT" => "KEY_LEFTSHIFT",
        "ALT" => "KEY_LEFTALT",
        "META" | "SUPER" | "WIN" | "WINDOWS" => "KEY_LEFTMETA",
        "ESCAPE" | "ESC" => "KEY_ESC",
        "RETURN" | "ENTER" => "KEY_ENTER",
        "SPACE" => "KEY_SPACE",
        "TAB" => "KEY_TAB",
        "BACKSPACE" | "BACK" => "KEY_BACKSPACE",
        "DELETE" | "DEL" => "KEY_DELETE",
        "INSERT" | "INS" => "KEY_INSERT",
        "HOME" => "KEY_HOME",
        "END" => "KEY_END",
        "PAGEUP" | "PGUP" => "KEY_PAGEUP",
        "PAGEDOWN" | "PGDN" | "PGDOWN" => "KEY_PAGEDOWN",
        "ARROWUP" | "UP" => "KEY_UP",
        "ARROWDOWN" | "DOWN" => "KEY_DOWN",
        "ARROWLEFT" | "LEFT" => "KEY_LEFT",
        "ARROWRIGHT" | "RIGHT" => "KEY_RIGHT",
        "PRINTSCREEN" | "PRTSC" | "PRINT" => "KEY_PRINT",
        "SCROLLLOCK" => "KEY_SCROLLLOCK",
        "PAUSE" | "BREAK" => "KEY_PAUSE",
        "NUMLOCK" => "KEY_NUMLOCK",
        "CAPSLOCK" | "CAPS" => "KEY_CAPSLOCK",
        "MOUSEBUTTON4" | "MOUSE4" | "MB4" => "BTN_SIDE",
        "MOUSEBUTTON5" | "MOUSE5" | "MB5" => "BTN_EXTRA",
        _ => return None,
    };

    KeyCode::from_str(evdev_name).ok()
}

#[cfg(target_os = "linux")]
fn parse_hotkey(hotkey_str: &str) -> Option<(HashSet<KeyCode>, KeyCode)> {
    if hotkey_str.is_empty() {
        return None;
    }

    let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = HashSet::new();
    let mut main_key = None;
    let is_single_key = parts.len() == 1;

    for part in parts {
        let upper = part.to_uppercase();

        match upper.as_str() {
            "SHIFT" => {
                if is_single_key {
                    main_key = Some(KeyCode::KEY_LEFTSHIFT);
                } else {
                    modifiers.insert(KeyCode::KEY_LEFTSHIFT);
                    modifiers.insert(KeyCode::KEY_RIGHTSHIFT);
                }
            }
            "CTRL" | "CONTROL" => {
                if is_single_key {
                    main_key = Some(KeyCode::KEY_LEFTCTRL);
                } else {
                    modifiers.insert(KeyCode::KEY_LEFTCTRL);
                    modifiers.insert(KeyCode::KEY_RIGHTCTRL);
                }
            }
            "ALT" => {
                if is_single_key {
                    main_key = Some(KeyCode::KEY_LEFTALT);
                } else {
                    modifiers.insert(KeyCode::KEY_LEFTALT);
                    modifiers.insert(KeyCode::KEY_RIGHTALT);
                }
            }
            "META" | "SUPER" | "WIN" | "WINDOWS" => {
                if is_single_key {
                    main_key = Some(KeyCode::KEY_LEFTMETA);
                } else {
                    modifiers.insert(KeyCode::KEY_LEFTMETA);
                    modifiers.insert(KeyCode::KEY_RIGHTMETA);
                }
            }
            _ => {
                if let Some(key) = key_to_evdev(part) {
                    main_key = Some(key);
                }
            }
        }
    }

    main_key.map(|key| (modifiers, key))
}

#[cfg(target_os = "linux")]
fn check_modifier(pressed: &HashSet<KeyCode>, left: KeyCode, right: KeyCode) -> bool {
    pressed.contains(&left) || pressed.contains(&right)
}

#[cfg(target_os = "linux")]
fn check_hotkey(pressed_keys: &HashSet<KeyCode>, hotkey_str: &str) -> bool {
    let Some((modifiers, main_key)) = parse_hotkey(hotkey_str) else {
        return false;
    };

    let main_key_pressed = match main_key {
        KeyCode::KEY_LEFTSHIFT => check_modifier(
            pressed_keys,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_RIGHTSHIFT,
        ),
        KeyCode::KEY_LEFTCTRL => {
            check_modifier(pressed_keys, KeyCode::KEY_LEFTCTRL, KeyCode::KEY_RIGHTCTRL)
        }
        KeyCode::KEY_LEFTALT => {
            check_modifier(pressed_keys, KeyCode::KEY_LEFTALT, KeyCode::KEY_RIGHTALT)
        }
        KeyCode::KEY_LEFTMETA => {
            check_modifier(pressed_keys, KeyCode::KEY_LEFTMETA, KeyCode::KEY_RIGHTMETA)
        }
        _ => pressed_keys.contains(&main_key),
    };

    if !main_key_pressed {
        return false;
    }

    if modifiers.contains(&KeyCode::KEY_LEFTSHIFT)
        && !check_modifier(
            pressed_keys,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_RIGHTSHIFT,
        )
    {
        return false;
    }
    if modifiers.contains(&KeyCode::KEY_LEFTCTRL)
        && !check_modifier(pressed_keys, KeyCode::KEY_LEFTCTRL, KeyCode::KEY_RIGHTCTRL)
    {
        return false;
    }
    if modifiers.contains(&KeyCode::KEY_LEFTALT)
        && !check_modifier(pressed_keys, KeyCode::KEY_LEFTALT, KeyCode::KEY_RIGHTALT)
    {
        return false;
    }
    if modifiers.contains(&KeyCode::KEY_LEFTMETA)
        && !check_modifier(pressed_keys, KeyCode::KEY_LEFTMETA, KeyCode::KEY_RIGHTMETA)
    {
        return false;
    }

    true
}

#[cfg(target_os = "linux")]
fn find_input_devices() -> Vec<Device> {
    let mut devices = Vec::new();

    let Ok(entries) = fs::read_dir("/dev/input") else {
        return devices;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !name.starts_with("event") {
            continue;
        }

        let Ok(device) = Device::open(&path) else {
            continue;
        };

        let has_keys = device
            .supported_keys()
            .map(|keys| keys.contains(KeyCode::KEY_A) || keys.contains(KeyCode::BTN_SIDE))
            .unwrap_or(false);

        if has_keys {
            if fcntl(device.as_raw_fd(), FcntlArg::F_SETFL(OFlag::O_NONBLOCK)).is_err() {
                continue;
            }
            devices.push(device);
        }
    }

    devices
}

#[cfg(target_os = "linux")]
pub fn start_hotkey_listener(state: Arc<AppState>) {
    thread::spawn(move || {
        let mut devices = find_input_devices();

        if devices.is_empty() {
            state
                .runtime
                .hotkeys_available
                .store(false, Ordering::SeqCst);
            return;
        }

        state
            .runtime
            .hotkeys_available
            .store(true, Ordering::SeqCst);

        let mut pressed_keys: HashSet<KeyCode> = HashSet::new();
        let mut prev_left_pressed = false;
        let mut prev_right_pressed = false;

        loop {
            let is_running = state.runtime.is_running.load(Ordering::SeqCst);

            if !is_running {
                if state.settings.read().hold_mode {
                    state
                        .runtime
                        .hotkey_left_active
                        .store(false, Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_right_active
                        .store(false, Ordering::SeqCst);
                }
                pressed_keys.clear();
                prev_left_pressed = false;
                prev_right_pressed = false;
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            let mut all_pressed = HashSet::new();
            for device in &mut devices {
                let _ = device.fetch_events();
                if let Ok(key_state) = device.get_key_state() {
                    for key in key_state.iter() {
                        all_pressed.insert(key);
                    }
                }
            }
            pressed_keys = all_pressed;

            let settings = state.settings.read();
            let hold_mode = settings.hold_mode;
            let hotkey_left = settings.hotkey_left.clone();
            let hotkey_right = settings.hotkey_right.clone();
            drop(settings);

            let left_pressed = !hotkey_left.is_empty() && check_hotkey(&pressed_keys, &hotkey_left);
            let right_pressed =
                !hotkey_right.is_empty() && check_hotkey(&pressed_keys, &hotkey_right);

            if hold_mode {
                state
                    .runtime
                    .hotkey_left_active
                    .store(left_pressed, Ordering::SeqCst);
                state
                    .runtime
                    .hotkey_right_active
                    .store(right_pressed, Ordering::SeqCst);
            } else {
                if left_pressed && !prev_left_pressed {
                    let current = state.runtime.hotkey_left_active.load(Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_left_active
                        .store(!current, Ordering::SeqCst);
                }
                if right_pressed && !prev_right_pressed {
                    let current = state.runtime.hotkey_right_active.load(Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_right_active
                        .store(!current, Ordering::SeqCst);
                }
            }

            prev_left_pressed = left_pressed;
            prev_right_pressed = right_pressed;

            thread::sleep(Duration::from_millis(5));
        }
    });
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn key_to_device_query(s: &str) -> Option<Keycode> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let upper = s.to_uppercase();

    if let Ok(key) = Keycode::from_str(&upper) {
        return Some(key);
    }

    let keycode_name = match upper.as_str() {
        "CTRL" | "CONTROL" => "LControl",
        "SHIFT" => "LShift",
        "ALT" => "LAlt",
        "META" | "SUPER" | "WIN" | "WINDOWS" => "LWin",
        "ESCAPE" | "ESC" => "Escape",
        "RETURN" | "ENTER" => "Enter",
        "SPACE" => "Space",
        "TAB" => "Tab",
        "BACKSPACE" | "BACK" => "Backspace",
        "DELETE" | "DEL" => "Delete",
        "INSERT" | "INS" => "Insert",
        "HOME" => "Home",
        "END" => "End",
        "PAGEUP" | "PGUP" => "PageUp",
        "PAGEDOWN" | "PGDN" | "PGDOWN" => "PageDown",
        "ARROWUP" | "UP" => "Up",
        "ARROWDOWN" | "DOWN" => "Down",
        "ARROWLEFT" | "LEFT" => "Left",
        "ARROWRIGHT" | "RIGHT" => "Right",
        "PRINTSCREEN" | "PRTSC" | "PRINT" => "PrintScreen",
        "SCROLLLOCK" => "ScrollLock",
        "PAUSE" | "BREAK" => "Pause",
        "NUMLOCK" => "NumLock",
        "CAPSLOCK" | "CAPS" => "CapsLock",
        "MOUSEBUTTON4" | "MOUSE4" | "MB4" => "Mouse4",
        "MOUSEBUTTON5" | "MOUSE5" | "MB5" => "Mouse5",
        _ => return None,
    };

    Keycode::from_str(keycode_name).ok()
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn parse_hotkey_device_query(hotkey_str: &str) -> Option<(HashSet<Keycode>, Keycode)> {
    if hotkey_str.is_empty() {
        return None;
    }

    let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = HashSet::new();
    let mut main_key = None;
    let is_single_key = parts.len() == 1;

    for part in parts {
        let upper = part.to_uppercase();

        match upper.as_str() {
            "SHIFT" => {
                if is_single_key {
                    if let Ok(key) = Keycode::from_str("LShift") {
                        main_key = Some(key);
                    }
                } else {
                    if let Ok(left) = Keycode::from_str("LShift") {
                        modifiers.insert(left);
                    }
                    if let Ok(right) = Keycode::from_str("RShift") {
                        modifiers.insert(right);
                    }
                }
            }
            "CTRL" | "CONTROL" => {
                if is_single_key {
                    if let Ok(key) = Keycode::from_str("LControl") {
                        main_key = Some(key);
                    }
                } else {
                    if let Ok(left) = Keycode::from_str("LControl") {
                        modifiers.insert(left);
                    }
                    if let Ok(right) = Keycode::from_str("RControl") {
                        modifiers.insert(right);
                    }
                }
            }
            "ALT" => {
                if is_single_key {
                    if let Ok(key) = Keycode::from_str("LAlt") {
                        main_key = Some(key);
                    }
                } else {
                    if let Ok(left) = Keycode::from_str("LAlt") {
                        modifiers.insert(left);
                    }
                    if let Ok(right) = Keycode::from_str("RAlt") {
                        modifiers.insert(right);
                    }
                }
            }
            "META" | "SUPER" | "WIN" | "WINDOWS" => {
                if is_single_key {
                    if let Ok(key) = Keycode::from_str("LWin") {
                        main_key = Some(key);
                    }
                } else {
                    if let Ok(left) = Keycode::from_str("LWin") {
                        modifiers.insert(left);
                    }
                    if let Ok(right) = Keycode::from_str("RWin") {
                        modifiers.insert(right);
                    }
                }
            }
            _ => {
                if let Some(key) = key_to_device_query(part) {
                    main_key = Some(key);
                }
            }
        }
    }

    main_key.map(|key| (modifiers, key))
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn check_modifier_device_query(
    pressed: &HashSet<Keycode>,
    left_name: &str,
    right_name: &str,
) -> bool {
    let left_ok = Keycode::from_str(left_name)
        .map(|k| pressed.contains(&k))
        .unwrap_or(false);
    let right_ok = Keycode::from_str(right_name)
        .map(|k| pressed.contains(&k))
        .unwrap_or(false);
    left_ok || right_ok
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn check_hotkey_device_query(pressed_keys: &HashSet<Keycode>, hotkey_str: &str) -> bool {
    let Some((modifiers, main_key)) = parse_hotkey_device_query(hotkey_str) else {
        return false;
    };

    if let Ok(lshift) = Keycode::from_str("LShift") {
        if main_key == lshift {
            return check_modifier_device_query(pressed_keys, "LShift", "RShift");
        }
    }
    if let Ok(lctrl) = Keycode::from_str("LControl") {
        if main_key == lctrl {
            return check_modifier_device_query(pressed_keys, "LControl", "RControl");
        }
    }
    if let Ok(lalt) = Keycode::from_str("LAlt") {
        if main_key == lalt {
            return check_modifier_device_query(pressed_keys, "LAlt", "RAlt");
        }
    }
    if let Ok(lwin) = Keycode::from_str("LWin") {
        if main_key == lwin {
            return check_modifier_device_query(pressed_keys, "LWin", "RWin");
        }
    }

    let main_key_pressed = pressed_keys.contains(&main_key);

    if !main_key_pressed {
        return false;
    }

    if let Ok(lshift) = Keycode::from_str("LShift") {
        if modifiers.contains(&lshift)
            && !check_modifier_device_query(pressed_keys, "LShift", "RShift")
        {
            return false;
        }
    }
    if let Ok(lctrl) = Keycode::from_str("LControl") {
        if modifiers.contains(&lctrl)
            && !check_modifier_device_query(pressed_keys, "LControl", "RControl")
        {
            return false;
        }
    }
    if let Ok(lalt) = Keycode::from_str("LAlt") {
        if modifiers.contains(&lalt)
            && !check_modifier_device_query(pressed_keys, "LAlt", "RAlt")
        {
            return false;
        }
    }
    if let Ok(lwin) = Keycode::from_str("LWin") {
        if modifiers.contains(&lwin)
            && !check_modifier_device_query(pressed_keys, "LWin", "RWin")
        {
            return false;
        }
    }

    true
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn start_hotkey_listener(state: Arc<AppState>) {
    thread::spawn(move || {
        let device_state = DeviceState::new();

        state
            .runtime
            .hotkeys_available
            .store(true, Ordering::SeqCst);

        let mut prev_left_pressed = false;
        let mut prev_right_pressed = false;

        loop {
            let is_running = state.runtime.is_running.load(Ordering::SeqCst);

            if !is_running {
                if state.settings.read().hold_mode {
                    state
                        .runtime
                        .hotkey_left_active
                        .store(false, Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_right_active
                        .store(false, Ordering::SeqCst);
                }
                prev_left_pressed = false;
                prev_right_pressed = false;
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            let keys_vec = device_state.get_keys();
            let pressed_keys: HashSet<Keycode> = keys_vec.into_iter().collect();

            let settings = state.settings.read();
            let hold_mode = settings.hold_mode;
            let hotkey_left = settings.hotkey_left.clone();
            let hotkey_right = settings.hotkey_right.clone();
            drop(settings);

            let left_pressed =
                !hotkey_left.is_empty() && check_hotkey_device_query(&pressed_keys, &hotkey_left);
            let right_pressed = !hotkey_right.is_empty()
                && check_hotkey_device_query(&pressed_keys, &hotkey_right);

            if hold_mode {
                state
                    .runtime
                    .hotkey_left_active
                    .store(left_pressed, Ordering::SeqCst);
                state
                    .runtime
                    .hotkey_right_active
                    .store(right_pressed, Ordering::SeqCst);
            } else {
                if left_pressed && !prev_left_pressed {
                    let current = state.runtime.hotkey_left_active.load(Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_left_active
                        .store(!current, Ordering::SeqCst);
                }
                if right_pressed && !prev_right_pressed {
                    let current = state.runtime.hotkey_right_active.load(Ordering::SeqCst);
                    state
                        .runtime
                        .hotkey_right_active
                        .store(!current, Ordering::SeqCst);
                }
            }

            prev_left_pressed = left_pressed;
            prev_right_pressed = right_pressed;

            thread::sleep(Duration::from_millis(5));
        }
    });
}
