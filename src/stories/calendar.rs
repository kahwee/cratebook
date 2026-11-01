use super::Story;
use crate::ui;
use chrono::{Datelike, Days, Months, NaiveDate};
use gpui::{prelude::*, *};
use gpui_jig::{button, card, toolbar};
use std::collections::BTreeSet;

struct Calendar {
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
            .new(|_| {
                let selected = NaiveDate::from_ymd_opt(2026, 10, 12).unwrap();
                Calendar {
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
        let first = self.selected.with_day(1).unwrap();
        let offset = first.weekday().num_days_from_sunday();
        let h = ui::control_height(w);
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .child(
                toolbar::root("calendar-nav", "Month navigation")
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(first.format("%B %Y").to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(ui::action("Previous", w, cx, |s: &mut Calendar| {
                                if let Some(d) = s.selected.checked_sub_months(Months::new(1)) {
                                    s.selected = d;
                                }
                            }))
                            .child(ui::action("Next", w, cx, |s: &mut Calendar| {
                                if let Some(d) = s.selected.checked_add_months(Months::new(1)) {
                                    s.selected = d;
                                }
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .children(["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map(|day| {
                        div()
                            .flex_1()
                            .text_size(px(12.))
                            .text_color(rgb(p.muted))
                            .child(day)
                    })),
            )
            .children((0u32..6).map(|row| {
                div().flex().children((0..7).map(|col| {
                    let index = row * 7 + col;
                    let date = index
                        .checked_sub(offset)
                        .and_then(|n| first.checked_add_days(Days::new(n as u64)))
                        .filter(|d| d.month() == first.month());
                    div()
                        .flex_1()
                        .min_w_0()
                        .border_t_1()
                        .border_color(rgb(p.border))
                        .when_some(date, |cell, date| {
                            cell.child(
                                button::root(("day", index as usize), date.to_string())
                                    .min_h(h)
                                    .p_1()
                                    .rounded_md()
                                    .when(date == self.selected, |s| s.bg(rgb(p.selected)))
                                    .focus_visible(|s| s.underline())
                                    .child(date.day().to_string())
                                    .when(self.focus_days.contains(&date), |s| {
                                        s.child(div().size(px(5.)).rounded_full().bg(rgb(p.text)))
                                    })
                                    .on_click(cx.listener(move |s, _, _, cx| {
                                        s.selected = date;
                                        cx.notify();
                                    })),
                            )
                        })
                }))
            }))
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
