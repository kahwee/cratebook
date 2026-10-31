use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{progress, state::Range};
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Progress",
        description: "Advance a four-file batch and inspect the bounded progress value.",
        path: file!(),
        source: include_str!("progress.rs"),
        preview: cx.new(|_| Batch(Range::new(0., 4., 0.).unwrap())).into(),
    }
}
struct Batch(Range);
impl Render for Batch {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                progress::root("batch", "Files processed", &self.0)
                    .child(format!("{} of 4 files", self.0.value()))
                    .child(
                        div()
                            .mt_2()
                            .h(px(5.))
                            .rounded_full()
                            .bg(rgb(p.border))
                            .child(
                                div()
                                    .h_full()
                                    .w(relative((self.0.value() / 4.) as f32))
                                    .rounded_full()
                                    .bg(rgb(p.text)),
                            ),
                    ),
            )
            .child(ui::action(
                if self.0.value() == 4. {
                    "Restart batch"
                } else {
                    "Process next file"
                },
                w,
                cx,
                |s: &mut Batch| {
                    if s.0.value() == 4. {
                        s.0.set(0.);
                    } else {
                        s.0.step(1.);
                    }
                },
            ))
    }
}
