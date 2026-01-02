use crate::clicker;
use crate::hotkey;
use crate::state::{AppState, ThemePreference};
use crate::ui::{CustomTitleBar, HotkeyControl, HotkeyType, SpeedControl, WaylandWarning};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    theme::{Theme, ThemeMode},
    ActiveTheme, IconName,
};
use std::sync::{atomic::Ordering, Arc};

pub struct AutoClickerApp {
    pub state: Arc<AppState>,
    titlebar: Entity<CustomTitleBar>,
    speed_control: Entity<SpeedControl>,
    hotkey_control: Entity<HotkeyControl>,
    wayland_warning: Entity<WaylandWarning>,
}

impl AutoClickerApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = AppState::new();
        let settings = state.settings.read();
        let click_speed_ms = settings.click_speed_ms as f32;
        let hold_mode = settings.hold_mode;
        let hotkey_left = settings.hotkey_left.clone();
        let hotkey_right = settings.hotkey_right.clone();
        let theme_pref = settings.theme;
        drop(settings);

        match theme_pref {
            ThemePreference::System => Theme::sync_system_appearance(Some(window), cx),
            ThemePreference::Light => Theme::change(ThemeMode::Light, Some(window), cx),
            ThemePreference::Dark => Theme::change(ThemeMode::Dark, Some(window), cx),
        }

        let is_running = state.runtime.is_running.load(Ordering::SeqCst);
        let left_active = state.runtime.hotkey_left_active.load(Ordering::SeqCst);
        let right_active = state.runtime.hotkey_right_active.load(Ordering::SeqCst);

        let state_clone = state.clone();
        let titlebar = cx.new(|_| {
            CustomTitleBar::new(is_running, theme_pref).on_theme_change(move |pref, window, cx| {
                state_clone.set_theme(pref);
                match pref {
                    ThemePreference::System => {
                        Theme::sync_system_appearance(Some(window), cx);
                    }
                    ThemePreference::Light => {
                        Theme::change(ThemeMode::Light, Some(window), cx);
                    }
                    ThemePreference::Dark => {
                        Theme::change(ThemeMode::Dark, Some(window), cx);
                    }
                }
            })
        });

        let state_clone = state.clone();
        let state_clone2 = state.clone();
        let speed_control = cx.new(|cx| {
            SpeedControl::new(window, cx, click_speed_ms, hold_mode)
                .on_speed_change(move |speed, _, _| {
                    state_clone.set_click_speed(speed as f64);
                })
                .on_hold_mode_change(move |enabled, _, _| {
                    state_clone2.set_hold_mode(enabled);
                })
        });

        let state_clone = state.clone();
        let hotkey_control = cx.new(|cx| {
            HotkeyControl::new(hotkey_left, hotkey_right, left_active, right_active, cx)
                .on_hotkey_change(move |hotkey_type, key, _, _| match hotkey_type {
                    HotkeyType::Left => state_clone.set_hotkey_left(key),
                    HotkeyType::Right => state_clone.set_hotkey_right(key),
                })
        });

        let wayland_warning = cx.new(|_| WaylandWarning::new());

        hotkey::start_hotkey_listener(state.clone());
        clicker::start_clicker(state.clone());

        Self {
            state,
            titlebar,
            speed_control,
            hotkey_control,
            wayland_warning,
        }
    }

    fn update_ui_state(&mut self, cx: &mut Context<Self>) {
        let is_running = self.state.runtime.is_running.load(Ordering::SeqCst);
        let left_active = self.state.runtime.hotkey_left_active.load(Ordering::SeqCst);
        let right_active = self
            .state
            .runtime
            .hotkey_right_active
            .load(Ordering::SeqCst);

        self.titlebar.update(cx, |titlebar, _| {
            titlebar.set_running(is_running);
        });

        self.hotkey_control.update(cx, |control, _| {
            control.update_active_states(left_active, right_active);
        });

        cx.notify();
    }

    fn toggle_running(&mut self, cx: &mut Context<Self>) {
        self.state.toggle_running();
        self.update_ui_state(cx);
    }
}

impl Render for AutoClickerApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_running = self.state.runtime.is_running.load(Ordering::SeqCst);
        let force_show = self.state.runtime.force_show_warning.load(Ordering::SeqCst);
        let hotkeys_available =
            self.state.runtime.hotkeys_available.load(Ordering::SeqCst) && !force_show;

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(self.titlebar.clone())
            .when(!hotkeys_available, |el| {
                el.child(self.wayland_warning.clone())
            })
            .when(hotkeys_available, |el| {
                el.child(
                    div()
                        .flex_1()
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(self.speed_control.clone())
                        .child(self.hotkey_control.clone()),
                )
                .child(
                    div().p_4().child(
                        Button::new("toggle-running")
                            .w_full()
                            .when(is_running, |btn| btn.danger())
                            .when(!is_running, |btn| btn.primary())
                            .icon(if is_running {
                                IconName::CircleX
                            } else {
                                IconName::CircleCheck
                            })
                            .label(if is_running {
                                "Stop Listening"
                            } else {
                                "Start Listening"
                            })
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.toggle_running(cx);
                            })),
                    ),
                )
            })
    }
}
