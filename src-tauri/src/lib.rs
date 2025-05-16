use tauri::Emitter;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;
use enigo::{Enigo, Mouse, Settings, Button, Direction::{Click}};
use tauri_plugin_zustand::ManagerExt;
use std::sync::{Arc, Mutex};

mod hotkey_utils;
mod zustand_keys;

use crate::zustand_keys::{autoclicker_keys, store, temp_keys};

fn get_mouse_button_index(hotkey_str: &str) -> Option<usize> {
    match hotkey_str {
        "MouseButton4" => Some(4),
        "MouseButton5" => Some(5),
        _ => None,
    }
}

fn is_mouse_button_pressed(mouse_buttons: &Vec<bool>, hotkey_str: &str) -> bool {
    match get_mouse_button_index(hotkey_str) {
        Some(index) => mouse_buttons.get(index).cloned().unwrap_or(false),
        None => false,
    }
}

fn was_mouse_button_just_pressed(current_buttons: &Vec<bool>, previous_buttons: &Vec<bool>, hotkey_str: &str) -> bool {
    if let Some(index) = get_mouse_button_index(hotkey_str) {
        let current_pressed = current_buttons.get(index).cloned().unwrap_or(false);
        let previous_pressed = previous_buttons.get(index).cloned().unwrap_or(false);
        current_pressed && !previous_pressed
    } else {
        false
    }
}

struct HotkeyManager {
    app_handle: tauri::AppHandle,
    is_running: Arc<Mutex<bool>>,
    hotkey_left: Arc<Mutex<String>>,
    hotkey_right: Arc<Mutex<String>>,
    hold_mode: Arc<Mutex<bool>>,
}

