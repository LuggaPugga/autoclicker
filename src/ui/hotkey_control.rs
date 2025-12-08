use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
    v_flex, ActiveTheme, Icon, IconName, Sizable,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HotkeyType {
    Left,
    Right,
}

pub struct HotkeyControl {
    hotkey_left: SharedString,
    hotkey_right: SharedString,
    hotkey_left_active: bool,
    hotkey_right_active: bool,
    recording: Option<HotkeyType>,
    recorded_hotkey: Option<String>,
    on_hotkey_change: Option<Box<dyn Fn(HotkeyType, String, &mut Window, &mut App) + 'static>>,
    focus_handle: FocusHandle,
}

impl HotkeyControl {
    pub fn new(
        hotkey_left: String,
        hotkey_right: String,
        hotkey_left_active: bool,
        hotkey_right_active: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            hotkey_left: SharedString::from(hotkey_left),
            hotkey_right: SharedString::from(hotkey_right),
            hotkey_left_active,
            hotkey_right_active,
            recording: None,
            recorded_hotkey: None,
            on_hotkey_change: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn on_hotkey_change<F>(mut self, f: F) -> Self
    where
        F: Fn(HotkeyType, String, &mut Window, &mut App) + 'static,
    {
        self.on_hotkey_change = Some(Box::new(f));
        self
    }

    pub fn update_active_states(&mut self, left: bool, right: bool) {
        self.hotkey_left_active = left;
        self.hotkey_right_active = right;
    }

    fn format_keystroke(keystroke: &Keystroke) -> String {
        let mut parts = Vec::new();

        if keystroke.modifiers.control {
            parts.push("Ctrl");
        }
        if keystroke.modifiers.alt {
            parts.push("Alt");
        }
        if keystroke.modifiers.shift {
            parts.push("Shift");
        }
        if keystroke.modifiers.platform {
            parts.push("Meta");
        }

        let key = keystroke.key.as_str();
        let key_display = match key {
            "space" => "Space",
            "enter" => "Enter",
            "tab" => "Tab",
            "backspace" => "Backspace",
            "escape" => "Escape",
            "control" | "alt" | "shift" | "meta" => "",
            s if s.len() == 1 => {
                return if parts.is_empty() {
                    s.to_uppercase()
                } else {
                    format!("{}+{}", parts.join("+"), s.to_uppercase())
                };
            }
            s => s,
        };

        if !key_display.is_empty() {
            parts.push(key_display);
        }

        parts.join("+")
    }

    fn start_recording(&mut self, hotkey_type: HotkeyType, window: &mut Window) {
        self.recording = Some(hotkey_type);
        self.recorded_hotkey = None;
        self.focus_handle.focus(window);
    }

    fn finish_recording(&mut self, window: &mut Window, cx: &mut App) {
        let Some(hotkey_type) = self.recording else {
            return;
        };

        let Some(hotkey) = self.recorded_hotkey.take() else {
            self.recording = None;
            return;
        };

        if hotkey.is_empty() {
            self.recording = None;
            return;
        }

        match hotkey_type {
            HotkeyType::Left => self.hotkey_left = SharedString::from(hotkey.clone()),
            HotkeyType::Right => self.hotkey_right = SharedString::from(hotkey.clone()),
        }

        if let Some(ref callback) = self.on_hotkey_change {
            callback(hotkey_type, hotkey, window, cx);
        }

        self.recording = None;
    }

    fn render_hotkey_button(
        &self,
        hotkey_type: HotkeyType,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_recording = self.recording == Some(hotkey_type);
        let (hotkey, is_active) = match hotkey_type {
            HotkeyType::Left => (self.hotkey_left.clone(), self.hotkey_left_active),
            HotkeyType::Right => (self.hotkey_right.clone(), self.hotkey_right_active),
        };

        let display_text = if is_recording {
            if let Some(ref recorded) = self.recorded_hotkey {
                SharedString::from(recorded.clone())
            } else {
                SharedString::from("Press key...")
            }
        } else if hotkey.is_empty() {
            SharedString::from("Click to set")
        } else {
            hotkey
        };

        let id = match hotkey_type {
            HotkeyType::Left => "hotkey-left-btn",
            HotkeyType::Right => "hotkey-right-btn",
        };

        Button::new(id)
            .small()
            .label(display_text)
            .when(is_recording, |btn| btn.primary())
            .when(!is_recording && is_active, |btn| btn.primary())
            .when(!is_recording && !is_active, |btn| btn.outline())
            .on_click(cx.listener(move |view, _, window, cx| {
                view.start_recording(hotkey_type, window);
                cx.notify();
            }))
    }
}

impl Render for HotkeyControl {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_recording = self.recording.is_some();
        let active_color = hsla(142.0 / 360.0, 0.71, 0.45, 1.0);
        let inactive_color = theme.muted_foreground.opacity(0.3);

