pub mod store {
    pub const TEMP: &str = "temp";
    pub const AUTOCLICKER: &str = "autoclicker";
}

pub mod temp_keys {
    pub const IS_RUNNING: &str = "isRunning";
    pub const HOTKEY_LEFT_ACTIVE: &str = "hotkeyLeftActive";
    pub const HOTKEY_RIGHT_ACTIVE: &str = "hotkeyRightActive";
}

pub mod autoclicker_keys {
    pub const HOTKEY_LEFT: &str = "hotkeyLeft";
    pub const HOTKEY_RIGHT: &str = "hotkeyRight";
    pub const HOLD_MODE: &str = "holdMode";
    pub const CLICK_SPEED: &str = "clickSpeed";
}
