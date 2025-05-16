use tauri::{Emitter};
use device_query::{DeviceQuery, DeviceState};
use std::thread;
use std::time::Duration;
use enigo::{Enigo, Mouse, Settings, Button, Direction::{Click}};
use tauri_plugin_zustand::ManagerExt;
use std::sync::{Arc, Mutex};

mod hotkey_utils;
mod zustand_keys;

use crate::zustand_keys::{autoclicker_keys, store, temp_keys};

// Helper function to parse "MouseButtonX" and check press state
fn is_mouse_button_pressed(mouse_buttons: &Vec<bool>, hotkey_str: &str) -> bool {
    match hotkey_str {
        "MouseButton4" => mouse_buttons.get(4).cloned().unwrap_or(false), // Index 4 for Button 4
        "MouseButton5" => mouse_buttons.get(5).cloned().unwrap_or(false), // Index 5 for Button 5
        _ => false,
    }
}

// Helper function for toggle mode to check if a mouse button was just pressed
fn was_mouse_button_just_pressed(current_buttons: &Vec<bool>, previous_buttons: &Vec<bool>, hotkey_str: &str) -> bool {
    let button_index = match hotkey_str {
        "MouseButton4" => Some(4),
        "MouseButton5" => Some(5),
        _ => None,
    };

    if let Some(index) = button_index {
        let current_pressed = current_buttons.get(index).cloned().unwrap_or(false);
        let previous_pressed = previous_buttons.get(index).cloned().unwrap_or(false);
        current_pressed && !previous_pressed
    } else {
        false
    }
}

