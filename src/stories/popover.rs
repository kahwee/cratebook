use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::popover;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Popover",
        description: "Inspect the popover parts in a bounded desktop preview.",
        path: file!(),
        source: include_str!("popover.rs"),
        preview: cx
            .new(|_| Preview {
                open: crate::capture_mode(),
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
        popover::root()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                popover::trigger("open", "Show details", self.open)
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
                    .child("Show details"),
            )
            .when(self.open, |s| {
                s.child(
                    popover::popup("panel", "Popover")
                        .p_3()
                        .rounded(px(p.corners.card as f32))
                        .border_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.canvas))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child("Working tree is clean. Last sync finished at 09:41.")
                        .child(
                            div().flex().flex_wrap().gap_2().child(
                                popover::close("confirm", "Got it")
                                    .min_h(ui::control_height(w))
                                    .px_2()
                                    .rounded(px(p.corners.control as f32))
                                    .bg(rgb(p.selected))
                                    .flex()
                                    .items_center()
                                    .focus_visible(|s| s.underline())
                                    .child("Got it")
                                    .on_click(cx.listener(|s, _, _, cx| {
                                        s.open = false;
                                        s.completed = true;
                                        cx.notify();
                                    })),
                            ),
                        ),
                )
            })
            .when(!self.open && self.completed, |s| s.child("Details closed"))
    }
}
