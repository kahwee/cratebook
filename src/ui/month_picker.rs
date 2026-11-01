//! Gallery styling for Jig's unstyled date grid, shared by Calendar and Project board.
use super::Palette;
use chrono::{Datelike, NaiveDate, Weekday};
use gpui::{prelude::*, *};
use gpui_jig::date_grid::{self, DateGrid, Month};
use std::rc::Rc;

pub(crate) fn month_picker(
    grid: &DateGrid,
    selected: NaiveDate,
    p: Palette,
    height: Pixels,
    marked: impl Fn(NaiveDate) -> bool,
    change: impl Fn(&NaiveDate, &mut Window, &mut App) + 'static,
) -> Div {
    let change = Rc::new(change);
    let keyboard = change.clone();
    let heading = selected.format("%B %Y").to_string();
    let toolbar = gpui_jig::toolbar::root("calendar-nav", "Month navigation")
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(px(20.))
                .font_weight(FontWeight::SEMIBOLD)
                .child(heading.clone()),
        )
        .child(
            div().flex().gap_1().children(
                [("Previous", "pageup"), ("Next", "pagedown")]
                    .into_iter()
                    .filter_map(|(label, key)| {
                        let next = date_grid::navigate(selected, Weekday::Sun, key)?;
                        let click = change.clone();
                        Some(
                            gpui_jig::button::root(label, label)
                                .min_h(height)
                                .flex()
                                .items_center()
                                .px_2()
                                .rounded_md()
                                .border_1()
                                .border_color(rgb(p.border))
                                .hover(move |s| s.bg(rgb(p.selected)))
                                .focus_visible(|s| s.underline())
                                .on_click(move |_, w, cx| click(&next, w, cx))
                                .child(label),
                        )
                    }),
            ),
        );
    let body = grid
        .root(
            "month-grid",
            heading,
            selected,
            Weekday::Sun,
            move |d, w, cx| keyboard(d, w, cx),
        )
        .flex()
        .flex_col()
        .child(
            date_grid::row("weekdays").flex().children(
                ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| {
                        div()
                            .id(("weekday", i))
                            .role(Role::ColumnHeader)
                            .flex_1()
                            .text_size(px(12.))
                            .text_color(rgb(p.muted))
                            .child(name)
                    }),
            ),
        )
        .children(
            Month::new(selected, Weekday::Sun)
                .weeks()
                .enumerate()
                .map(|(i, week)| {
                    date_grid::row(("week", i))
                        .flex()
                        .children(week.into_iter().map(|day| {
                            div()
                                .flex_1()
                                .min_w_0()
                                .border_t_1()
                                .border_color(rgb(p.border))
                                .when_some(day, |cell, date| {
                                    let click = change.clone();
                                    cell.child(
                                        grid.day(
                                            date,
                                            selected,
                                            date.to_string(),
                                            move |d, w, cx| click(d, w, cx),
                                        )
                                        .min_h(height)
                                        .p_1()
                                        .rounded_md()
                                        .focus_visible(|s| s.underline())
                                        .when(date == selected, |s| s.bg(rgb(p.selected)))
                                        .child(date.day().to_string())
                                        .when(
                                            marked(date),
                                            |s| {
                                                s.child(
                                                    div()
                                                        .size(px(5.))
                                                        .rounded_full()
                                                        .bg(rgb(p.text)),
                                                )
                                            },
                                        ),
                                    )
                                })
                        }))
                }),
        );
    div().flex().flex_col().gap_2().child(toolbar).child(body)
}
