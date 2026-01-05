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
        let yellow = hsla(45.0 / 360.0, 0.9, 0.55, 1.0);

        v_flex()
            .size_full()
            .p_6()
            .gap_6()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .size(px(48.0))
                            .rounded(px(10.0))
                            .border_1()
                            .border_color(yellow.opacity(0.4))
                            .bg(yellow.opacity(0.1))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Label::new("!")
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(yellow),
                            ),
                    )
                    .child(
                        Label::new("Setup Required")
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.foreground),
                    ),
            )
            .child(
                Label::new("Wayland requires input permissions for hotkeys.")
                    .text_sm()
                    .text_color(theme.muted_foreground),
            )
            .child(div().flex_1())
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        v_flex()
                            .p_4()
                            .gap_3()
                            .rounded(px(10.0))
                            .border_1()
                            .border_color(theme.border)
                            .child(
                                Label::new("Option 1 — Temporary")
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.foreground),
                            )
                            .child(CopyableCommand::new(
                                "sudo setfacl -m u:$USER:r /dev/input/event*",
                            )),
                    )
                    .child(
                        v_flex()
                            .p_4()
                            .gap_3()
                            .rounded(px(10.0))
                            .border_1()
                            .border_color(theme.border)
                            .child(
                                Label::new("Option 2 — Permanent")
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.foreground),
                            )
                            .child(CopyableCommand::new("sudo usermod -aG input $USER"))
                            .child(
                                Label::new("Log out and back in after.")
                                    .text_xs()
                                    .text_color(theme.muted_foreground),
                            ),
                    ),
            )
    }
}

#[derive(IntoElement)]
struct CopyableCommand {
    command: SharedString,
}

impl CopyableCommand {
    fn new(command: impl Into<SharedString>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

impl RenderOnce for CopyableCommand {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let command = self.command.clone();

        div()
            .id(ElementId::Name(self.command.clone()))
            .w_full()
            .px_3()
            .py_2p5()
            .rounded(px(6.0))
            .bg(theme.secondary)
            .cursor_pointer()
            .hover(|s| s.bg(theme.secondary.opacity(0.8)))
            .active(|s| s.bg(theme.secondary.opacity(0.6)))
            .on_click(move |_, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(command.to_string()));
            })
            .child(
                Label::new(self.command)
                    .text_xs()
                    .text_color(theme.foreground),
            )
    }
}
