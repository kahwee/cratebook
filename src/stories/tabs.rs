use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::tabs;

const TITLES: [&str; 3] = [
    "Field notes.md",
    "Reading list.md",
    "An unusually long working title for a draft.md",
];
const EXCERPTS: [&str; 3] = [
    "The studio was quiet before the first interview. On the table: a recorder, a notebook, and three questions.",
    "A short shelf of books on observation, drawing, and the everyday details of working places.",
    "An early outline for the next issue. Start with the workshop visit, then return to the open questions.",
];

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        path: file!(),
        source: include_str!("tabs.rs"),
        title: "Document tabs",
        description: "Browse the notebook. Use the arrow keys to switch documents or close a tab to continue reading.",
        preview: cx
            .new(|cx| Tabs {
                tabs: vec![0, 1, 2],
                active: 0,
                dirty: false,
                handles: std::array::from_fn(|_| cx.focus_handle()),
                restore: cx.focus_handle(),
            })
            .into(),
    }
}
struct Tabs {
    tabs: Vec<usize>,
    active: usize,
    dirty: bool,
    handles: [FocusHandle; 3],
    restore: FocusHandle,
}
impl Tabs {
    fn tab(&self, index: usize, pos: usize, p: ui::Palette, w: &Window, cx: &Context<Self>) -> Div {
        let active = self.active == index;
        let compact = ui::compact_layout(w);
        let title = TITLES[index];
        div()
            .flex()
            .items_center()
            .when(compact, |s| {
                s.flex_1().min_w_0().when(active, |s| s.flex_basis(px(44.)))
            })
            .when(!compact, |s| s.flex_shrink_0())
            .h_full()
            .px_1()
            .rounded(px(p.corners.control as f32))
            .border_1()
            .border_color(rgb(if active { p.border } else { p.sidebar }))
            .when(active, |s| s.bg(rgb(p.canvas)))
            .when(!active, |s| s.hover(move |s| s.bg(rgb(p.selected))))
            .child(
                tabs::tab(
                    ("tab", index),
                    if index == 0 && self.dirty {
                        format!("{title}, unsaved")
                    } else {
                        title.to_owned()
                    },
                    active,
                )
                .track_focus(&self.handles[index])
                .h_full()
                .flex()
                .items_center()
                .gap_2()
                .px_1()
                .max_w(px(if compact { 136. } else { 196. }))
                .min_w_0()
                .text_color(rgb(if active { p.text } else { p.muted }))
                .when(active, |s| s.font_weight(FontWeight::SEMIBOLD))
                .focus_visible(|s| s.underline())
                .when(!compact, |s| {
                    s.child(
                        svg()
                            .path("phosphor/file-text.svg")
                            .size(px(15.))
                            .flex_shrink_0()
                            .text_color(rgb(p.muted)),
                    )
                })
                .when(index == 0 && self.dirty, |s| {
                    s.child(
                        div()
                            .size(px(5.))
                            .flex_shrink_0()
                            .rounded_full()
                            .bg(rgb(p.text)),
                    )
                })
                .when(compact, |s| s.flex_1())
                .child(div().min_w_0().truncate().child(if compact {
                    title.trim_end_matches(".md")
                } else {
                    title
                }))
                .on_click(cx.listener(move |s, _, w, cx| {
                    s.active = index;
                    w.focus(&s.handles[index], cx);
                    cx.notify();
                }))
                .on_key_down(cx.listener(move |s, e: &KeyDownEvent, w, cx| {
                    if e.keystroke.modifiers.modified() {
                        return;
                    }
                    if let Some(next) = tabs::destination(&e.keystroke.key, pos, s.tabs.len()) {
                        s.active = s.tabs[next];
                        w.focus(&s.handles[s.active], cx);
                        w.prevent_default();
                        cx.stop_propagation();
                        cx.notify();
                    }
                })),
            )
            .when(!compact || active, |s| {
                s.child(
                    tabs::close(("close", index), format!("Close {title}"))
                        .flex_shrink_0()
                        .w(px(if compact { 44. } else { 28. }))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(p.muted))
                        .rounded(px(
                            gpui_jig::corners::Corners::inset(p.corners.control, 1) as f32
                        ))
                        .focus_visible(|s| s.underline())
                        .hover(move |s| s.bg(rgb(p.selected)))
                        .child(
                            svg()
                                .path("phosphor/x.svg")
                                .size(px(12.))
                                .text_color(rgb(p.muted)),
                        )
                        .on_click(cx.listener(move |s, _, w, cx| {
                            s.tabs.remove(pos);
                            if s.active == index && !s.tabs.is_empty() {
                                s.active = s.tabs[pos.min(s.tabs.len() - 1)];
                            }
                            if !s.tabs.is_empty() {
                                w.focus(&s.handles[s.active], cx);
                            } else {
                                w.focus(&s.restore, cx);
                            }
                            cx.notify();
                        })),
                )
            })
    }
}
impl Render for Tabs {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let strip = tabs::list("tab-strip", "Open documents")
            .overflow_x_scroll()
            .flex()
            .h(px(if ui::compact_layout(w) { 52. } else { 40. }))
            .py_1()
            .px_1()
            .gap(px(3.))
            .bg(rgb(p.sidebar))
            .rounded_t(px(
                gpui_jig::corners::Corners::inset(p.corners.card, 1) as f32
            ))
            .text_size(px(13.))
            .children(
                self.tabs
                    .iter()
                    .enumerate()
                    .map(|(pos, &i)| self.tab(i, pos, p, w, cx)),
            );
        tabs::root("documents", "Notebook")
            .flex()
            .flex_col()
            .when(!self.tabs.is_empty(), |s| s.child(strip))
            .child(
                tabs::panel("document-content", "Active document")
                    .flex()
                    .flex_col()
                    .gap_3()
                    .min_h(px(200.))
                    .px(px(if ui::compact_layout(w) { 16. } else { 24. }))
                    .py(px(if ui::compact_layout(w) { 20. } else { 24. }))
                    .bg(rgb(p.canvas))
                    .when(!self.tabs.is_empty(), |s| {
                        s.child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(p.muted))
                                .child("Studio notebook  /  Manuscripts"),
                        )
                        .child(
                            div()
                                .text_size(px(22.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(TITLES[self.active].trim_end_matches(".md")),
                        )
                        .child(
                            div()
                                .max_w(px(520.))
                                .text_size(px(14.))
                                .text_color(rgb(p.muted))
                                .child(EXCERPTS[self.active]),
                        )
                        .child(
                            div()
                                .mt_2()
                                .text_size(px(12.))
                                .text_color(rgb(p.muted))
                                .child(if self.active == 0 && self.dirty {
                                    "Unsaved changes"
                                } else {
                                    "Ready to read"
                                }),
                        )
                    })
                    .when(self.tabs.is_empty(), |s| {
                        s.child("No open documents")
                            .child("Restore the notebook to reopen its three documents.")
                    }),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_t_1()
                    .border_color(rgb(p.border))
                    .bg(rgb(p.sidebar))
                    .rounded_b(px(
                        gpui_jig::corners::Corners::inset(p.corners.card, 1) as f32
                    ))
                    .flex()
                    .items_center()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        div()
                            .when(ui::compact_layout(w), |s| s.w_full())
                            .when(!ui::compact_layout(w), |s| s.flex_1().min_w(px(100.)))
                            .text_size(px(12.))
                            .text_color(rgb(p.muted))
                            .child(format!("{} open", self.tabs.len())),
                    )
                    .when(self.tabs.contains(&0), |s| {
                        s.child(ui::action(
                            if self.dirty {
                                "Mark saved"
                            } else {
                                "Mark notes unsaved"
                            },
                            w,
                            cx,
                            |s: &mut Tabs| s.dirty = !s.dirty,
                        ))
                    })
                    .child(
                        ui::action("Restore tabs", w, cx, |s: &mut Tabs| {
                            s.tabs = vec![0, 1, 2];
                            s.active = 0;
                            s.dirty = false;
                        })
                        .track_focus(&self.restore),
                    ),
            )
    }
}
