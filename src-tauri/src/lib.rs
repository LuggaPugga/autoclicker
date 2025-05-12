use tauri::{Emitter, AppHandle};
use device_query::{DeviceQuery, DeviceState};
use std::thread;
use std::time::Duration;
use enigo::{Enigo, Mouse, Settings, Button, Direction::{Click}};
use tauri_plugin_zustand::ManagerExt;

mod hotkey_utils;

#[tauri::command]
fn set_click_speed(speed: f64, app: AppHandle) {
    if let Err(e) = app.zustand().set("store", "clickSpeed", speed) {
        eprintln!("Failed to set clickSpeed in Zustand store: {}", e);
    }
}

#[tauri::command]
fn toggle_is_running(app: AppHandle) {
    let current_value = app.zustand().try_get::<bool>("store", "isRunning").unwrap_or(false);
    if let Err(e) = app.zustand().set("store", "isRunning", !current_value) {
         eprintln!("Failed to set isRunning in Zustand store: {}", e);
    }
}

#[tauri::command]
fn set_hotkey_left(hotkey: String, app: AppHandle) {
    if let Err(e) = app.zustand().set("store", "hotkeyLeft", hotkey) {
        eprintln!("Failed to set hotkeyLeft in Zustand store: {}", e);
    }
}

#[tauri::command]
fn set_hotkey_right(hotkey: String, app: AppHandle) {
    if let Err(e) = app.zustand().set("store", "hotkeyRight", hotkey) {
        eprintln!("Failed to set hotkeyRight in Zustand store: {}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let app_handle_hotkey = app_handle.clone();
            thread::spawn(move || {
                let device_state = DeviceState::new();
                let mut previous_keys = device_state.get_keys();

                loop {
                    let is_running = app_handle_hotkey.zustand().try_get::<bool>("store", "isRunning").unwrap_or(false);

                    if is_running {
                        let current_keys = device_state.get_keys();

                        if current_keys != previous_keys {
                            let hotkey_left = app_handle_hotkey.zustand().try_get::<String>("store", "hotkeyLeft").unwrap_or_default();
                            let hotkey_right = app_handle_hotkey.zustand().try_get::<String>("store", "hotkeyRight").unwrap_or_default();

                            if hotkey_utils::check_hotkey(&current_keys, &hotkey_left) {
                                let current_left_active = app_handle_hotkey.zustand().try_get::<bool>("store", "leftActive").unwrap_or(false);
                                let new_left_active = !current_left_active;
                                if let Err(e) = app_handle_hotkey.zustand().set("store", "leftActive", new_left_active) {
                                    eprintln!("Failed to set leftActive in Zustand store: {}", e);
                                }
                                app_handle_hotkey.emit("left-hotkey-activated", new_left_active).unwrap();
                            }
                            if hotkey_utils::check_hotkey(&current_keys, &hotkey_right) {
                                let current_right_active = app_handle_hotkey.zustand().try_get::<bool>("store", "rightActive").unwrap_or(false);
                                let new_right_active = !current_right_active;
                                if let Err(e) = app_handle_hotkey.zustand().set("store", "rightActive", new_right_active) {
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
                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                loop {
                    let is_running = app_handle_clicker.zustand().try_get::<bool>("store", "isRunning").unwrap_or(false);
                    let left_active = app_handle_clicker.zustand().try_get::<bool>("store", "leftActive").unwrap_or(false);
                    let right_active = app_handle_clicker.zustand().try_get::<bool>("store", "rightActive").unwrap_or(false);
                    let speed_ms = app_handle_clicker.zustand().try_get::<f64>("store", "clickSpeed").unwrap_or(100.0);

                    let sleep_duration = Duration::from_millis(speed_ms.max(1.0) as u64);

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
        .invoke_handler(tauri::generate_handler![
            set_click_speed,
            toggle_is_running,
            set_hotkey_left,
            set_hotkey_right
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}