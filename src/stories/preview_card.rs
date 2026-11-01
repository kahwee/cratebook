use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::preview_card;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Preview card",
        description: "Inspect the preview card parts in a bounded desktop preview.",
        path: file!(),
        source: include_str!("preview_card.rs"),
        preview: cx
            .new(|_| Preview {
                open: false,
                completed: false,
            })
            .into(),
    }
}
struct Preview {
    open: bool,
    completed: bool,
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        preview_card::root()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                preview_card::trigger("open", "Field notes.md", self.open)
                    .min_h(ui::control_height(w))
                    .flex()
                    .items_center()
                    .px_2()
                    .rounded(px(p.corners.control as f32))
                    .bg(rgb(p.sidebar))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|s, _, _, cx| {
                        s.open = !s.open;
                        cx.notify();
                    }))
                    .child("Field notes.md"),
            )
            .when(self.open, |s| {
                s.child(
                    preview_card::popup("panel", "Preview card")
                        .p_3()
                        .rounded(px(p.corners.card as f32))
                        .border_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.canvas))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child("Research notes from the desktop workspace. Updated today.")
                        .child(
                            div().flex().flex_wrap().gap_2().child(
                                preview_card::close("confirm", "Close preview")
                                    .min_h(ui::control_height(w))
                                    .px_2()
                                    .rounded(px(p.corners.control as f32))
                                    .bg(rgb(p.selected))
                                    .flex()
                                    .items_center()
                                    .focus_visible(|s| s.underline())
                                    .child("Close preview")
                                    .on_click(cx.listener(|s, _, _, cx| {
                                        s.open = false;
                                        s.completed = true;
                                        cx.notify();
                                    })),
                            ),
                        ),
                )
            })
            .when(!self.open && self.completed, |s| s.child("Preview closed"))
    }
}
