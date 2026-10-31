use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{checkbox, checkbox_group};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Checkbox",
        description: "Select all events, or choose individual notifications. A dash means only some are selected.",
        path: file!(),
        source: include_str!("checkbox.rs"),
        preview: cx.new(|_| Feed([true, false, true])).into(),
    }
}
struct Feed([bool; 3]);
impl Render for Feed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let state = checkbox_group::aggregate(self.0);
        checkbox_group::root("events", "Feed notifications")
            .p_4()
            .flex()
            .flex_col()
            .child(
                checkbox::tri_state("select-all", "All events", state)
                    .min_h(ui::control_height(window))
                    .flex()
                    .gap_2()
                    .items_center()
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(move |s, _, _, cx| {
                        s.0.fill(state != Toggled::True);
                        cx.notify();
                    }))
                    .child(
                        checkbox::indicator()
                            .size(px(14.))
                            .border_1()
                            .rounded_sm()
                            .border_color(rgb(p.muted))
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(state != Toggled::False, |s| {
                                s.child(
                                    div()
                                        .w(px(8.))
                                        .h(px(if state == Toggled::Mixed { 2. } else { 8. }))
                                        .bg(rgb(p.text)),
                                )
                            }),
                    )
                    .child("All events"),
            )
            .children(
                ["Mentions", "Replies", "Assignments"]
                    .into_iter()
                    .enumerate()
                    .map(|(i, label)| {
                        checkbox::root(("event", i), label, self.0[i])
                            .min_h(ui::control_height(window))
                            .flex()
                            .gap_2()
                            .items_center()
                            .focus_visible(|s| s.underline())
                            .on_click(cx.listener(move |s, _, _, cx| {
                                s.0[i] = !s.0[i];
                                cx.notify();
                            }))
                            .child(
                                checkbox::indicator()
                                    .size(px(14.))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(rgb(p.muted))
                                    .when(self.0[i], |s| {
                                        s.bg(rgb(p.text))
                                            .p(px(3.))
                                            .child(div().size_full().bg(rgb(p.canvas)))
                                    }),
                            )
                            .child(label)
                    }),
            )
    }
}
