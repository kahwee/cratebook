use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::dialog;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Dialog",
        description: "Inspect the dialog parts in a bounded desktop preview.",
        path: file!(),
        source: include_str!("dialog.rs"),
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
        dialog::root()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                dialog::trigger("open", "Export report", self.open)
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
                    .child("Export report"),
            )
            .when(self.open, |s| {
                s.child(
                    dialog::popup("panel", "Dialog")
                        .p_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.canvas))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child("Export the current report as a PDF?")
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(
                                    dialog::close("confirm", "Export PDF")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .rounded_md()
                                        .bg(rgb(p.selected))
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child("Export PDF")
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
            .when(!self.open && self.completed, |s| s.child("Report exported"))
    }
}
