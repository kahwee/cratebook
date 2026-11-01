use super::Story;
use crate::ui;
use chrono::{Days, NaiveDate};
use gpui::{prelude::*, *};
use gpui_jig::{card, date_grid::DateGrid};
use std::collections::BTreeSet;

struct Calendar {
    grid: DateGrid,
    selected: NaiveDate,
    focus_days: BTreeSet<NaiveDate>,
}
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Calendar",
        description: "Browse a month, select a day, and schedule a focus block in its agenda.",
        path: file!(),
        source: include_str!("calendar.rs"),
        preview: cx
            .new(|cx| {
                let selected = NaiveDate::from_ymd_opt(2026, 10, 12).unwrap();
                Calendar {
                    grid: DateGrid::new(cx),
                    selected,
                    focus_days: BTreeSet::from([selected, selected + Days::new(3)]),
                }
            })
            .into(),
    }
}
impl Render for Calendar {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let h = ui::control_height(w);
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .child(ui::month_picker(
                &self.grid,
                self.selected,
                p,
                h,
                |date| self.focus_days.contains(&date),
                cx.listener(|s, date, _, cx| {
                    s.selected = *date;
                    cx.notify();
                }),
            ))
            .child(
                card::root("agenda", "Day agenda")
                    .rounded_lg()
                    .bg(rgb(p.sidebar))
                    .p_3()
                    .child(
                        card::title("agenda-title", "Selected date")
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(self.selected.format("%A, %B %-d").to_string()),
                    )
                    .when(self.focus_days.contains(&self.selected), |s| {
                        s.child("09:00–10:00   Focus block").child(ui::action(
                            "Remove block",
                            w,
                            cx,
                            |s: &mut Calendar| {
                                s.focus_days.remove(&s.selected);
                            },
                        ))
                    })
                    .when(!self.focus_days.contains(&self.selected), |s| {
                        s.child("No events scheduled").child(ui::action(
                            "Add focus block",
                            w,
                            cx,
                            |s: &mut Calendar| {
                                s.focus_days.insert(s.selected);
                            },
                        ))
                    }),
            )
    }
}
