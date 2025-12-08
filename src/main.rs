mod app;
mod clicker;
mod hotkey;
mod state;
mod ui;

use gpui::*;
use gpui_component::Root;
use std::path::PathBuf;

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let themes_dir = PathBuf::from("./themes");
        if themes_dir.exists() && themes_dir.is_dir() {
            if let Err(err) =
                gpui_component::ThemeRegistry::watch_dir(themes_dir.clone(), cx, move |cx| {
                    if let Some(theme) = gpui_component::ThemeRegistry::global(cx)
                        .themes()
                        .get(&gpui::SharedString::from("Autoclicker Dark"))
                        .cloned()
                    {
                        gpui_component::Theme::global_mut(cx).apply_config(&theme);
                    }
                })
            {
                eprintln!("Failed to watch themes directory: {}", err);
            }
        }

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
                    let view = cx.new(|cx| app::AutoClickerApp::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                })
            })
            .ok();
        })
        .detach();
    });
}
