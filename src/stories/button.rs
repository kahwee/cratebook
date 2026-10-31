use super::Story;
use crate::ui;
use gpui::{prelude::*, *};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        path: file!(),
        source: include_str!("button.rs"),
        title: "Button",
        description: "Activate with a click, Return or Space.",
        preview: cx.new(|_| Button { presses: 0 }).into(),
    }
}

struct Button {
    presses: usize,
}

impl Render for Button {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = ui::preview_palette(cx);
        div()
            .p_4()
            .flex()
            .items_center()
            .gap_3()
            .child(
                gpui_jig::button::root("run", "Run action")
                    .min_h(ui::control_height(window))
                    .text_size(px(13.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(theme.border))
                    .bg(rgb(theme.selected))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.presses += 1;
                        cx.notify();
                    }))
                    .child("Run action"),
            )
            .child(format!("Activations  {}", self.presses))
    }
}
