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

fn handle_hotkeys(app_handle_hotkey: tauri::AppHandle) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut previous_keys = device_state.get_keys();

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
                let hotkey_left_str = hotkey_left_arc.lock().unwrap().clone();
                let hotkey_right_str = hotkey_right_arc.lock().unwrap().clone();
                let hold_mode = *hold_mode_arc.lock().unwrap();

                if hold_mode {
                    let left_hotkey_is_pressed = hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str);
                    let current_left_active_in_zustand = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);

                    if left_hotkey_is_pressed != current_left_active_in_zustand {
                        if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE, left_hotkey_is_pressed) {
                            eprintln!("Failed to set leftActive (hold) in Zustand store: {}", e);
                        }
                        app_handle_hotkey.emit("left-hotkey-activated", left_hotkey_is_pressed).unwrap_or_else(|e| {
                            eprintln!("Failed to emit left-hotkey-activated (hold): {}", e);
                        });
                    }

                    let right_hotkey_is_pressed = hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str);
                    let current_right_active_in_zustand = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);

                    if right_hotkey_is_pressed != current_right_active_in_zustand {
                        if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE, right_hotkey_is_pressed) {
                            eprintln!("Failed to set rightActive (hold) in Zustand store: {}", e);
                        }
                        app_handle_hotkey.emit("right-hotkey-activated", right_hotkey_is_pressed).unwrap_or_else(|e| {
                            eprintln!("Failed to emit right-hotkey-activated (hold): {}", e);
                        });
                    }
                } else {
                    if current_keys != previous_keys {
                        if !hotkey_left_str.is_empty() && hotkey_utils::check_hotkey(&current_keys, &hotkey_left_str) {
                            let current_left_active = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE).unwrap_or(false);
                            let new_left_active = !current_left_active;
                            if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_LEFT_ACTIVE, new_left_active) {
                                eprintln!("Failed to set leftActive (toggle) in Zustand store: {}", e);
                            }
                            app_handle_hotkey.emit("left-hotkey-activated", new_left_active).unwrap_or_else(|e| {
                                eprintln!("Failed to emit left-hotkey-activated (toggle): {}", e);
                            });
                        }
                        if !hotkey_right_str.is_empty() && hotkey_utils::check_hotkey(&current_keys, &hotkey_right_str) {
                            let current_right_active = app_handle_hotkey.zustand().try_get::<bool>(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE).unwrap_or(false);
                            let new_right_active = !current_right_active;
                            if let Err(e) = app_handle_hotkey.zustand().set(store::TEMP, temp_keys::HOTKEY_RIGHT_ACTIVE, new_right_active) {
                                eprintln!("Failed to set rightActive (toggle) in Zustand store: {}", e);
                            }
                            app_handle_hotkey.emit("right-hotkey-activated", new_right_active).unwrap_or_else(|e| {
                                eprintln!("Failed to emit right-hotkey-activated (toggle): {}", e);
                            });
                        }
                        previous_keys = current_keys;
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

          let settings = Settings {
            linux_delay: 0,
            ..Default::default()
        };
        let mut enigo = Enigo::new(&settings).unwrap();
        enigo.set_delay(0);

        loop {
            let speed_ms = *speed_ms_arc.lock().unwrap();
            let sleep_duration = Duration::from_micros((speed_ms * 1000.0) as u64);

            let is_running = *is_running_arc.lock().unwrap();
            let left_active = *left_active_arc.lock().unwrap();
            let right_active = *right_active_arc.lock().unwrap();

            if is_running {
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
                if left_active || right_active {
                    thread::sleep(sleep_duration);
                } else {
                    thread::sleep(Duration::from_millis(50));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
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