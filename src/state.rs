use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ThemePreference {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoClickerSettings {
    pub hotkey_left: String,
    pub hotkey_right: String,
    pub click_speed_ms: f64,
    pub hold_mode: bool,
    #[serde(default)]
    pub theme: ThemePreference,
}

impl Default for AutoClickerSettings {
    fn default() -> Self {
        Self {
            hotkey_left: String::new(),
            hotkey_right: String::new(),
            click_speed_ms: 100.0,
            hold_mode: false,
            theme: ThemePreference::System,
        }
    }
}

#[derive(Debug, Default)]
pub struct RuntimeState {
    pub is_running: AtomicBool,
    pub hotkey_left_active: AtomicBool,
    pub hotkey_right_active: AtomicBool,
    pub hotkeys_available: AtomicBool,
}

pub struct AppState {
    pub settings: RwLock<AutoClickerSettings>,
    pub runtime: Arc<RuntimeState>,
    config_path: PathBuf,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let config_path = Self::get_config_path();
        let settings = Self::load_settings(&config_path);

        Arc::new(Self {
            settings: RwLock::new(settings),
            runtime: Arc::new(RuntimeState::default()),
            config_path,
        })
    }

    fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("autoclicker");
        fs::create_dir_all(&config_dir).ok();
        config_dir.join("settings.json")
    }

    fn load_settings(path: &PathBuf) -> AutoClickerSettings {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save_settings(&self) {
        let settings = self.settings.read();
        if let Ok(json) = serde_json::to_string_pretty(&*settings) {
            fs::write(&self.config_path, json).ok();
        }
    }

    pub fn set_hotkey_left(&self, hotkey: String) {
        self.settings.write().hotkey_left = hotkey;
        self.save_settings();
    }

    pub fn set_hotkey_right(&self, hotkey: String) {
        self.settings.write().hotkey_right = hotkey;
        self.save_settings();
    }

    pub fn set_click_speed(&self, speed_ms: f64) {
        self.settings.write().click_speed_ms = speed_ms;
        self.save_settings();
    }

    pub fn set_hold_mode(&self, enabled: bool) {
        self.settings.write().hold_mode = enabled;
        self.save_settings();
    }

    pub fn set_theme(&self, theme: ThemePreference) {
        self.settings.write().theme = theme;
        self.save_settings();
    }

    pub fn toggle_running(&self) {
        let current = self.runtime.is_running.load(Ordering::SeqCst);
        self.runtime.is_running.store(!current, Ordering::SeqCst);
    }
}
