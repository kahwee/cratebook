use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{menu, menubar, navigation_menu, separator, toolbar};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Workspace navigation",
        description: "Switch project sections and use the View menu to change the toolbar density.",
        path: file!(),
        source: include_str!("workspace_nav.rs"),
        preview: cx
            .new(|_| Preview {
                section: 0,
                menu_open: false,
                dense: false,
            })
            .into(),
    }
}
struct Preview {
    section: usize,
    menu_open: bool,
    dense: bool,
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let sections = ["Overview", "Documents", "Activity"];
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                menubar::root("bar", "Application menu")
                    .child(
                        menu::trigger("view", "View", self.menu_open)
                            .min_h(ui::control_height(w))
                            .flex()
                            .items_center()
                            .child("View ▾")
                            .on_click(cx.listener(|s, _, _, cx| {
                                s.menu_open = !s.menu_open;
                                cx.notify();
                            })),
                    )
                    .when(self.menu_open, |s| {
                        s.child(
                            menu::popup("view-menu", "View options")
                                .p_2()
                                .bg(rgb(p.sidebar))
                                .child(
                                    menu::checkbox_item("density", "Compact spacing", self.dense)
                                        .child(if self.dense {
                                            "Compact spacing: on"
                                        } else {
                                            "Compact spacing: off"
                                        })
                                        .on_click(cx.listener(|s, _, _, cx| {
                                            s.dense = !s.dense;
                                            s.menu_open = false;
                                            cx.notify();
                                        })),
                                ),
                        )
                    }),
            )
            .child(
                toolbar::root("project-tools", "Project toolbar").child(
                    navigation_menu::root("sections", "Project sections")
                        .flex()
                        .flex_wrap()
                        .children(sections.into_iter().enumerate().map(|(i, title)| {
                            navigation_menu::link(("section", i), title)
                                .min_h(ui::control_height(w))
                                .flex()
                                .items_center()
                                .px(if self.dense { px(5.) } else { px(12.) })
                                .rounded(px(p.corners.control as f32))
                                .when(self.section == i, |s| s.bg(rgb(p.selected)))
                                .focus_visible(|s| s.underline())
                                .child(title)
                                .on_click(cx.listener(move |s, _, _, cx| {
                                    s.section = i;
                                    cx.notify();
                                }))
                        })),
                ),
            )
            .child(
                separator::root("divider", "Section divider")
                    .h(px(1.))
                    .bg(rgb(p.border)),
            )
            .child(
                [
                    "Project overview · 3 documents",
                    "Field notes.md · Roadmap.md · Release.md",
                    "Most recent change: Roadmap updated",
                ][self.section],
            )
    }
}
