use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::alert_dialog;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Alert dialog",
        description: "Inspect the alert dialog parts in a bounded desktop preview.",
        path: file!(),
        source: include_str!("alert_dialog.rs"),
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
        alert_dialog::root()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                alert_dialog::trigger("open", "Remove draft", self.open)
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
                    .child("Remove draft"),
            )
            .when(self.open, |s| {
                s.child(
                    alert_dialog::popup("panel", "Alert dialog")
                        .p_3()
                        .rounded(px(p.corners.dialog as f32))
                        .border_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.canvas))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child("Remove the draft from this preview?")
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(
                                    alert_dialog::close("confirm", "Remove draft")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .rounded(px(p.corners.control as f32))
                                        .bg(rgb(p.selected))
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child("Remove draft")
                                        .on_click(cx.listener(|s, _, _, cx| {
                                            s.open = false;
                                            s.completed = true;
                                            cx.notify();
                                        })),
                                )
                                .child(ui::action("Cancel", w, cx, |s: &mut Preview| {
                                    s.open = false
                                })),
                        ),
                )
            })
            .when(!self.open && self.completed, |s| s.child("Draft removed"))
    }
}
