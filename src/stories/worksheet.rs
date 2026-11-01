//! A small sales workbook. Every chart reads from the cells displayed above it.
use super::{Story, workbench};
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{button, card, scroll_area, toolbar};

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const REGIONS: [&str; 4] = ["North", "South", "East", "West"];
const GREEN: u32 = 0x198358;

struct Worksheet {
    sales: [[u16; 6]; 4],
    row: usize,
    col: usize,
    draft: String,
}

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Sales worksheet",
        description: "Select a monthly sales cell, edit its value, and apply the change to the workbook and charts.",
        path: file!(),
        source: include_str!("worksheet.rs"),
        preview: cx
            .new(|_| Worksheet {
                sales: [
                    [32, 38, 35, 47, 51, 64],
                    [54, 48, 53, 42, 44, 39],
                    [21, 25, 37, 31, 46, 52],
                    [41, 35, 43, 48, 44, 59],
                ],
                row: 0,
                col: 0,
                draft: "32".into(),
            })
            .into(),
    }
}

fn line(values: [u16; 6], ceiling: u16, color: u32) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let mut path = PathBuilder::stroke(px(1.8));
            for (i, value) in values.into_iter().enumerate() {
                let x = px(2.) + (bounds.size.width - px(4.)) * (i as f32 / 5.);
                let y = px(2.)
                    + (bounds.size.height - px(4.))
                        * (1. - f32::from(value) / f32::from(ceiling.max(1)));
                let at = bounds.origin + point(x, y);
                if i == 0 {
                    path.move_to(at)
                } else {
                    path.line_to(at)
                }
            }
            if let Ok(path) = path.build() {
                window.paint_path(path, rgb(color));
            }
        },
    )
    .size_full()
}
fn label(text: impl Into<SharedString>, width: f32, p: ui::Palette) -> Div {
    div()
        .w(px(width))
        .when(width == 53., |s| s.flex_grow(1.))
        .flex_shrink_0()
        .px_2()
        .h_full()
        .flex()
        .items_center()
        .border_r_1()
        .border_color(rgb(p.border))
        .child(text.into())
}
impl Render for Worksheet {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let compact = ui::compact_layout(w);
        let height = ui::control_height(w);
        let accent = if *cx.global::<ui::Palette>() == ui::Palette::DARK {
            0x72dba6
        } else {
            GREEN
        };
        let max = self
            .sales
            .iter()
            .flatten()
            .copied()
            .max()
            .unwrap_or(1)
            .max(1);
        let totals = self
            .sales
            .map(|row| row.into_iter().map(u32::from).sum::<u32>());
        let largest = totals.into_iter().max().unwrap_or(1).max(1);
        let addr = format!("{}{}", (b'B' + self.col as u8) as char, self.row + 2);
        let parsed = self.draft.trim().parse::<u16>().ok();
        let grid = div()
            .w_full()
            .min_w(px(518.))
            .text_size(px(12.))
            .child(
                div()
                    .h(px(24.))
                    .flex()
                    .bg(rgb(p.sidebar))
                    .child(label("", 28., p))
                    .child(label("A", 84., p))
                    .children(["B", "C", "D", "E", "F", "G"].map(|v| label(v, 53., p)))
                    .child(label("H", 88., p)),
            )
            .child(
                div()
                    .h(height)
                    .flex()
                    .border_y_1()
                    .border_color(rgb(p.border))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(label("1", 28., p))
                    .child(label("Region", 84., p))
                    .children(MONTHS.map(|m| label(m, 53., p)))
                    .child(label("Trend", 88., p)),
            )
            .children(self.sales.iter().enumerate().map(|(r, values)| {
                div()
                    .flex()
                    .h(height)
                    .border_b_1()
                    .border_color(rgb(p.border))
                    .child(label((r + 2).to_string(), 28., p).bg(rgb(p.sidebar)))
                    .child(
                        label(REGIONS[r], 84., p)
                            .when(self.row == r, |s| s.font_weight(FontWeight::SEMIBOLD)),
                    )
                    .children(values.iter().enumerate().map(|(c, &value)| {
                        button::root(
                            ("cell", r * 6 + c),
                            format!("{} {}, {value} thousand dollars", REGIONS[r], MONTHS[c]),
                        )
                        .w(px(53.))
                        .flex_grow(1.)
                        .h_full()
                        .px_2()
                        .flex()
                        .items_center()
                        .justify_end()
                        .border_r_1()
                        .border_color(rgb(p.border))
                        .when((r, c) == (self.row, self.col), |s| {
                            s.bg(rgb(p.selected)).border_2().border_color(rgb(accent))
                        })
                        .focus_visible(|s| s.underline())
                        .child(value.to_string())
                        .on_click(cx.listener(move |s, _, _, cx| {
                            s.row = r;
                            s.col = c;
                            s.draft = s.sales[r][c].to_string();
                            cx.notify();
                        }))
                    }))
                    .child(
                        div()
                            .w(px(88.))
                            .px_2()
                            .py_1()
                            .child(line(*values, max, accent)),
                    )
            }));
        let chart = |id: &'static str, title: &'static str| {
            card::root(id, title)
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap_2()
                .p_3()
                .rounded(px(p.corners.card as f32))
                .border_1()
                .border_color(rgb(p.border))
                .child(
                    card::title("heading", title)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title),
                )
        };
        div()
            .flex()
            .flex_col()
            .child(
                toolbar::root("workbook", "Sales workbook")
                    .bg(rgb(0x176344))
                    .text_color(rgb(0xffffff))
                    .px_3()
                    .py_2()
                    .child("Sales workbook   /   First half"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .flex_wrap()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .child(addr)
                    .child(div().text_color(rgb(p.muted)).child("ƒx"))
                    .child(
                        workbench::entry("value", "Sales in thousands", &self.draft, w, cx, |s| {
                            &mut s.draft
                        })
                        .w(px(80.)),
                    )
                    .when_some(parsed, |row, n| {
                        row.child(ui::action("Apply", w, cx, move |s: &mut Worksheet| {
                            s.sales[s.row][s.col] = n
                        }))
                    })
                    .when(parsed.is_none(), |row| row.child("Enter 0–65535")),
            )
            .child(
                scroll_area::viewport("sheet-scroll")
                    .w_full()
                    .overflow_y_hidden()
                    .child(grid),
            )
            .child(
                div()
                    .flex()
                    .when(compact, |s| s.flex_col())
                    .gap_2()
                    .p_3()
                    .child(
                        chart("totals", "Sales by region")
                            .child(
                                div()
                                    .text_color(rgb(p.muted))
                                    .child("Six-month totals · $k"),
                            )
                            .children(totals.into_iter().enumerate().map(|(i, total)| {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().w(px(42.)).child(REGIONS[i]))
                                    .child(
                                        div().flex_1().h(px(8.)).bg(rgb(p.sidebar)).child(
                                            div()
                                                .h_full()
                                                .w(relative(total as f32 / largest as f32))
                                                .bg(rgb(accent)),
                                        ),
                                    )
                                    .child(total.to_string())
                            })),
                    )
                    .child(
                        chart("trend", "Monthly sales")
                            .child(format!("{} · $k · 0–{max}", REGIONS[self.row]))
                            .child(
                                div()
                                    .h(px(72.))
                                    .border_b_1()
                                    .border_color(rgb(p.border))
                                    .child(line(self.sales[self.row], max, accent)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_size(px(12.))
                                    .text_color(rgb(p.muted))
                                    .children(MONTHS.map(|m| div().child(m))),
                            ),
                    ),
            )
    }
}
