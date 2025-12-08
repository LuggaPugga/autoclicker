use crate::state::AppState;
use enigo::{Button, Direction::Click, Enigo, Mouse, Settings};
use std::{
    sync::{atomic::Ordering, Arc},
    thread,
    time::Duration,
};

pub fn start_clicker(state: Arc<AppState>) {
    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");

        #[cfg(target_os = "linux")]
        enigo.set_delay(0);

        loop {
            let is_running = state.runtime.is_running.load(Ordering::SeqCst);

            if !is_running {
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            let left_active = state.runtime.hotkey_left_active.load(Ordering::SeqCst);
            let right_active = state.runtime.hotkey_right_active.load(Ordering::SeqCst);
            let speed_ms = state.settings.read().click_speed_ms;

            if left_active {
                let _ = enigo.button(Button::Left, Click);
            }

            if right_active {
                let _ = enigo.button(Button::Right, Click);
            }

            let sleep_time = if left_active || right_active {
                Duration::from_micros((speed_ms * 1000.0) as u64)
            } else {
                Duration::from_millis(50)
            };
            thread::sleep(sleep_time);
        }
    });
}
