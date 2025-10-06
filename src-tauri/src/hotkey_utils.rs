use device_query::Keycode;

pub fn keycode_from_string(s: &str) -> Option<Keycode> {
    match s.to_uppercase().as_str() {
        // Letters
        "A" => Some(Keycode::A),
        "B" => Some(Keycode::B),
        "C" => Some(Keycode::C),
        "D" => Some(Keycode::D),
        "E" => Some(Keycode::E),
        "F" => Some(Keycode::F),
        "G" => Some(Keycode::G),
        "H" => Some(Keycode::H),
        "I" => Some(Keycode::I),
        "J" => Some(Keycode::J),
        "K" => Some(Keycode::K),
        "L" => Some(Keycode::L),
        "M" => Some(Keycode::M),
        "N" => Some(Keycode::N),
        "O" => Some(Keycode::O),
        "P" => Some(Keycode::P),
        "Q" => Some(Keycode::Q),
        "R" => Some(Keycode::R),
        "S" => Some(Keycode::S),
        "T" => Some(Keycode::T),
        "U" => Some(Keycode::U),
        "V" => Some(Keycode::V),
        "W" => Some(Keycode::W),
        "X" => Some(Keycode::X),
        "Y" => Some(Keycode::Y),
        "Z" => Some(Keycode::Z),
        // Numbers
        "0" => Some(Keycode::Key0),
        "1" => Some(Keycode::Key1),
        "2" => Some(Keycode::Key2),
        "3" => Some(Keycode::Key3),
        "4" => Some(Keycode::Key4),
        "5" => Some(Keycode::Key5),
        "6" => Some(Keycode::Key6),
        "7" => Some(Keycode::Key7),
        "8" => Some(Keycode::Key8),
        "9" => Some(Keycode::Key9),
        // F keys
        "F1" => Some(Keycode::F1),
        "F2" => Some(Keycode::F2),
        "F3" => Some(Keycode::F3),
        "F4" => Some(Keycode::F4),
        "F5" => Some(Keycode::F5),
        "F6" => Some(Keycode::F6),
        "F7" => Some(Keycode::F7),
        "F8" => Some(Keycode::F8),
        "F9" => Some(Keycode::F9),
        "F10" => Some(Keycode::F10),
        "F11" => Some(Keycode::F11),
        "F12" => Some(Keycode::F12),
        // Modifiers (handled separately in check_hotkey, but might be useful)
        "LSHIFT" => Some(Keycode::LShift),
        "RSHIFT" => Some(Keycode::RShift),
        "LCTRL" | "LCONTROL" => Some(Keycode::LControl),
        "RCTRL" | "RCONTROL" => Some(Keycode::RControl),
        "LALT" => Some(Keycode::LAlt),
        "RALT" => Some(Keycode::RAlt),
        // Others (Add more as needed)
        "SPACE" => Some(Keycode::Space),
        "ENTER" => Some(Keycode::Enter),
        "TAB" => Some(Keycode::Tab),
        "BACKSPACE" => Some(Keycode::Backspace),
        "CAPSLOCK" => Some(Keycode::CapsLock),
        _ => None,
    }
}

pub fn check_hotkey(pressed_keys: &Vec<Keycode>, hotkey_string: &str) -> bool {
    if hotkey_string.is_empty() {
        return false;
    }

    let required_key_strings: Vec<&str> = hotkey_string.split('+').map(|s| s.trim()).collect();
    let mut required_non_modifier_keycodes: Vec<Keycode> = Vec::new();
    let mut require_shift = false;
    let mut require_ctrl = false;
    let mut require_alt = false;

    for key_str in required_key_strings {
        match key_str.to_uppercase().as_str() {
            "SHIFT" => require_shift = true,
            "CTRL" | "CONTROL" => require_ctrl = true,
            "ALT" => require_alt = true,
            _ => {
                // Normal key
                match keycode_from_string(key_str) {
                    Some(kc) => required_non_modifier_keycodes.push(kc),
                    None => {
                        eprintln!("Warning: Unknown key in hotkey string: {}", key_str);
                        return false; // Unknown key means hotkey can't be matched
                    }
                }
            }
        }
    }

    if require_shift
        && !pressed_keys.contains(&Keycode::LShift)
        && !pressed_keys.contains(&Keycode::RShift)
    {
        return false;
    }
    if require_ctrl
        && !pressed_keys.contains(&Keycode::LControl)
        && !pressed_keys.contains(&Keycode::RControl)
    {
        return false;
    }
    if require_alt
        && !pressed_keys.contains(&Keycode::LAlt)
        && !pressed_keys.contains(&Keycode::RAlt)
    {
        return false;
    }

    for kc in &required_non_modifier_keycodes {
        if !pressed_keys.contains(kc) {
            return false;
        }
    }

    if required_non_modifier_keycodes.is_empty() && (require_shift || require_ctrl || require_alt) {
        return false;
    }

    !required_non_modifier_keycodes.is_empty()
}
