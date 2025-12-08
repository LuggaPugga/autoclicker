use crate::clicker;
use crate::hotkey;
use crate::state::{AppState, ThemePreference};
use crate::ui::{CustomTitleBar, HotkeyControl, HotkeyType, SpeedControl};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
    theme::{Theme, ThemeMode},
    v_flex, ActiveTheme, Icon, IconName,
};
use std::sync::{atomic::Ordering, Arc};

pub struct AutoClickerApp {
    state: Arc<AppState>,
    titlebar: Entity<CustomTitleBar>,
    speed_control: Entity<SpeedControl>,
    hotkey_control: Entity<HotkeyControl>,
    show_warning_details: bool,
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
            CustomTitleBar::new(is_running, theme_pref).on_theme_change(move |pref, _, _| {
                state_clone.set_theme(pref);
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

        hotkey::start_hotkey_listener(state.clone());
        clicker::start_clicker(state.clone());

        Self {
            state,
            titlebar,
            speed_control,
            hotkey_control,
            show_warning_details: false,
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
        let hotkeys_available = self.state.runtime.hotkeys_available.load(Ordering::SeqCst);

        let warning_bg = hsla(45.0 / 360.0, 0.93, 0.47, 0.15);
        let warning_border = hsla(45.0 / 360.0, 0.93, 0.47, 0.4);
        let warning_text = hsla(45.0 / 360.0, 0.92, 0.40, 1.0);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(self.titlebar.clone())
            .when(!hotkeys_available, |el| {
                el.child(
                    div()
                        .mx_4()
                        .mt_4()
                        .p_3()
                        .rounded_lg()
                        .bg(warning_bg)
                        .border_1()
                        .border_color(warning_border)
                        .child(
                            v_flex()
                                .gap_2()
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    Icon::new(IconName::TriangleAlert)
                                                        .size_4()
                                                        .text_color(warning_text),
                                                )
                                                .child(
                                                    Label::new("Global hotkeys unavailable")
                                                        .text_sm()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(warning_text),
                                                ),
                                        )
                                        .child(
                                            Button::new("toggle-warning-details")
                                                .ghost()
                                                .compact()
                                                .icon(if self.show_warning_details {
                                                    IconName::ChevronUp
                                                } else {
                                                    IconName::ChevronDown
                                                })
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.show_warning_details = !view.show_warning_details;
                                                    cx.notify();
                                                })),
                                        ),
                                )
                                .when(self.show_warning_details, |el| {
                                    el.child(
                                        v_flex()
                                            .gap_2()
                                            .mt_1()
                                            .child(
                                                Label::new("Cannot access input devices. Run one of these commands:")
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        Label::new("Option 1 - Temporary (current session):")
                                                            .text_xs()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.foreground),
                                                    )
                                                    .child(
                                                        div()
                                                            .px_2()
                                                            .py_1()
                                                            .rounded(px(4.0))
                                                            .bg(theme.secondary)
                                                            .child(
                                                                Label::new("sudo setfacl -m u:$USER:r /dev/input/event*")
                                                                    .text_xs()
                                                                    .text_color(theme.foreground),
                                                            ),
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        Label::new("Option 2 - Permanent (requires logout):")
                                                            .text_xs()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.foreground),
                                                    )
                                                    .child(
                                                        div()
                                                            .px_2()
                                                            .py_1()
                                                            .rounded(px(4.0))
                                                            .bg(theme.secondary)
                                                            .child(
                                                                Label::new("sudo usermod -aG input $USER")
                                                                    .text_xs()
                                                                    .text_color(theme.foreground),
                                                            ),
                                                    ),
                                            )
                                            .child(
                                                Label::new("Note: Option 2 grants access to ALL apps you run.")
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground),
                                            ),
                                    )
                                }),
                        ),
                )
            })
            .child(
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
    }
}
