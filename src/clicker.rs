use crate::state::AppState;
use enigo::{Button, Direction::Click, Enigo, Mouse, Settings};
use std::{
    sync::{atomic::Ordering, Arc},
    thread,
    time::{Duration, Instant},
};

pub fn start_clicker(state: Arc<AppState>) {
    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");

        #[cfg(target_os = "linux")]
        enigo.set_delay(0);

        let mut next_click_time = Instant::now();
        let mut last_speed_ms = 0.0;

        loop {
            let is_running = state.runtime.is_running.load(Ordering::SeqCst);

            if !is_running {
                thread::sleep(Duration::from_millis(200));
                next_click_time = Instant::now();
                continue;
            }

            let left_active = state.runtime.hotkey_left_active.load(Ordering::SeqCst);
            let right_active = state.runtime.hotkey_right_active.load(Ordering::SeqCst);
            let speed_ms = state.settings.read().click_speed_ms;

            if (speed_ms - last_speed_ms).abs() > 0.001 {
                next_click_time = Instant::now();
                last_speed_ms = speed_ms;
            }

            let now = Instant::now();

            if (left_active || right_active) && now >= next_click_time {
                if left_active {
                    let _ = enigo.button(Button::Left, Click);
                }

                if right_active {
                    let _ = enigo.button(Button::Right, Click);
                }

                let interval = Duration::from_secs_f64(speed_ms / 1000.0);
                next_click_time += interval;

                if next_click_time < now {
                    next_click_time = now + interval;
                }
            }

            let sleep_time = if left_active || right_active {
                if now < next_click_time {
                    next_click_time.duration_since(now)
                } else {
                    Duration::from_micros(1)
                }
            } else {
                Duration::from_millis(50)
            };

            thread::sleep(sleep_time);
        }
    });
}
