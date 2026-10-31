use super::Story;
use crate::{
    table::{ColumnDef, Direction, Sorting},
    ui,
};
use gpui::{prelude::*, *};

struct Package {
    name: &'static str,
    builds: u32,
}

struct Packages {
    columns: Vec<ColumnDef<Package>>,
    sorting: Sorting,
    rows: Vec<Package>,
}

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Data table",
        description: "Choose a column to sort. Choose it again to reverse, then once more to restore the original order.",
        path: file!(),
        source: include_str!("table.rs"),
        preview: cx
            .new(|_| Packages {
                columns: vec![
                    ColumnDef::accessor("Package", |p: &Package| p.name),
                    ColumnDef::accessor("Builds", |p: &Package| &p.builds),
                ],
                sorting: Sorting::default(),
                rows: vec![
                    Package {
                        name: "Canvas",
                        builds: 12,
                    },
                    Package {
                        name: "Foundation",
                        builds: 3,
                    },
                    Package {
                        name: "Studio",
                        builds: 24,
                    },
                    Package {
                        name: "Atlas",
                        builds: 8,
                    },
                ],
            })
            .into(),
    }
}

impl Render for Packages {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = ui::preview_palette(cx);
        let headers = self.columns.iter().enumerate().map(|(index, column)| {
            let order = match self.sorting.direction(index) {
                Some(Direction::Ascending) => " (asc)",
                Some(Direction::Descending) => " (desc)",
                None => "",
            };
            let label = format!("{}{order}", column.header);
            div()
                .id(("head", index))
                .role(Role::ColumnHeader)
                .flex_1()
                .child(
                    ui::button(("sort", index), format!("Sort by {label}"))
                        .w_full()
                        .h(ui::control_height(window))
                        .px_3()
                        .flex()
                        .items_center()
                        .font_weight(FontWeight::SEMIBOLD)
                        .focus_visible(|s| s.underline())
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.sorting.toggle(index);
                            cx.notify();
                        }))
                        .child(label),
                )
        });
        div()
            .id("packages")
            .role(Role::Table)
            .aria_label("Package builds")
            .text_size(px(14.))
            .child(
                div()
                    .id("headers")
                    .role(Role::Row)
                    .flex()
                    .bg(rgb(palette.sidebar))
                    .children(headers),
            )
            .children(
                self.sorting
                    .rows(&self.rows, &self.columns)
                    .into_iter()
                    .map(|row| {
                        div()
                            .id(row.name)
                            .role(Role::Row)
                            .flex()
                            .border_t_1()
                            .border_color(rgb(palette.border))
                            .children(self.columns.iter().map(|column| {
                                let value = column.cell(row);
                                div()
                                    .id(column.header)
                                    .role(Role::Cell)
                                    .aria_label(value.clone())
                                    .flex_1()
                                    .px_3()
                                    .py_2()
                                    .child(value)
                            }))
                    }),
            )
    }
}
