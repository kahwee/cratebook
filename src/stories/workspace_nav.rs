use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{menu, menubar, navigation_menu, separator, toolbar};
use std::{cell::Cell, rc::Rc};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Workspace navigation",
        description: "Switch project sections and use the View menu to change the toolbar density.",
        path: file!(),
        source: include_str!("workspace_nav.rs"),
        preview: cx
            .new(|cx| Preview {
                section: 0,
                menu_open: crate::capture_mode(),
                dense: false,
                menu_focus: cx.focus_handle(),
                opener: cx.focus_handle(),
                menu_anchor: Rc::new(Cell::new(Bounds::default())),
            })
            .into(),
    }
}
struct Preview {
    section: usize,
    menu_open: bool,
    dense: bool,
    menu_focus: FocusHandle,
    opener: FocusHandle,
    menu_anchor: Rc<Cell<Bounds<Pixels>>>,
}
impl Preview {
    fn close_menu(&mut self, w: &mut Window, cx: &mut Context<Self>) {
        self.menu_open = false;
        w.focus(&self.opener, cx);
        cx.notify();
    }
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let sections = ["Overview", "Documents", "Activity"];
        let menu_anchor = self.menu_anchor.clone();
        let entity_id = cx.entity_id();
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                menubar::root("bar", "Application menu")
                    .relative()
                    .flex()
                    .items_start()
                    .child(
                        div()
                            .child(
                                menu::trigger("view", "View", self.menu_open)
                                    .track_focus(&self.opener)
                                    .min_h(ui::control_height(w))
                                    .flex()
                                    .items_center()
                                    .px_2()
                                    .rounded(px(p.corners.control as f32))
                                    .bg(rgb(p.sidebar))
                                    .hover(move |s| s.bg(rgb(p.selected)))
                                    .focus_visible(|s| s.underline())
                                    .child("View ▾")
                                    .on_click(cx.listener(|s, _, w, cx| {
                                        s.menu_open = !s.menu_open;
                                        if s.menu_open {
                                            w.focus(&s.menu_focus, cx);
                                        } else {
                                            w.focus(&s.opener, cx);
                                        }
                                        cx.notify();
                                    }))
                                    .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                                        if e.keystroke.key == "down" {
                                            s.menu_open = true;
                                            w.focus(&s.menu_focus, cx);
                                            w.prevent_default();
                                            cx.stop_propagation();
                                            cx.notify();
                                        }
                                    })),
                            )
                            .on_children_prepainted(move |bounds, _, cx| {
                                if let Some(bounds) = bounds.first() {
                                    let anchor = *bounds;
                                    if menu_anchor.get() != anchor {
                                        menu_anchor.set(anchor);
                                        cx.notify(entity_id);
                                    }
                                }
                            }),
                    )
                    .when(self.menu_open, |s| {
                        s.child(deferred(
                            anchored()
                                .position(
                                    self.menu_anchor.get().bottom_left() + point(px(0.), px(4.)),
                                )
                                .snap_to_window_with_margin(px(8.))
                                .child(
                                    menu::popup("view-menu", "View options")
                                        .track_focus(&self.menu_focus)
                                        .w(px(200.))
                                        .p_1()
                                        .rounded(px(p.corners.control as f32))
                                        .border_1()
                                        .border_color(rgb(p.border))
                                        .bg(rgb(p.canvas))
                                        .shadow_lg()
                                        .occlude()
                                        .on_mouse_down_out(cx.listener(
                                            |s, event: &MouseDownEvent, w, cx| {
                                                if !s.menu_anchor.get().contains(&event.position) {
                                                    s.close_menu(w, cx);
                                                }
                                            },
                                        ))
                                        .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                                            match e.keystroke.key.as_str() {
                                                "escape" | "tab" => s.close_menu(w, cx),
                                                "enter" | "space" => {
                                                    s.dense = !s.dense;
                                                    s.close_menu(w, cx);
                                                }
                                                "up" | "down" | "home" | "end" => {}
                                                _ => return,
                                            }
                                            w.prevent_default();
                                            cx.stop_propagation();
                                        }))
                                        .child(
                                            menu::group_label("layout", "Toolbar layout")
                                                .h(px(28.))
                                                .px_2()
                                                .flex()
                                                .items_center()
                                                .text_size(px(12.))
                                                .text_color(rgb(p.muted))
                                                .child("Toolbar layout"),
                                        )
                                        .child(
                                            menu::separator()
                                                .h(px(1.))
                                                .mx_1()
                                                .my_1()
                                                .bg(rgb(p.border)),
                                        )
                                        .child(
                                            menu::checkbox_item(
                                                "density",
                                                "Compact spacing",
                                                self.dense,
                                            )
                                            .tab_stop(false)
                                            .h(px(if ui::compact_layout(w) { 44. } else { 32. }))
                                            .px_2()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .rounded(px(gpui_jig::corners::Corners::inset(
                                                p.corners.control,
                                                3,
                                            )
                                                as f32))
                                            .hover(move |s| s.bg(rgb(p.selected)))
                                            .child("Compact spacing")
                                            .child(if self.dense { "✓" } else { "" })
                                            .on_click(
                                                cx.listener(|s, _, w, cx| {
                                                    s.dense = !s.dense;
                                                    s.close_menu(w, cx);
                                                }),
                                            ),
                                        ),
                                ),
                        ))
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
