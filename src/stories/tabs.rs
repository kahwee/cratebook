use super::Story;
use crate::ui;
use gpui::{prelude::*, *};

const TITLES: [&str; 3] = [
    "Field notes.md",
    "Reading list.md",
    "An unusually long working title for a draft.md",
];

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        path: file!(),
        source: include_str!("tabs.rs"),
        title: "Document tabs",
        description: "Switch and close documents, or mark the first tab unsaved.",
        preview: cx
            .new(|_| Tabs {
                tabs: vec![0, 1, 2],
                active: 0,
                dirty: false,
            })
            .into(),
    }
}

struct Tabs {
    tabs: Vec<usize>,
    active: usize,
    dirty: bool,
}

impl Tabs {
    fn tab(&self, index: usize, palette: ui::Palette, cx: &Context<Self>) -> Stateful<Div> {
        let title = TITLES[index];
        ui::document_tab(
            ("tab", index),
            format!("Open {title}"),
            self.active == index,
            palette.canvas,
            palette.sidebar,
            palette.border,
            palette.selected,
        )
        .on_click(cx.listener(move |this, _, _, cx| {
            this.active = index;
            cx.notify();
        }))
        .child(
            div()
                .max_w(px(180.))
                .overflow_hidden()
                .text_ellipsis()
                .child(format!(
                    "{}{title}",
                    if index == 0 && self.dirty { "● " } else { "" }
                )),
        )
        .child(
            ui::button(("close", index), format!("Close {title}"))
                .px_1()
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.tabs.retain(|v| *v != index);
                    if this.active == index {
                        this.active = this.tabs.first().copied().unwrap_or(0);
                    }
                    cx.notify();
                }))
                .child("×"),
        )
    }
}
impl Render for Tabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = ui::preview_palette(cx);
        let strip = gpui_jig::tabs::list("tab-strip", "Open documents")
            .overflow_x_scroll()
            .flex()
            .h(ui::control_height(window))
            .bg(rgb(palette.sidebar))
            .border_b_1()
            .border_color(rgb(palette.border))
            .text_size(px(14.))
            .children(self.tabs.iter().map(|&index| self.tab(index, palette, cx)));
        gpui_jig::tabs::root("documents", "Documents")
            .flex()
            .flex_col()
            .child(strip)
            .child(
                gpui_jig::tabs::panel("document-content", "Active document")
                    .p_4()
                    .child(if self.tabs.is_empty() {
                        "No open documents"
                    } else {
                        TITLES[self.active]
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .p_3()
                    .child(ui::action("Toggle unsaved mark", window, cx, |s| {
                        s.dirty = !s.dirty
                    }))
                    .child(ui::action("Restore tabs", window, cx, |s| {
                        s.tabs = vec![0, 1, 2];
                        s.active = 0;
                        s.dirty = false;
                    })),
            )
    }
}
