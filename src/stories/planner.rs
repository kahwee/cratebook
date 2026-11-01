use super::{Story, workbench};
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{checkbox, toggle_group};
#[derive(Clone, Copy, PartialEq)]
enum List {
    Inbox,
    Today,
    Logbook,
}
impl List {
    const ALL: [Self; 3] = [Self::Inbox, Self::Today, Self::Logbook];
    fn name(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::Today => "Today",
            Self::Logbook => "Logbook",
        }
    }
}
struct Task {
    title: String,
    today: bool,
    done: bool,
}
impl Task {
    fn list(&self) -> List {
        if self.done {
            List::Logbook
        } else if self.today {
            List::Today
        } else {
            List::Inbox
        }
    }
}
struct Planner {
    list: List,
    tasks: Vec<Task>,
    draft: String,
}
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Task planner",
        description: "Capture a to-do, schedule it for Today, and review completed work in the Logbook.",
        source: include_str!("planner.rs"),
        path: file!(),
        preview: cx
            .new(|_| Planner {
                list: List::Inbox,
                draft: String::new(),
                tasks: [
                    ("Gather workshop notes", false, false),
                    ("Prepare the design review", true, false),
                    ("Book the studio", false, true),
                ]
                .into_iter()
                .map(|(title, today, done)| Task {
                    title: title.to_owned(),
                    today,
                    done,
                })
                .collect(),
            })
            .into(),
    }
}
impl Render for Planner {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let nav = toggle_group::root("lists", "Task lists")
            .flex()
            .flex_col()
            .gap_1()
            .child("My lists")
            .children(List::ALL.into_iter().map(|list| {
                let count = self.tasks.iter().filter(|t| t.list() == list).count();
                toggle_group::item(list.name(), list.name(), list == self.list)
                    .min_h(ui::control_height(w))
                    .px_2()
                    .rounded(px(p.corners.control as f32))
                    .flex()
                    .items_center()
                    .justify_between()
                    .when(list == self.list, |s| s.bg(rgb(p.selected)))
                    .focus_visible(|s| s.underline())
                    .child(list.name())
                    .child(div().text_color(rgb(p.muted)).child(count.to_string()))
                    .on_click(cx.listener(move |s, _, _, cx| {
                        s.list = list;
                        cx.notify();
                    }))
            }));
        let detail = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_size(px(22.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.list.name()),
            )
            .when(self.list != List::Logbook, |s| {
                s.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .child(
                            workbench::entry("capture", "New to-do", &self.draft, w, cx, |s| {
                                &mut s.draft
                            })
                            .flex_1(),
                        )
                        .when(!self.draft.trim().is_empty(), |s| {
                            s.child(ui::action("Add task", w, cx, |s: &mut Planner| {
                                let title = s.draft.trim().to_owned();
                                if !title.is_empty() {
                                    s.tasks.push(Task {
                                        title,
                                        today: s.list == List::Today,
                                        done: false,
                                    });
                                    s.draft.clear();
                                }
                            }))
                        }),
                )
            })
            .when(!self.tasks.iter().any(|t| t.list() == self.list), |s| {
                s.child(
                    div()
                        .text_color(rgb(p.muted))
                        .child("No tasks in this list."),
                )
            })
            .children(
                self.tasks
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.list() == self.list)
                    .map(|(i, t)| {
                        div()
                            .flex()
                            .flex_wrap()
                            .items_center()
                            .gap_1()
                            .border_t_1()
                            .border_color(rgb(p.border))
                            .py_1()
                            .child(
                                checkbox::root(("task", i), t.title.clone(), t.done)
                                    .flex_1()
                                    .min_w(px(100.))
                                    .min_h(ui::control_height(w))
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .focus_visible(|s| s.underline())
                                    .child(
                                        checkbox::indicator()
                                            .size(px(13.))
                                            .border_1()
                                            .border_color(rgb(p.muted))
                                            .rounded(px(gpui_jig::corners::Corners::inset(
                                                p.corners.control,
                                                3,
                                            )
                                                as f32))
                                            .when(t.done, |s| s.bg(rgb(p.selected)).child("✓")),
                                    )
                                    .child(t.title.clone())
                                    .on_click(cx.listener(move |s, _, _, cx| {
                                        s.tasks[i].done = !s.tasks[i].done;
                                        cx.notify();
                                    })),
                            )
                            .when(!t.done, |s| {
                                s.child(
                                    gpui_jig::button::root(("schedule", i), "Change schedule")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child(if t.today {
                                            "Move to Inbox"
                                        } else {
                                            "Schedule today"
                                        })
                                        .on_click(cx.listener(move |s, _, _, cx| {
                                            s.tasks[i].today = !s.tasks[i].today;
                                            cx.notify();
                                        })),
                                )
                            })
                    }),
            );
        workbench::panes(w, p, nav, detail)
    }
}
