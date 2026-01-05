mod app;
mod clicker;
mod hotkey;
mod state;
mod ui;

use gpui::*;
use gpui_component::Root;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

const EMBEDDED_THEME: &str = include_str!("../themes/shadcn.json");

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        setup_themes(cx);

        let force_warning = std::env::var("SHOW_WAYLAND_WARNING").is_ok();

        cx.spawn(async move |cx| {
            cx.update(|cx| {
                let options = WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(420.0), px(650.0)),
                        cx,
                    ))),
                    window_min_size: Some(size(px(380.0), px(600.0))),
                    ..Default::default()
                };

                cx.open_window(options, |window, cx| {
                    window.set_window_title("AutoClicker");
                    let view = cx.new(|cx| {
                        let app_view = app::AutoClickerApp::new(window, cx);
                        if force_warning {
                            app_view
                                .state
                                .runtime
                                .force_show_warning
                                .store(true, Ordering::SeqCst);
                        }
                        app_view
                    });
                    cx.new(|cx| Root::new(view, window, cx))
                })
            })
            .ok();
        })
        .detach();
    });
}

fn setup_themes(cx: &mut App) {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autoclicker")
        .join("themes");

    std::fs::create_dir_all(&config_dir).ok();
    std::fs::write(config_dir.join("theme.json"), EMBEDDED_THEME).ok();

    gpui_component::ThemeRegistry::watch_dir(config_dir, cx, apply_theme_for_mode).ok();
}

fn apply_theme_for_mode(cx: &mut App) {
    let is_dark = gpui_component::Theme::global(cx).mode.is_dark();
    let theme_name = if is_dark {
        "Autoclicker Dark"
    } else {
        "Autoclicker Light"
    };

    if let Some(theme) = gpui_component::ThemeRegistry::global(cx)
        .themes()
        .get(&SharedString::from(theme_name))
        .cloned()
    {
        gpui_component::Theme::global_mut(cx).apply_config(&theme);
    }
}
