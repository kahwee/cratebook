use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{meter, state::Range};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Meter",
        description: "Compare workspace storage against a fixed capacity.",
        path: file!(),
        source: include_str!("meter.rs"),
        preview: cx.new(|_| Storage(Range::new(0., 10., 4.).unwrap())).into(),
    }
}
struct Storage(Range);
impl Render for Storage {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                meter::root("storage", "Workspace storage", &self.0)
                    .child(meter::value().child(format!("{} GB of 10 GB used", self.0.value())))
                    .child(
                        meter::track()
                            .h(px(6.))
                            .mt_2()
                            .rounded_full()
                            .bg(rgb(p.border))
                            .child(
                                meter::indicator()
                                    .h_full()
                                    .w(relative((self.0.value() / 10.) as f32))
                                    .bg(rgb(p.text))
                                    .rounded_full(),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(ui::action("Add 1 GB", w, cx, |s: &mut Storage| {
                        s.0.step(1.)
                    }))
                    .child(ui::action("Free 1 GB", w, cx, |s: &mut Storage| {
                        s.0.step(-1.)
                    })),
            )
    }
}
