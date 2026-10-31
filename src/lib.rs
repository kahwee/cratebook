mod gallery;
mod stories;
pub mod table;
pub mod ui;
use gallery::Gallery;
use gpui::{prelude::*, *};
fn open_gallery(cx: &mut App) {
    #[cfg(target_family = "wasm")]
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/NotoSans.ttf"
        ))])
        .expect("embedded gallery font");
    cx.set_global(ui::Palette::LIGHT);
    cx.set_global(ui::Theme::Editor);
    let bounds = Bounds::centered(None, size(px(1120.), px(720.)), cx);
    cx.open_window(
        WindowOptions {
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
    gpui_platform::application().run(open_gallery);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_web() {
    gpui_platform::web_init();
    gpui_platform::single_threaded_web().run(open_gallery);
}
