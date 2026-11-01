mod gallery;
mod stories;
pub mod table;
pub mod ui;
use gallery::Gallery;
use gpui::{prelude::*, *};

/// Deterministic, native comparison fixture; ordinary app launches are unchanged.
pub(crate) fn capture_mode() -> bool {
    !cfg!(target_family = "wasm")
        && (std::env::args().any(|arg| arg == "--capture")
            || std::env::current_exe()
                .ok()
                .and_then(|p| p.file_stem().map(|s| s == "cratebook-capture"))
                .unwrap_or(false))
}
fn open_gallery(cx: &mut App) {
    // A standalone Windows gallery should exit when its last window closes.
    #[cfg(target_os = "windows")]
    cx.on_window_closed(|cx, _| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach();
    #[cfg(target_family = "wasm")]
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/NotoSans.ttf"
        ))])
        .expect("embedded gallery font");
    let (mut theme, mut palette) = (ui::Theme::Editor, ui::Palette::LIGHT);
    if !cfg!(target_family = "wasm") {
        for arg in std::env::args().skip(1) {
            if let Some(name) = arg.strip_prefix("--theme=") {
                if let Some(choice) = ui::Theme::ALL
                    .iter()
                    .find(|choice| choice.name().eq_ignore_ascii_case(name))
                {
                    theme = *choice;
                }
            } else if arg == "--dark" {
                palette = ui::Palette::DARK;
            }
        }
    }
    cx.set_global(palette);
    cx.set_global(theme);
    cx.set_global(ui::CornerStyle::default());
    let dimensions = if capture_mode() {
        size(px(960.), px(640.))
    } else {
        size(px(1120.), px(720.))
    };
    let mut bounds = Bounds::centered(None, dimensions, cx);
    if capture_mode() {
        // Fractional centered origins can make macOS captures one pixel taller.
        // An integer origin keeps native client bounds pixel-aligned at 1x.
        bounds.origin = point(px(32.), px(32.));
    }
    cx.open_window(
        WindowOptions {
            window_min_size: Some(size(px(360.), px(480.))),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            app_owns_titlebar_drag: cfg!(target_os = "macos"),
            titlebar: Some(TitlebarOptions {
                title: Some("Cratebook".into()),
                appears_transparent: cfg!(target_os = "macos"),
                traffic_light_position: cfg!(target_os = "macos").then(|| point(px(15.), px(17.))),
            }),
            ..Default::default()
        },
        |_, cx| cx.new(Gallery::new),
    )
    .unwrap();
    cx.activate(true);
}
#[cfg(not(target_family = "wasm"))]
pub fn run_native() {
    gpui_platform::application()
        .with_assets(ui::icons::Icons)
        .run(open_gallery);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_web() {
    gpui_platform::web_init();
    gpui_platform::single_threaded_web()
        .with_assets(ui::icons::Icons)
        .run(open_gallery);
}
