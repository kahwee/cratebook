use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{avatar, scroll_area};
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Scroll area",
        description: "Scroll a bounded contributor list with avatar fallbacks.",
        path: file!(),
        source: include_str!("scroll_area.rs"),
        preview: cx.new(|_| Contributors).into(),
    }
}
struct Contributors;
impl Render for Contributors {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        scroll_area::root().p_4().child(
            scroll_area::viewport("people").h(px(148.)).children(
                [
                    "Ada", "Bo", "Cam", "Dev", "Emi", "Flo", "Gia", "Hal", "Ira", "Jo",
                ]
                .into_iter()
                .map(|name| {
                    div()
                        .h(px(32.))
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            avatar::root(name, name)
                                .size(px(24.))
                                .rounded_full()
                                .bg(rgb(p.selected))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(avatar::fallback().child(name[..1].to_owned())),
                        )
                        .child(name)
                }),
            ),
        )
    }
}
