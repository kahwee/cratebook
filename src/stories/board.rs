use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{button, card, toggle_group, toolbar};

const LANES: [&str; 3] = ["Planned", "In progress", "Done"];
const TASKS: [(&str, &str); 5] = [
    ("Sketch the month view", "Design"),
    ("Review keyboard navigation", "Engineering"),
    ("Write the onboarding guide", "Docs"),
    ("Ship the first prototype", "Engineering"),
    ("Collect field notes", "Research"),
];
struct Board {
    grid: gpui_jig::date_grid::DateGrid,
    due: [chrono::NaiveDate; 5],
    lanes: [usize; 5],
    selected: Option<usize>,
}
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Project board",
        description: "Open a work item and move it through the sprint. Lane counts follow the work.",
        path: file!(),
        source: include_str!("board.rs"),
        preview: cx
            .new(|cx| Board {
                grid: gpui_jig::date_grid::DateGrid::new(cx),
                due: [chrono::NaiveDate::from_ymd_opt(2026, 10, 12).unwrap(); 5],
                lanes: [0, 1, 0, 1, 2],
                selected: None,
            })
            .into(),
    }
}
impl Render for Board {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let h = ui::control_height(w);
        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_3()
            .child(
                toolbar::root("board-summary", "Sprint summary")
                    .flex()
                    .flex_wrap()
                    .justify_between()
                    .child("Workspace / Autumn sprint")
                    .child(format!(
                        "{} of {} done",
                        self.lanes.iter().filter(|&&lane| lane == 2).count(),
                        TASKS.len()
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .when(ui::compact_layout(w), |s| s.flex_col())
                    .children(LANES.iter().enumerate().map(|(lane, &name)| {
                        let count = self.lanes.iter().filter(|&&l| l == lane).count();
                        card::root(("lane", lane), name)
                            .flex_1()
                            .min_w_0()
                            .p_2()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .rounded_lg()
                            .bg(rgb(p.sidebar))
                            .child(
                                card::title("heading", name)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format!("{name} · {count}")),
                            )
                            .children(
                                TASKS
                                    .iter()
                                    .enumerate()
                                    .filter(|(i, _)| self.lanes[*i] == lane)
                                    .map(|(i, &(title, team))| {
                                        button::root(("task", i), title)
                                            .min_h(h)
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(p.canvas))
                                            .border_1()
                                            .border_color(rgb(if self.selected == Some(i) {
                                                p.text
                                            } else {
                                                p.border
                                            }))
                                            .focus_visible(|s| s.underline())
                                            .child(
                                                div()
                                                    .text_size(px(12.))
                                                    .text_color(rgb(p.muted))
                                                    .child(format!("PM-{}   /   {team}", i + 1)),
                                            )
                                            .child(title)
                                            .on_click(cx.listener(move |s, _, _, cx| {
                                                s.selected = Some(i);
                                                cx.notify();
                                            }))
                                    }),
                            )
                    })),
            )
            .when_some(self.selected, |s, i| {
                s.child(
                    card::root("inspector", "Work item details")
                        .p_3()
                        .border_1()
                        .border_color(rgb(p.border))
                        .rounded_md()
                        .child(
                            card::title("task-title", TASKS[i].0)
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(TASKS[i].0),
                        )
                        .child(div().text_color(rgb(p.muted)).child("Move to"))
                        .child(
                            toggle_group::root("status", "Task status")
                                .flex()
                                .flex_wrap()
                                .gap_1()
                                .children(LANES.iter().enumerate().map(|(lane, &label)| {
                                    let active = self.lanes[i] == lane;
                                    toggle_group::item(("status", lane), label, active)
                                        .min_h(h)
                                        .flex()
                                        .items_center()
                                        .px_2()
                                        .rounded_md()
                                        .when(active, |s| s.bg(rgb(p.selected)))
                                        .focus_visible(|s| s.underline())
                                        .child(label)
                                        .on_click(cx.listener(move |s, _, _, cx| {
                                            s.lanes[i] = lane;
                                            cx.notify();
                                        }))
                                })),
                        )
                        .child(
                            div()
                                .py_2()
                                .child(format!("Due · {}", self.due[i].format("%B %-d, %Y"))),
                        )
                        .child(ui::month_picker(
                            &self.grid,
                            self.due[i],
                            p,
                            h,
                            |_| false,
                            cx.listener(move |s, date, _, cx| {
                                s.due[i] = *date;
                                cx.notify();
                            }),
                        ))
                        .child(ui::action("Close details", w, cx, |s: &mut Board| {
                            s.selected = None
                        })),
                )
            })
    }
}