impl HotkeyManager {
    fn new(app_handle: tauri::AppHandle) -> Self {
        let initial_is_running = app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::IS_RUNNING).unwrap_or(false);
        let initial_hotkey_left = app_handle.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_LEFT).unwrap_or_default();
        let initial_hotkey_right = app_handle.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_RIGHT).unwrap_or_default();
        let initial_hold_mode = app_handle.zustand().try_get::<bool>(store::AUTOCLICKER, autoclicker_keys::HOLD_MODE).unwrap_or(false);

        let is_running = Arc::new(Mutex::new(initial_is_running));
        let hotkey_left = Arc::new(Mutex::new(initial_hotkey_left));
        let hotkey_right = Arc::new(Mutex::new(initial_hotkey_right));
        let hold_mode = Arc::new(Mutex::new(initial_hold_mode));

        let is_running_clone = Arc::clone(&is_running);
        let _ = app_handle.zustand().watch(store::TEMP, move |app| {
            if let Ok(new_val) = app.zustand().try_get::<bool>(store::TEMP, temp_keys::IS_RUNNING) {
                *is_running_clone.lock().unwrap() = new_val;
            }
            Ok(())
        });

        let hotkey_left_clone = Arc::clone(&hotkey_left);
        let hotkey_right_clone = Arc::clone(&hotkey_right);
        let hold_mode_clone = Arc::clone(&hold_mode);
        let _ = app_handle.zustand().watch(store::AUTOCLICKER, move |app| {
            if let Ok(new_val) = app.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_LEFT) {
                *hotkey_left_clone.lock().unwrap() = new_val;
            }
            if let Ok(new_val) = app.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_RIGHT) {
                *hotkey_right_clone.lock().unwrap() = new_val;
            }
            if let Ok(new_val) = app.zustand().try_get::<bool>(store::AUTOCLICKER, autoclicker_keys::HOLD_MODE) {
                *hold_mode_clone.lock().unwrap() = new_val;
            }
            Ok(())
        });

        Self {
            app_handle,
            is_running,
            hotkey_left,
            hotkey_right,
            hold_mode,
        }
    }

    fn start(self) {
        thread::spawn(move || {
            self.process_hotkeys_loop();
        });
    }

    fn update_hotkey_state(
        &self,
        zustand_key: &'static str,
        new_active_state: bool,
        emit_event_name: &'static str,
        mode_description: &str,
    ) {
        if let Err(e) = self.app_handle.zustand().set(store::TEMP, zustand_key, new_active_state) {
            eprintln!("Failed to set {} ({}) in Zustand store: {}", zustand_key, mode_description, e);
        }
        self.app_handle.emit(emit_event_name, new_active_state).unwrap_or_else(|e| {
            eprintln!("Failed to emit {} ({}): {}", emit_event_name, mode_description, e);
        });
    }
    
    fn handle_hold_mode(&self, current_keys: &Vec<Keycode>, current_mouse_buttons: &Vec<bool>) {
        let hotkey_left_str = self.hotkey_left.lock().unwrap().clone();
        let hotkey_right_str = self.hotkey_right.lock().unwrap().clone();

        if !hotkey_left_str.is_empty() {
            let left_hotkey_is_active = if get_mouse_button_index(&hotkey_left_str).is_some() {
                is_mouse_button_pressed(&current_mouse_buttons, &hotkey_left_str)
            } else {
                hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str)
            };
            let current_left_active_in_zustand = self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);
            if left_hotkey_is_active != current_left_active_in_zustand {
                self.update_hotkey_state(temp_keys::HOTKEY_LEFT_ACTIVE, left_hotkey_is_active, "left-hotkey-activated", "hold");
            }
        }

        if !hotkey_right_str.is_empty() {
            let right_hotkey_is_active = if get_mouse_button_index(&hotkey_right_str).is_some() {
                is_mouse_button_pressed(&current_mouse_buttons, &hotkey_right_str)
            } else {
                hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str)
            };
            let current_right_active_in_zustand = self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);
            if right_hotkey_is_active != current_right_active_in_zustand {
                self.update_hotkey_state(temp_keys::HOTKEY_RIGHT_ACTIVE, right_hotkey_is_active, "right-hotkey-activated", "hold");
            }
        }
    }

    fn handle_toggle_mode(
        &self,
        current_keys: &Vec<Keycode>,
        current_mouse_buttons: &Vec<bool>,
        previous_keys: &mut Vec<Keycode>,
        previous_mouse_buttons: &mut Vec<bool>
    ) {
        if current_keys != previous_keys || current_mouse_buttons != previous_mouse_buttons {
            let hotkey_left_str = self.hotkey_left.lock().unwrap().clone();
            let hotkey_right_str = self.hotkey_right.lock().unwrap().clone();

            if !hotkey_left_str.is_empty() {
                let mut triggered = false;
                if get_mouse_button_index(&hotkey_left_str).is_some() {
                    if was_mouse_button_just_pressed(current_mouse_buttons, previous_mouse_buttons, &hotkey_left_str) {
                        triggered = true;
                    }
                } else {
                    if hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str) &&
                       !hotkey_utils::check_hotkey(previous_keys, &hotkey_left_str) {
                        triggered = true;
                    }
                }
                if triggered {
                    let current_left_active = self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);
                    self.update_hotkey_state(temp_keys::HOTKEY_LEFT_ACTIVE, !current_left_active, "left-hotkey-activated", "toggle");
                }
            }

            if !hotkey_right_str.is_empty() {
                let mut triggered = false;
                if get_mouse_button_index(&hotkey_right_str).is_some() {
                    if was_mouse_button_just_pressed(current_mouse_buttons, previous_mouse_buttons, &hotkey_right_str) {
                        triggered = true;
                    }
                } else {
                    if hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str) &&
                       !hotkey_utils::check_hotkey(previous_keys, &hotkey_right_str) {
                        triggered = true;
                    }
                }
                if triggered {
                    let current_right_active = self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);
                    self.update_hotkey_state(temp_keys::HOTKEY_RIGHT_ACTIVE, !current_right_active, "right-hotkey-activated", "toggle");
                }
            }
            *previous_keys = current_keys.clone();
            *previous_mouse_buttons = current_mouse_buttons.clone();
        }
    }
    
    fn reset_hold_mode_hotkeys(&self) {
        if self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false) {
            self.update_hotkey_state(temp_keys::HOTKEY_LEFT_ACTIVE, false, "left-hotkey-activated", "reset (hold)");
        }
        if self.app_handle.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false) {
            self.update_hotkey_state(temp_keys::HOTKEY_RIGHT_ACTIVE, false, "right-hotkey-activated", "reset (hold)");
        }
    }

    fn process_hotkeys_loop(&self) {
        let device_state = DeviceState::new();
        let mut previous_keys = device_state.get_keys(); 
        let mut previous_mouse_buttons = device_state.get_mouse().button_pressed;

        loop {
            let is_running_val = *self.is_running.lock().unwrap();

            if is_running_val {
                let current_keys = device_state.get_keys();
                let current_mouse_buttons = device_state.get_mouse().button_pressed;
                let hold_mode_val = *self.hold_mode.lock().unwrap();

                if hold_mode_val {
                    self.handle_hold_mode(&current_keys, &current_mouse_buttons);
                } else {
                    self.handle_toggle_mode(&current_keys, &current_mouse_buttons, &mut previous_keys, &mut previous_mouse_buttons);
                }
                thread::sleep(Duration::from_millis(50));
            } else {
                if *self.hold_mode.lock().unwrap() {
                    self.reset_hold_mode_hotkeys();
                }
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

fn handle_hotkeys(app_handle_hotkey: tauri::AppHandle) {
    let manager = HotkeyManager::new(app_handle_hotkey);
    manager.start();
}

fn handle_clicking(app_handle_clicker: tauri::AppHandle) {
    thread::spawn(move || {

        let is_running_arc = Arc::new(Mutex::new(false));
        let left_active_arc = Arc::new(Mutex::new(false));
        let right_active_arc = Arc::new(Mutex::new(false));

        let is_running_clone = Arc::clone(&is_running_arc);
        let left_active_clone = Arc::clone(&left_active_arc);
        let right_active_clone = Arc::clone(&right_active_arc);

        let _ = app_handle_clicker.zustand().watch(store::TEMP, move |app| {
            let new_is_running = app
              .zustand()
              .try_get::<bool>(store::TEMP, temp_keys::IS_RUNNING)
              .unwrap_or(false);

            let new_left_active = app
              .zustand()
              .try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE)
              .unwrap_or(false);
              
            let new_right_active = app
              .zustand()
              .try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE)
              .unwrap_or(false);
            
            let mut is_running_lock = is_running_clone.lock().unwrap();
            *is_running_lock = new_is_running;

            let mut left_active_lock = left_active_clone.lock().unwrap();
            *left_active_lock = new_left_active;

            let mut right_active_lock = right_active_clone.lock().unwrap();
            *right_active_lock = new_right_active;
                            
            Ok(())
          });
        
        let speed_ms_arc = Arc::new(Mutex::new(app_handle_clicker.zustand().try_get::<f64>(store::AUTOCLICKER, autoclicker_keys::CLICK_SPEED).unwrap_or(100.0)));
        let speed_ms_clone = Arc::clone(&speed_ms_arc);

        let _ = app_handle_clicker.zustand().watch(store::AUTOCLICKER, move |app| {
            let new_speed_ms = app
              .zustand()
              .try_get::<f64>(store::AUTOCLICKER, autoclicker_keys::CLICK_SPEED)
              .unwrap_or(100.0);

            let mut speed_ms_lock = speed_ms_clone.lock().unwrap();
            *speed_ms_lock = new_speed_ms;

            Ok(())
          });

        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        #[cfg(target_os = "linux")]
        enigo.set_delay(0);

        loop {
            let (speed_ms, is_running, left_active, right_active) = {
                let speed = *speed_ms_arc.lock().expect("Failed to lock speed mutex");
                let running = *is_running_arc.lock().expect("Failed to lock running mutex"); 
                let left = *left_active_arc.lock().expect("Failed to lock left active mutex");
                let right = *right_active_arc.lock().expect("Failed to lock right active mutex");
                (speed, running, left, right)
            };

            let sleep_duration = Duration::from_micros((speed_ms * 1000.0) as u64);

            if !is_running {
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            if left_active {
                if let Err(e) = enigo.button(Button::Left, Click) {
                    eprintln!("Failed to perform left click: {}", e);
                }
            }

            if right_active {
                if let Err(e) = enigo.button(Button::Right, Click) {
                    eprintln!("Failed to perform right click: {}", e);
                }
            }

            let sleep_time = if left_active || right_active {
                sleep_duration
            } else {
                Duration::from_millis(50)
            };
            thread::sleep(sleep_time);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            handle_hotkeys(app_handle.clone());
            handle_clicking(app_handle.clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_zustand::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}