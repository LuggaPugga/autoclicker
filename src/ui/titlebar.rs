use crate::state::ThemePreference;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
    theme::{Theme, ThemeMode},
    ActiveTheme, IconName, TitleBar,
};

pub struct CustomTitleBar {
    is_running: bool,
    theme_pref: ThemePreference,
    on_theme_change: Option<Box<dyn Fn(ThemePreference, &mut Window, &mut App) + 'static>>,
}

impl CustomTitleBar {
    pub fn new(is_running: bool, theme_pref: ThemePreference) -> Self {
        Self {
            is_running,
            theme_pref,
            on_theme_change: None,
        }
    }

    pub fn on_theme_change<F>(mut self, f: F) -> Self
    where
        F: Fn(ThemePreference, &mut Window, &mut App) + 'static,
    {
        self.on_theme_change = Some(Box::new(f));
        self
    }

    pub fn set_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }
}

impl Render for CustomTitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let green = hsla(142.0 / 360.0, 0.71, 0.45, 1.0);
        let green_bg = hsla(142.0 / 360.0, 0.71, 0.45, 0.12);
        let green_text = hsla(142.0 / 360.0, 0.65, 0.35, 1.0);

        TitleBar::new()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .size_5()
                            .rounded_md()
                            .bg(hsla(220.0 / 360.0, 0.85, 0.55, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Label::new("⚡").text_xs().text_color(white())),
                    )
                    .child(
                        Label::new("AutoClicker")
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.foreground),
                    ),
            )
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .items_center()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .when(self.is_running, |el| el.bg(green_bg))
                            .child(
                                div()
                                    .size(px(6.0))
                                    .rounded_full()
                                    .when(self.is_running, |el| el.bg(green))
                                    .when(!self.is_running, |el| {
                                        el.bg(theme.muted_foreground.opacity(0.5))
                                    }),
                            )
                            .child(
                                Label::new(if self.is_running { "Active" } else { "Idle" })
                                    .text_xs()
                                    .when(self.is_running, |el| el.text_color(green_text))
                                    .when(!self.is_running, |el| {
                                        el.text_color(theme.muted_foreground)
                                    }),
                            ),
                    )
                    .child(
                        Button::new("theme-toggle")
                            .ghost()
                            .compact()
                            .icon(match self.theme_pref {
                                ThemePreference::System => IconName::Sun,
                                ThemePreference::Light => IconName::Sun,
                                ThemePreference::Dark => IconName::Moon,
                            })
                            .on_click(cx.listener(|view, _, window, cx| {
                                let new_pref = match view.theme_pref {
                                    ThemePreference::System => ThemePreference::Dark,
                                    ThemePreference::Light => ThemePreference::Dark,
                                    ThemePreference::Dark => ThemePreference::Light,
                                };
                                view.theme_pref = new_pref;
                                match new_pref {
                                    ThemePreference::System => {
                                        Theme::sync_system_appearance(Some(window), cx)
                                    }
                                    ThemePreference::Light => {
                                        Theme::change(ThemeMode::Light, Some(window), cx)
                                    }
                                    ThemePreference::Dark => {
                                        Theme::change(ThemeMode::Dark, Some(window), cx)
                                    }
                                }
                                if let Some(ref cb) = view.on_theme_change {
                                    cb(new_pref, window, cx);
                                }
                                cx.notify();
                            })),
                    ),
            )
    }
}
