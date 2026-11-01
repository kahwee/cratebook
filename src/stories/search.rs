use super::{Story, workbench};
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{autocomplete, combobox};

pub(super) fn stories(cx: &mut App) -> [Story; 2] {
    [false, true].map(|free| Story {
        title: if free { "Autocomplete" } else { "Combobox" },
        description: if free {
            "Type a project tag. Select a suggestion or press Return to keep your own."
        } else {
            "Filter workspace names, then choose a matching workspace."
        },
        path: file!(),
        source: include_str!("search.rs"),
        preview: cx
            .new(|_| Search {
                free,
                query: String::new(),
                chosen: String::new(),
                open: false,
            })
            .into(),
    })
}
struct Search {
    free: bool,
    query: String,
    chosen: String,
    open: bool,
}
impl Render for Search {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let names = if self.free {
            ["Design", "Research", "Engineering"]
        } else {
            ["Desktop", "Documentation", "Experiments"]
        };
        let matches: Vec<_> = names
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&self.query.to_lowercase()))
            .collect();
        let edit = div()
            .id("query")
            .focusable()
            .tab_stop(true)
            .min_h(ui::control_height(w))
            .flex()
            .items_center()
            .px_2()
            .border_1()
            .border_color(rgb(p.border))
            .rounded(px(p.corners.control as f32))
            .focus_visible(move |s| s.border_color(rgb(p.text)));
        let edit = if self.free {
            autocomplete::input(edit, "Project tag", self.open)
        } else {
            combobox::input(edit, "Workspace name", self.open)
        };
        let first = matches.first().copied();
        let edit = edit
            .child(if self.query.is_empty() {
                "Type to filter…".into()
            } else {
                self.query.clone()
            })
            .on_click(cx.listener(|s, _, _, cx| {
                s.open = true;
                cx.notify();
            }))
            .on_key_down(cx.listener(move |s, e: &KeyDownEvent, w, cx| {
                if let Some(text) = workbench::pasted_text(e, cx) {
                    s.query.push_str(&text);
                    s.open = true;
                    w.prevent_default();
                    cx.notify();
                    return;
                }
                if e.keystroke.modifiers.control || e.keystroke.modifiers.platform {
                    return;
                }
                match e.keystroke.key.as_str() {
                    "escape" => s.open = false,
                    "enter" => {
                        if s.free && !s.query.trim().is_empty() {
                            s.chosen = s.query.clone();
                        } else if let Some(value) = first {
                            s.chosen = value.into();
                            s.query = value.into();
                        }
                        s.open = false;
                    }
                    "backspace" => {
                        s.query.pop();
                        s.open = true;
                    }
                    _ => {
                        if let Some(ch) = &e.keystroke.key_char {
                            if ch.chars().any(char::is_control) {
                                return;
                            }
                            s.query.push_str(ch);
                            s.open = true;
                        } else {
                            return;
                        }
                    }
                }
                w.prevent_default();
                cx.notify();
            }));
        let list = if self.free {
            autocomplete::list("matches", "Suggested tags")
        } else {
            combobox::list("matches", "Matching workspaces")
        };
        let root = if self.free {
            autocomplete::root()
        } else {
            combobox::root()
        };
        root.p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(edit)
            .when(self.open, |s| {
                s.child(
                    list.when(matches.is_empty(), |s| s.child("No suggestions"))
                        .children(matches.into_iter().map(|name| {
                            combobox::item(name, name, self.chosen == name)
                                .min_h(ui::control_height(w))
                                .flex()
                                .items_center()
                                .px_2()
                                .hover(move |s| s.bg(rgb(p.selected)))
                                .focus_visible(|s| s.underline())
                                .child(name)
                                .on_click(cx.listener(move |s, _, _, cx| {
                                    s.chosen = name.into();
                                    s.query = name.into();
                                    s.open = false;
                                    cx.notify();
                                }))
                        })),
                )
            })
            .when(!self.chosen.is_empty(), |s| {
                s.child(format!("Selected: {}", self.chosen))
            })
    }
}
