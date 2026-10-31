use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::drawer;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Drawer",
        description: "Inspect the drawer parts in a bounded desktop preview.",
        path: file!(),
        source: include_str!("drawer.rs"),
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
        drawer::root()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                drawer::trigger("open", "Show inspector", self.open)
                    .min_h(ui::control_height(w))
                    .flex()
                    .items_center()
                    .px_2()
                    .rounded_md()
                    .bg(rgb(p.sidebar))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|s, _, _, cx| {
                        s.open = !s.open;
                        cx.notify();
                    }))
                    .child("Show inspector"),
            )
            .when(self.open, |s| {
                s.child(
                    div().flex().justify_end().child(
                        drawer::popup("panel", "Drawer")
                            .p_3()
                            .rounded_lg()
                            .border_1()
                            .border_color(rgb(p.border))
                            .bg(rgb(p.canvas))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child("Field notes.md · Markdown · Edited today")
                            .child(
                                div().flex().flex_wrap().gap_2().child(
                                    drawer::close("confirm", "Close inspector")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .rounded_md()
                                        .bg(rgb(p.selected))
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child("Close inspector")
                                        .on_click(cx.listener(|s, _, _, cx| {
                                            s.open = false;
                                            s.completed = true;
                                            cx.notify();
                                        })),
                                ),
                            )
                            .w(px(240.)),
                    ),
                )
            })
            .when(!self.open && self.completed, |s| {
                s.child("Inspector closed")
            })
    }
}