        div()
            .w_full()
            .p_3()
            .rounded_md()
            .border_1()
            .border_color(theme.border.opacity(0.3))
            .bg(Hsla::transparent_black())
            .track_focus(&self.focus_handle)
            .when(is_recording, |el| {
                el.on_key_down(cx.listener(|view, event: &KeyDownEvent, _window, cx| {
                    if view.recording.is_none() {
                        return;
                    }

                    let key_str = event.keystroke.key.as_str();

                    if key_str == "escape" {
                        view.recording = None;
                        view.recorded_hotkey = None;
                        cx.notify();
                        return;
                    }

                    let formatted = Self::format_keystroke(&event.keystroke);
                    if !formatted.is_empty() {
                        view.recorded_hotkey = Some(formatted);
                    }
                    cx.notify();
                }))
                .on_key_up(cx.listener(
                    |view, event: &KeyUpEvent, window, cx| {
                        if view.recording.is_none() {
                            return;
                        }

                        let key_str = event.keystroke.key.as_str();
                        let is_modifier = matches!(key_str, "control" | "alt" | "shift" | "meta");

                        if !is_modifier && view.recorded_hotkey.is_some() {
                            view.finish_recording(window, cx);
                        } else if is_modifier && view.recorded_hotkey.is_some() {
                            let hotkey = view.recorded_hotkey.as_ref().unwrap();
                            let is_single_modifier =
                                matches!(hotkey.as_str(), "Ctrl" | "Alt" | "Shift" | "Meta");
                            if is_single_modifier {
                                view.finish_recording(window, cx);
                            }
                        }
                        cx.notify();
                    },
                ))
            })
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        h_flex().mb_3().justify_between().items_center().child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    Icon::new(IconName::Settings)
                                        .size_4()
                                        .text_color(theme.muted_foreground),
                                )
                                .child(Label::new("Hotkeys (Global)").text_sm()),
                        ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .min_h(px(28.0))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(div().size_2().rounded_full().bg(
                                        if self.hotkey_left_active {
                                            active_color
                                        } else {
                                            inactive_color
                                        },
                                    ))
                                    .child(
                                        Label::new("Left click")
                                            .text_xs()
                                            .text_color(theme.muted_foreground),
                                    ),
                            )
                            .child(self.render_hotkey_button(HotkeyType::Left, cx)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .min_h(px(28.0))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(div().size_2().rounded_full().bg(
                                        if self.hotkey_right_active {
                                            active_color
                                        } else {
                                            inactive_color
                                        },
                                    ))
                                    .child(
                                        Label::new("Right click")
                                            .text_xs()
                                            .text_color(theme.muted_foreground),
                                    ),
                            )
                            .child(self.render_hotkey_button(HotkeyType::Right, cx)),
                    )
                    .child(
                        Label::new("Press a key. ESC to cancel. Hotkeys work globally.")
                            .text_xs()
                            .text_color(theme.muted_foreground),
                    ),
            )
    }
}
