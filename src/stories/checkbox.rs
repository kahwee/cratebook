use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::checkbox;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Checkbox",
        description: "Choose which events appear in your activity feed.",
        path: file!(),
        source: include_str!("checkbox.rs"),
        preview: cx.new(|_| Feed([true, false, true])).into(),
    }
}
struct Feed([bool; 3]);
impl Render for Feed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        div().p_4().flex().flex_col().children(
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
