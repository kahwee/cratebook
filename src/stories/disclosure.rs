use super::Story;
use gpui::{prelude::*, *};
use gpui_jig::collapsible;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        path: file!(),
        source: include_str!("disclosure.rs"),
        title: "Collapsible",
        description: "Expand or collapse the list of page references.",
        preview: cx
            .new(|_| References {
                expanded: crate::capture_mode(),
            })
            .into(),
    }
}

struct References {
    expanded: bool,
}

impl Render for References {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        collapsible::root()
            .p_4()
            .child(
                collapsible::trigger("details", "Page references", self.expanded)
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.expanded = !this.expanded;
                        cx.notify();
                    }))
                    .child(if self.expanded {
                        "▾  Page references"
                    } else {
                        "▸  Page references"
                    }),
            )
            .when(self.expanded, |s| {
                s.child(
                    collapsible::panel()
                        .p_2()
                        .child("Reading list.md   →   Field notes.md"),
                )
            })
    }
}
