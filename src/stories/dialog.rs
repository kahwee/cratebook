use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{card, dialog, toggle_group};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Dialog",
        description: "Compose a report card and an export dialog. Choose a format and save the preset.",
        path: file!(),
        source: include_str!("dialog.rs"),
        preview: cx
            .new(|_| Preview {
                open: false,
                saved: "PDF",
                draft: "PDF",
            })
            .into(),
    }
}
struct Preview {
    open: bool,
    saved: &'static str,
    draft: &'static str,
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
                card::root("report", "Quarterly report")
                    .p_3()
                    .rounded_lg()
                    .bg(rgb(p.sidebar))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(p.muted))
                            .child("REPORT LIBRARY   /   Q3"),
                    )
                    .child(
                        card::title("report-name", "Quarterly review")
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Quarterly review"),
                    )
                    .child(card::content().child(format!("Export preset · {}", self.saved))),
            )
            .when(!self.open, |s| {
                s.child(
                    dialog::trigger("open", "Configure export", self.open)
                        .min_h(ui::control_height(w))
                        .flex()
                        .items_center()
                        .px_2()
                        .rounded_md()
                        .bg(rgb(p.sidebar))
                        .focus_visible(|s| s.underline())
                        .on_click(cx.listener(|s, _, _, cx| {
                            s.draft = s.saved;
                            s.open = true;
                            cx.notify();
                        }))
                        .child("Configure export"),
                )
            })
            .when(self.open, |s| {
                s.child(
                    dialog::popup("panel", "Export settings")
                        .p_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.canvas))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_size(px(18.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Export settings"),
                        )
                        .child(
                            div()
                                .text_color(rgb(p.muted))
                                .child("Choose the format for this report."),
                        )
                        .child(
                            toggle_group::root("formats", "Export format")
                                .flex()
                                .flex_wrap()
                                .gap_1()
                                .children(["PDF", "Markdown", "CSV"].into_iter().map(|format| {
                                    let selected = format == self.draft;
                                    toggle_group::item(format, format, selected)
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .flex()
                                        .items_center()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(rgb(p.border))
                                        .when(selected, |s| s.bg(rgb(p.selected)))
                                        .focus_visible(|s| s.underline())
                                        .child(format)
                                        .on_click(cx.listener(move |s, _, _, cx| {
                                            s.draft = format;
                                            cx.notify();
                                        }))
                                })),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(
                                    dialog::close("confirm", "Save preset")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .rounded_md()
                                        .bg(rgb(p.selected))
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child("Save preset")
                                        .on_click(cx.listener(|s, _, _, cx| {
                                            s.open = false;
                                            s.saved = s.draft;
                                            cx.notify();
                                        })),
                                )
                                .child(ui::action("Cancel", w, cx, |s: &mut Preview| {
                                    s.open = false
                                })),
                        ),
                )
            })
    }
}
