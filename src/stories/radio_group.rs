use super::Story;
use crate::ui;
use gpui::{prelude::*, *};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        path: file!(),
        source: include_str!("radio_group.rs"),
        title: "Radio group",
        description: "Use the arrow keys to switch modes.",
        preview: cx
            .new(|cx| Modes {
                radio: ui::RadioGroup::new(cx),
                choice: 0,
            })
            .into(),
    }
}

struct Modes {
    radio: ui::RadioGroup<3>,
    choice: usize,
}

impl Render for Modes {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = ui::preview_palette(cx);

        let mut group = self.radio.root("view-mode", "View mode").flex().gap_1();
        for (i, name) in ["Read", "Write", "Source"].into_iter().enumerate() {
            group = group.child(
                self.radio
                    .item(
                        ("mode", i),
                        name,
                        i,
                        self.choice == i,
                        cx.listener(|this, index, _, cx| {
                            this.choice = *index;
                            cx.notify();
                        }),
                    )
                    .px_3()
                    .py_1()
                    .when(self.choice == i, |s| s.bg(rgb(palette.selected)))
                    .focus_visible(|s| s.underline())
                    .child(name),
            );
        }
        div().p_4().child(group)
    }
}
