use gpui::*;
use gpui_component::{label::Label, v_flex, ActiveTheme};

pub struct WaylandWarning;

impl WaylandWarning {
    pub fn new() -> Self {
        Self
    }
}

impl Render for WaylandWarning {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .px_8()
            .py_6()
            .child(
                v_flex()
                    .gap_6()
                    .max_w(px(480.0))
                    .child(
                        v_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .size(px(56.0))
                                    .rounded(px(16.0))
                                    .bg(hsla(220.0 / 360.0, 0.85, 0.55, 0.12))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_2xl()
                                    .child("⚙️"),
                            )
                            .child(
                                Label::new("Setup Required")
                                    .text_2xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.foreground),
                            )
                            .child(
                                Label::new("Grant input device access to continue")
                                    .text_sm()
                                    .text_color(theme.muted_foreground),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_4()
                            .p_5()
                            .rounded(px(12.0))
                            .bg(theme.background)
                            .border_1()
                            .border_color(theme.border.opacity(0.6))
                            .child(
                                v_flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .size(px(20.0))
                                                    .rounded_full()
                                                    .bg(hsla(220.0 / 360.0, 0.85, 0.55, 0.15))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        Label::new("1")
                                                            .text_xs()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(hsla(
                                                                220.0 / 360.0,
                                                                0.85,
                                                                0.55,
                                                                1.0,
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                Label::new("Temporary (until reboot)")
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.foreground),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_2p5()
                                            .rounded(px(8.0))
                                            .bg(theme.secondary)
                                            .border_1()
                                            .border_color(theme.border.opacity(0.4))
                                            .child(
                                                Label::new(
                                                    "sudo setfacl -m u:$USER:r /dev/input/event*",
                                                )
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.foreground),
                                            ),
                                    )
                                    .child(
                                        Label::new("Run this command each time you restart")
                                            .text_xs()
                                            .text_color(theme.muted_foreground.opacity(0.8)),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_4()
                            .p_5()
                            .rounded(px(12.0))
                            .bg(theme.background)
                            .border_1()
                            .border_color(theme.border.opacity(0.6))
                            .child(
                                v_flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .size(px(20.0))
                                                    .rounded_full()
                                                    .bg(hsla(142.0 / 360.0, 0.71, 0.45, 0.12))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        Label::new("2")
                                                            .text_xs()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(hsla(
                                                                142.0 / 360.0,
                                                                0.71,
                                                                0.45,
                                                                1.0,
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                Label::new("Permanent (recommended)")
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.foreground),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_2p5()
                                            .rounded(px(8.0))
                                            .bg(theme.secondary)
                                            .border_1()
                                            .border_color(theme.border.opacity(0.4))
                                            .child(
                                                Label::new("sudo usermod -aG input $USER")
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.foreground),
                                            ),
                                    )
                                    .child(
                                        Label::new("Log out and log back in after running")
                                            .text_xs()
                                            .text_color(theme.muted_foreground.opacity(0.8)),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .p_3p5()
                            .rounded(px(10.0))
                            .bg(theme.accent.opacity(0.08))
                            .border_1()
                            .border_color(theme.accent.opacity(0.2))
                            .child(
                                Label::new(
                                    "ℹ️  The permanent option grants all applications access to input devices",
                                )
                                .text_xs()
                                .text_color(theme.muted_foreground),
                            ),
                    ),
            )
    }
}
