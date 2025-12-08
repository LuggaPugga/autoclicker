use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputEvent, InputState},
    label::Label,
    slider::{Slider, SliderEvent, SliderState},
    switch::Switch,
    v_flex, ActiveTheme, IconName,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedMode {
    Cps,
    Ms,
}

pub struct SpeedControl {
    mode: SpeedMode,
    slider_state: Entity<SliderState>,
    input_state: Entity<InputState>,
    click_speed_ms: f32,
    last_saved_speed_ms: f32,
    hold_mode: bool,
    on_speed_change: Option<Box<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    on_hold_mode_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl SpeedControl {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        click_speed_ms: f32,
        hold_mode: bool,
    ) -> Self {
        let cps_value = 1000.0 / click_speed_ms;

        let slider_state = cx.new(|_| {
            SliderState::new()
                .min(1.0)
                .max(100.0)
                .default_value(cps_value.min(100.0))
                .step(1.0)
        });
        let input_state =
            cx.new(|cx| InputState::new(window, cx).default_value(&format!("{:.0}", cps_value)));

        cx.subscribe(&slider_state, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            let new_value = value.start();
            this.click_speed_ms = match this.mode {
                SpeedMode::Cps => 1000.0 / new_value,
                SpeedMode::Ms => new_value,
            };
            cx.notify();
        })
        .detach();

        cx.subscribe_in(&input_state, window, |this, state, event, window, cx| {
            if let InputEvent::Change = event {
                if let Ok(val) = state.read(cx).value().parse::<f32>() {
                    let (min, max, slider_max) = match this.mode {
                        SpeedMode::Cps => (1.0, 1000.0, 100.0),
                        SpeedMode::Ms => (1.0, 10000.0, 1000.0),
                    };
                    let clamped = val.clamp(min, max);
                    this.click_speed_ms = match this.mode {
                        SpeedMode::Cps => 1000.0 / clamped,
                        SpeedMode::Ms => clamped,
                    };
                    this.slider_state.update(cx, |s, cx| {
                        s.set_value(clamped.min(slider_max), window, cx);
                    });
                    cx.notify();
                }
            }
        })
        .detach();

        Self {
            mode: SpeedMode::Cps,
            slider_state,
            input_state,
            click_speed_ms,
            last_saved_speed_ms: click_speed_ms,
            hold_mode,
            on_speed_change: None,
            on_hold_mode_change: None,
        }
    }

    pub fn on_speed_change<F: Fn(f32, &mut Window, &mut App) + 'static>(mut self, f: F) -> Self {
        self.on_speed_change = Some(Box::new(f));
        self
    }

    pub fn on_hold_mode_change<F: Fn(bool, &mut Window, &mut App) + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.on_hold_mode_change = Some(Box::new(f));
        self
    }

    fn switch_mode(&mut self, new_mode: SpeedMode, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode == new_mode {
            return;
        }
        self.mode = new_mode;

        let (min, max, value) = match new_mode {
            SpeedMode::Cps => (1.0, 100.0, (1000.0 / self.click_speed_ms).min(100.0)),
            SpeedMode::Ms => (1.0, 1000.0, self.click_speed_ms.min(1000.0)),
        };

        self.slider_state.update(cx, |s, cx| {
            *s = SliderState::new()
                .min(min)
                .max(max)
                .step(1.0)
                .default_value(value);
            s.set_value(value, window, cx);
        });

        self.input_state.update(cx, |s, cx| {
            s.set_value(&format!("{:.0}", value), window, cx);
        });

        cx.notify();
    }
}

impl Render for SpeedControl {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if (self.click_speed_ms - self.last_saved_speed_ms).abs() > 0.001 {
            self.last_saved_speed_ms = self.click_speed_ms;
            if let Some(ref cb) = self.on_speed_change {
                cb(self.click_speed_ms, window, cx);
            }
        }

        let theme = cx.theme();
        let cps = 1000.0 / self.click_speed_ms;
        let mode = self.mode;

        v_flex()
            .w_full()
            .gap_4()
            .child(
                v_flex()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border.opacity(0.5))
                    .bg(theme.secondary)
                    .gap_4()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .child(
                                Button::new("cps-mode")
                                    .flex_1()
                                    .when(mode == SpeedMode::Cps, |b| b.primary())
                                    .when(mode != SpeedMode::Cps, |b| b.outline())
                                    .icon(IconName::ChartPie)
                                    .label("Clicks/sec")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.switch_mode(SpeedMode::Cps, window, cx);
                                    })),
                            )
                            .child(
                                Button::new("ms-mode")
                                    .flex_1()
                                    .when(mode == SpeedMode::Ms, |b| b.primary())
                                    .when(mode != SpeedMode::Ms, |b| b.outline())
                                    .icon(IconName::Calendar)
                                    .label("Milliseconds")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.switch_mode(SpeedMode::Ms, window, cx);
                                    })),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_3()
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        Label::new(match mode {
                                            SpeedMode::Cps => "Clicks per second",
                                            SpeedMode::Ms => "Milliseconds between clicks",
                                        })
                                        .text_sm(),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded(px(4.0))
                                            .bg(theme.muted.opacity(0.3))
                                            .text_xs()
                                            .text_color(theme.muted_foreground)
                                            .child(format!(
                                                "{:.0} CPS • {:.0}ms",
                                                cps, self.click_speed_ms
                                            )),
                                    ),
                            )
                            .child(Slider::new(&self.slider_state))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Input::new(&self.input_state).w(px(80.0)))
                                    .child(
                                        Label::new(match mode {
                                            SpeedMode::Cps => "clicks/sec",
                                            SpeedMode::Ms => "ms",
                                        })
                                        .text_sm()
                                        .text_color(theme.muted_foreground),
                                    ),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border.opacity(0.3))
                    .bg(theme.background.opacity(0.5))
                    .w_full()
                    .justify_between()
                    .items_center()
                    .gap_3()
                    .child(
                        v_flex()
                            .flex_1()
                            .child(Label::new("Toggle vs Hold").text_sm())
                            .child(
                                Label::new(if self.hold_mode {
                                    "Clicks while hotkey is held down"
                                } else {
                                    "Press hotkey to start/stop clicking"
                                })
                                .text_xs()
                                .text_color(theme.muted_foreground),
                            ),
                    )
                    .child(
                        Switch::new("hold-mode")
                            .checked(self.hold_mode)
                            .on_click(cx.listener(|this, checked, window, cx| {
                                this.hold_mode = *checked;
                                if let Some(ref cb) = this.on_hold_mode_change {
                                    cb(*checked, window, cx);
                                }
                                cx.notify();
                            })),
                    ),
            )
    }
}
