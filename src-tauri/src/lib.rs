use tauri::{Emitter};
use device_query::{DeviceQuery, DeviceState};
use std::thread;
use std::time::Duration;
use enigo::{Enigo, Mouse, Settings, Button, Direction::{Click}};
use tauri_plugin_zustand::ManagerExt;
use std::sync::{Arc, Mutex};

mod hotkey_utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let app_handle_hotkey = app_handle.clone();
            thread::spawn(move || {
                let device_state = DeviceState::new();
                let mut previous_keys = device_state.get_keys();

                let initial_is_running = app_handle_hotkey.zustand().try_get::<bool>("temp", "isRunning").unwrap_or(false);
                let initial_hotkey_left = app_handle_hotkey.zustand().try_get::<String>("autoclicker", "hotkeyLeft").unwrap_or_default();
                let initial_hotkey_right = app_handle_hotkey.zustand().try_get::<String>("autoclicker", "hotkeyRight").unwrap_or_default();

                let is_running_arc = Arc::new(Mutex::new(initial_is_running));
                let hotkey_left_arc = Arc::new(Mutex::new(initial_hotkey_left));
                let hotkey_right_arc = Arc::new(Mutex::new(initial_hotkey_right));

                let is_running_watch_clone = Arc::clone(&is_running_arc);
                let _ = app_handle_hotkey.zustand().watch("temp", move |app| {
                    if let Ok(new_val) = app.zustand().try_get::<bool>("temp", "isRunning") {
                        *is_running_watch_clone.lock().unwrap() = new_val;
                    }
                    Ok(())
                });

                let hotkey_left_watch_clone = Arc::clone(&hotkey_left_arc);
                let hotkey_right_watch_clone = Arc::clone(&hotkey_right_arc);
                let _ = app_handle_hotkey.zustand().watch("autoclicker", move |app| {
                    if let Ok(new_val) = app.zustand().try_get::<String>("autoclicker", "hotkeyLeft") {
                        *hotkey_left_watch_clone.lock().unwrap() = new_val;
                    }
                    if let Ok(new_val) = app.zustand().try_get::<String>("autoclicker", "hotkeyRight") {
                        *hotkey_right_watch_clone.lock().unwrap() = new_val;
                    }
                    Ok(())
                });

                loop {
                    let is_running = *is_running_arc.lock().unwrap();

                    if is_running {
                        let current_keys = device_state.get_keys();

                        if current_keys != previous_keys {
                            let hotkey_left = hotkey_left_arc.lock().unwrap().clone();
                            let hotkey_right = hotkey_right_arc.lock().unwrap().clone();

                            if hotkey_utils::check_hotkey(&current_keys, &hotkey_left) {
                                let current_left_active = app_handle_hotkey.zustand().try_get::<bool>("temp", "hotkeyLeftActive").unwrap_or(false);
                                let new_left_active = !current_left_active;
                                if let Err(e) = app_handle_hotkey.zustand().set("temp", "hotkeyLeftActive", new_left_active) {
                                    eprintln!("Failed to set leftActive in Zustand store: {}", e);
                                }
                                app_handle_hotkey.emit("left-hotkey-activated", new_left_active).unwrap();
                            }
                            if hotkey_utils::check_hotkey(&current_keys, &hotkey_right) {
                                let current_right_active = app_handle_hotkey.zustand().try_get::<bool>("temp", "hotkeyRightActive").unwrap_or(false);
                                let new_right_active = !current_right_active;
                                if let Err(e) = app_handle_hotkey.zustand().set("temp", "hotkeyRightActive", new_right_active) {
                                    eprintln!("Failed to set rightActive in Zustand store: {}", e);
                                }
                                app_handle_hotkey.emit("right-hotkey-activated", new_right_active).unwrap();
                            }

                            previous_keys = current_keys;
                        }

                        thread::sleep(Duration::from_millis(50));
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                }
            });

            let app_handle_clicker = app_handle.clone();
            thread::spawn(move || {

                let is_running_arc = Arc::new(Mutex::new(false));
                let left_active_arc = Arc::new(Mutex::new(false));
                let right_active_arc = Arc::new(Mutex::new(false));

                let is_running_clone = Arc::clone(&is_running_arc);
                let left_active_clone = Arc::clone(&left_active_arc);
                let right_active_clone = Arc::clone(&right_active_arc);

                let _ = app_handle_clicker.zustand().watch("temp", move |app| {
                    let new_is_running = app
                      .zustand()
                      .try_get::<bool>("temp", "isRunning")
                      .unwrap_or(false);

                    let new_left_active = app
                      .zustand()
                      .try_get::<bool>("temp", "hotkeyLeftActive")
                      .unwrap_or(false);
                      
                    let new_right_active = app
                      .zustand()
                      .try_get::<bool>("temp", "hotkeyRightActive")
                      .unwrap_or(false);
                    
                    let mut is_running_lock = is_running_clone.lock().unwrap();
                    *is_running_lock = new_is_running;

                    let mut left_active_lock = left_active_clone.lock().unwrap();
                    *left_active_lock = new_left_active;

                    let mut right_active_lock = right_active_clone.lock().unwrap();
                    *right_active_lock = new_right_active;
                                    
                    Ok(())
                  });
                
                let speed_ms_arc = Arc::new(Mutex::new(app_handle_clicker.zustand().try_get::<f64>("autoclicker", "clickSpeed").unwrap_or(100.0)));
                let speed_ms_clone = Arc::clone(&speed_ms_arc);

                let _ = app_handle_clicker.zustand().watch("autoclicker", move |app| {
                    let new_speed_ms = app
                      .zustand()
                      .try_get::<f64>("autoclicker", "clickSpeed")
                      .unwrap_or(100.0);

                    let mut speed_ms_lock = speed_ms_clone.lock().unwrap();
                    *speed_ms_lock = new_speed_ms;

                    Ok(())
                  });

                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                loop {
                    let speed_ms = *speed_ms_arc.lock().unwrap();
                    let sleep_duration = Duration::from_millis(speed_ms.max(1.0) as u64);

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

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_zustand::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}