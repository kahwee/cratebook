use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{accordion, state::Accordion};
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Accordion",
        description: "Explore one release-note section at a time.",
        path: file!(),
        source: include_str!("accordion.rs"),
        preview: cx
            .new(|_| {
                let mut state = Accordion::new(false);
                if crate::capture_mode() {
                    state.toggle(0);
                }
                Notes(state)
            })
            .into(),
    }
}
struct Notes(Accordion<usize>);
impl Render for Notes {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        accordion::root("notes", "Release notes").p_4().children(
            [
                ("Components", "Cards support a title and content."),
                (
                    "Keyboard",
                    "Use Tab to focus a section and Space to open it.",
                ),
                (
                    "Appearance",
                    "Select Mint in the gallery to preview another palette.",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(|(i, (title, body))| {
                let open = self.0.is_open(&i);
                accordion::item(("section", i), title)
                    .child(
                        accordion::header("heading", title).child(
                            accordion::trigger("expand", title, open)
                                .min_h(ui::control_height(w))
                                .flex()
                                .items_center()
                                .focus_visible(|s| s.underline())
                                .child(format!("{}  {title}", if open { "−" } else { "+" }))
                                .on_click(cx.listener(move |s, _, _, cx| {
                                    s.0.toggle(i);
                                    cx.notify();
                                })),
                        ),
                    )
                    .when(open, |s| s.child(accordion::panel().p_2().child(body)))
            }),
        )
    }
}