fn handle_hotkeys(app_handle_hotkey: tauri::AppHandle) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut previous_keys = device_state.get_keys();
        let mut previous_mouse_buttons = device_state.get_mouse().button_pressed;

        let initial_is_running = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::IS_RUNNING).unwrap_or(false);
        let initial_hotkey_left = app_handle_hotkey.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_LEFT).unwrap_or_default();
        let initial_hotkey_right = app_handle_hotkey.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_RIGHT).unwrap_or_default();
        let initial_hold_mode = app_handle_hotkey.zustand().try_get::<bool>(store::AUTOCLICKER, autoclicker_keys::HOLD_MODE).unwrap_or(false);

        let is_running_arc = Arc::new(Mutex::new(initial_is_running));
        let hotkey_left_arc = Arc::new(Mutex::new(initial_hotkey_left));
        let hotkey_right_arc = Arc::new(Mutex::new(initial_hotkey_right));
        let hold_mode_arc = Arc::new(Mutex::new(initial_hold_mode));

        let is_running_watch_clone = Arc::clone(&is_running_arc);
        let _ = app_handle_hotkey.zustand().watch(store::TEMP, move |app| {
            if let Ok(new_val) = app.zustand().try_get::<bool>(store::TEMP, temp_keys::IS_RUNNING) {
                *is_running_watch_clone.lock().unwrap() = new_val;
            }
            Ok(())
        });

        let hotkey_left_watch_clone = Arc::clone(&hotkey_left_arc);
        let hotkey_right_watch_clone = Arc::clone(&hotkey_right_arc);
        let hold_mode_watch_clone = Arc::clone(&hold_mode_arc);
        let _ = app_handle_hotkey.zustand().watch(store::AUTOCLICKER, move |app| {
            if let Ok(new_val) = app.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_LEFT) {
                *hotkey_left_watch_clone.lock().unwrap() = new_val;
            }
            if let Ok(new_val) = app.zustand().try_get::<String>(store::AUTOCLICKER, autoclicker_keys::HOTKEY_RIGHT) {
                *hotkey_right_watch_clone.lock().unwrap() = new_val;
            }
            if let Ok(new_val) = app.zustand().try_get::<bool>(store::AUTOCLICKER, autoclicker_keys::HOLD_MODE) {
                *hold_mode_watch_clone.lock().unwrap() = new_val;
            }
            Ok(())
        });

        loop {
            let is_running = *is_running_arc.lock().unwrap();

            if is_running {
                let current_keys = device_state.get_keys();
                let current_mouse_buttons = device_state.get_mouse().button_pressed;

                let hotkey_left_str = hotkey_left_arc.lock().unwrap().clone();
                let hotkey_right_str = hotkey_right_arc.lock().unwrap().clone();
                let hold_mode = *hold_mode_arc.lock().unwrap();

                if hold_mode {
                    let left_hotkey_is_active = if !hotkey_left_str.is_empty() {
                        if hotkey_left_str.starts_with("MouseButton") {
                            is_mouse_button_pressed(&current_mouse_buttons, &hotkey_left_str)
                        } else {
                            hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str)
                        }
                    } else {
                        false
                    };
                    
                    let current_left_active_in_zustand = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);

                    if left_hotkey_is_active != current_left_active_in_zustand {
                        if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE, left_hotkey_is_active) {
                            eprintln!("Failed to set leftActive (hold) in Zustand store: {}", e);
                        }
                        app_handle_hotkey.emit("left-hotkey-activated", left_hotkey_is_active).unwrap_or_else(|e| {
                            eprintln!("Failed to emit left-hotkey-activated (hold): {}", e);
                        });
                    }

                    let right_hotkey_is_active = if !hotkey_right_str.is_empty() {
                        if hotkey_right_str.starts_with("MouseButton") {
                            is_mouse_button_pressed(&current_mouse_buttons, &hotkey_right_str)
                        } else {
                            hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str)
                        }
                    } else {
                        false
                    };
                    let current_right_active_in_zustand = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);

                    if right_hotkey_is_active != current_right_active_in_zustand {
                        if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE, right_hotkey_is_active) {
                            eprintln!("Failed to set rightActive (hold) in Zustand store: {}", e);
                        }
                        app_handle_hotkey.emit("right-hotkey-activated", right_hotkey_is_active).unwrap_or_else(|e| {
                            eprintln!("Failed to emit right-hotkey-activated (hold): {}", e);
                        });
                    }
                } else { // Toggle mode
                    if current_keys != previous_keys || current_mouse_buttons != previous_mouse_buttons {
                        if !hotkey_left_str.is_empty() {
                            let mut triggered = false;
                            if hotkey_left_str.starts_with("MouseButton") {
                                if was_mouse_button_just_pressed(&current_mouse_buttons, &previous_mouse_buttons, &hotkey_left_str) {
                                    triggered = true;
                                }
                            } else {
                                // Keyboard hotkey: check if it just became active
                                if hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str) && 
                                   !hotkey_utils::check_hotkey(&previous_keys, &hotkey_left_str) {
                                    triggered = true;
                                }
                            }

                            if triggered {
                                let current_left_active = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);
                                let new_left_active = !current_left_active;
                                if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE, new_left_active) {
                                    eprintln!("Failed to set leftActive (toggle) in Zustand store: {}", e);
                                }
                                app_handle_hotkey.emit("left-hotkey-activated", new_left_active).unwrap_or_else(|e| {
                                    eprintln!("Failed to emit left-hotkey-activated (toggle): {}", e);
                                });
                            }
                        }

                        if !hotkey_right_str.is_empty() {
                            let mut triggered = false;
                            if hotkey_right_str.starts_with("MouseButton") {
                                if was_mouse_button_just_pressed(&current_mouse_buttons, &previous_mouse_buttons, &hotkey_right_str) {
                                    triggered = true;
                                }
                            } else {
                                // Keyboard hotkey: check if it just became active
                                if hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str) && 
                                   !hotkey_utils::check_hotkey(&previous_keys, &hotkey_right_str) {
                                    triggered = true;
                                }
                            }

                            if triggered {
                                let current_right_active = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);
                                let new_right_active = !current_right_active;
                                if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE, new_right_active) {
                                    eprintln!("Failed to set rightActive (toggle) in Zustand store: {}", e);
                                }
                                app_handle_hotkey.emit("right-hotkey-activated", new_right_active).unwrap_or_else(|e| {
                                    eprintln!("Failed to emit right-hotkey-activated (toggle): {}", e);
                                });
                            }
                        }
                        previous_keys = current_keys;
                        previous_mouse_buttons = current_mouse_buttons.clone();
                    }
                }
                thread::sleep(Duration::from_millis(50));
            } else {
                let hold_mode = *hold_mode_arc.lock().unwrap();
                if hold_mode {
                    if app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false) {
                        if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE, false) {
                            eprintln!("Failed to reset leftActive (is_running false, hold mode): {}", e);
                        } else {
                            app_handle_hotkey.emit("left-hotkey-activated", false).unwrap_or_else(|e| {
                                eprintln!("Failed to emit reset left-hotkey-activated (is_running false, hold mode): {}", e);
                            });
                        }
                    }
                    if app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false) {
                         if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE, false) {
                            eprintln!("Failed to reset rightActive (is_running false, hold mode): {}", e);
                        } else {
                            app_handle_hotkey.emit("right-hotkey-activated", false).unwrap_or_else(|e| {
                                eprintln!("Failed to emit reset right-hotkey-activated (is_running false, hold mode): {}", e);
                            });
                        }
                    }
                }
                thread::sleep(Duration::from_millis(200));
            }
        }
    });
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