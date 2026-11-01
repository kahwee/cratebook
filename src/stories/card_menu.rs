use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{card, context_menu as menu};
use std::{cell::Cell, rc::Rc};

const MENU_WIDTH: f32 = 200.;
const MENU_GAP: f32 = 6.;
const MENU_MARGIN: f32 = 8.;

struct MenuPlacement {
    origin: Point<Pixels>,
    size: Size<Pixels>,
    parent: Point<Pixels>,
    child: Point<Pixels>,
}

// Position each panel independently so opening a submenu never shifts its parent.
fn menu_placement(
    at: Point<Pixels>,
    viewport: Size<Pixels>,
    row_height: Pixels,
    nested: bool,
) -> MenuPlacement {
    let width = f32::from(viewport.width);
    let height = f32::from(viewport.height);
    let row = f32::from(row_height);
    let menu_height = 10. + 28. + 9. + row * 2.;
    let max_x = (width - MENU_MARGIN - MENU_WIDTH).max(MENU_MARGIN);
    let max_y = (height - MENU_MARGIN - menu_height).max(MENU_MARGIN);
    let x = f32::from(at.x).clamp(MENU_MARGIN, max_x);
    let y = f32::from(at.y).clamp(MENU_MARGIN, max_y);
    let right = x + MENU_WIDTH + MENU_GAP;
    let left = x - MENU_WIDTH - MENU_GAP;
    let child_x = if right + MENU_WIDTH <= width - MENU_MARGIN {
        right
    } else if left >= MENU_MARGIN {
        left
    } else {
        right.clamp(MENU_MARGIN, max_x)
    };
    let child_y = (y + row).clamp(MENU_MARGIN, max_y);
    let origin_x = if nested { x.min(child_x) } else { x };
    let origin_y = if nested { y.min(child_y) } else { y };
    MenuPlacement {
        origin: point(px(origin_x), px(origin_y)),
        size: size(
            px(if nested {
                x.max(child_x) + MENU_WIDTH - origin_x
            } else {
                MENU_WIDTH
            }),
            px(if nested {
                y.max(child_y) + menu_height - origin_y
            } else {
                menu_height
            }),
        ),
        parent: point(px(x - origin_x), px(y - origin_y)),
        child: point(px(child_x - origin_x), px(child_y - origin_y)),
    }
}

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Card & context menu",
        description: "Right-click the project card or choose Actions. Move to opens a submenu.",
        path: file!(),
        source: include_str!("card_menu.rs"),
        preview: cx
            .new(|cx| Project {
                position: crate::capture_mode().then(|| point(px(320.), px(300.))),
                nested: crate::capture_mode(),
                pinned: false,
                folder: "Inbox",
                focus: cx.focus_handle(),
                opener: cx.focus_handle(),
                opener_bounds: Rc::new(Cell::new(Bounds::default())),
                active: 0,
            })
            .into(),
    }
}
#[derive(Clone, Copy)]
enum MenuIcon {
    Pin,
    Chevron,
    Inbox,
    Archive,
    Check,
}

impl MenuIcon {
    fn draw(self, color: u32) -> impl IntoElement {
        let strokes: &[&[(f32, f32)]] = match self {
            Self::Pin => &[
                &[
                    (4., 2.),
                    (10., 2.),
                    (9., 6.),
                    (11., 8.),
                    (3., 8.),
                    (5., 6.),
                    (4., 2.),
                ],
                &[(7., 8.), (7., 13.)],
            ],
            Self::Chevron => &[&[(5., 3.), (9., 7.), (5., 11.)]],
            Self::Inbox => &[&[
                (2., 9.),
                (4., 3.),
                (10., 3.),
                (12., 9.),
                (12., 12.),
                (2., 12.),
                (2., 9.),
                (5., 9.),
                (6., 10.),
                (8., 10.),
                (9., 9.),
                (12., 9.),
            ]],
            Self::Archive => &[
                &[(2., 2.), (12., 2.), (12., 5.), (2., 5.), (2., 2.)],
                &[(3., 5.), (3., 12.), (11., 12.), (11., 5.)],
                &[(6., 7.), (8., 7.)],
            ],
            Self::Check => &[&[(3., 7.), (6., 10.), (11., 4.)]],
        };
        canvas(
            |_, _, _| (),
            move |bounds, _, window, _| {
                let mut path = PathBuilder::stroke(px(1.3));
                for stroke in strokes {
                    for (i, &(x, y)) in stroke.iter().enumerate() {
                        let p = bounds.origin + point(px(x), px(y));
                        if i == 0 {
                            path.move_to(p);
                        } else {
                            path.line_to(p);
                        }
                    }
                }
                if let Ok(path) = path.build() {
                    window.paint_path(path, rgb(color));
                }
            },
        )
        .size(px(14.))
    }
}

struct Project {
    position: Option<Point<Pixels>>,
    nested: bool,
    pinned: bool,
    folder: &'static str,
    focus: FocusHandle,
    opener: FocusHandle,
    opener_bounds: Rc<Cell<Bounds<Pixels>>>,
    active: usize,
}
impl Project {
    fn open(&mut self, at: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.position = Some(at);
        self.nested = false;
        self.active = 0;
        window.focus(&self.focus, cx);
        cx.notify();
    }
    fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.position = None;
        window.focus(&self.opener, cx);
        cx.notify();
    }
    fn choose(&mut self, i: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.nested {
            self.folder = ["Inbox", "Archive"][i];
        } else if i == 1 {
            self.nested = true;
            self.active = 0;
            cx.notify();
            return;
        } else {
            self.pinned = !self.pinned;
        }
        self.close(window, cx);
    }
    fn popup(
        &self,
        nested: bool,
        p: ui::Palette,
        row_height: Pixels,
        cx: &Context<Self>,
    ) -> impl IntoElement + use<> {
        let entries = if nested {
            [(MenuIcon::Inbox, "Inbox"), (MenuIcon::Archive, "Archive")]
        } else {
            [
                (MenuIcon::Pin, if self.pinned { "Unpin" } else { "Pin" }),
                (MenuIcon::Inbox, "Move to"),
            ]
        };
        menu::popup(
            if nested { "destinations" } else { "commands" },
            if nested {
                "Destinations"
            } else {
                "Project actions"
            },
        )
        .w(px(MENU_WIDTH))
        .p_1()
        .rounded(px(p.corners.control as f32))
        .border_1()
        .border_color(rgb(p.border))
        .bg(rgb(p.canvas))
        .shadow_lg()
        .child(
            menu::group_label(
                "caption",
                if nested {
                    "Destination"
                } else {
                    "Field notes.md"
                },
            )
            .h(px(28.))
            .px_2()
            .flex()
            .items_center()
            .text_size(px(12.))
            .text_color(rgb(p.muted))
            .child(if nested { "Move to" } else { "Field notes.md" }),
        )
        .child(menu::separator().h(px(1.)).mx_1().my_1().bg(rgb(p.border)))
        .children(entries.into_iter().enumerate().map(|(i, (icon, label))| {
            let entry = if nested {
                menu::radio_item(("folder", i), label, self.folder == label)
            } else if i == 1 {
                menu::submenu_trigger("move", label, self.nested)
            } else {
                menu::checkbox_item("pin", label, self.pinned)
            };
            entry
                .tab_stop(false)
                .px_2()
                .h(row_height)
                .rounded(px(
                    gpui_jig::corners::Corners::inset(p.corners.control, 3) as f32
                ))
                .flex()
                .items_center()
                .gap_2()
                .when(
                    (self.nested == nested && self.active == i)
                        || (!nested && self.nested && i == 1),
                    |s| s.bg(rgb(p.selected)),
                )
                .hover(move |s| s.bg(rgb(p.selected)))
                .on_hover(cx.listener(move |s, hovered: &bool, _, cx| {
                    if !hovered {
                        return;
                    }
                    let open_child = nested || i == 1;
                    let active = if !nested && i == 1 { 0 } else { i };
                    if s.nested != open_child || s.active != active {
                        s.nested = open_child;
                        s.active = active;
                        cx.notify();
                    }
                }))
                .on_click(cx.listener(move |s, _, w, cx| {
                    s.nested = nested;
                    s.choose(i, w, cx);
                }))
                .child(icon.draw(p.muted))
                .child(div().flex_1().child(label))
                .when(!nested && i == 1, |s| {
                    s.child(MenuIcon::Chevron.draw(p.muted))
                })
                .when(
                    (nested && self.folder == label) || (!nested && i == 0 && self.pinned),
                    |s| s.child(MenuIcon::Check.draw(p.text)),
                )
        }))
    }
}
impl Render for Project {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let actions = ui::button("actions", "Project actions")
            .track_focus(&self.opener)
            .aria_expanded(self.position.is_some())
            .min_h(ui::control_height(window))
            .px_2()
            .rounded(px(p.corners.control as f32))
            .bg(rgb(p.sidebar))
            .flex()
            .items_center()
            .focus_visible(|s| s.underline())
            .on_click(cx.listener(|s, _: &ClickEvent, w, cx| {
                let at = s.opener_bounds.get().bottom_left() + point(px(0.), px(4.));
                s.open(at, w, cx)
            }))
            .child("Actions");
        let opener_bounds = self.opener_bounds.clone();
        let actions = div()
            .child(actions)
            .on_children_prepainted(move |bounds, _, _| {
                if let Some(bounds) = bounds.first() {
                    opener_bounds.set(*bounds);
                }
            });
        let target = menu::trigger(
            "project-target",
            "Project card",
            cx.listener(|s, e: &MouseDownEvent, w, cx| s.open(e.position, w, cx)),
        )
        .focusable()
        .tab_stop(true)
        .on_key_down(cx.listener(|s, event: &KeyDownEvent, w, cx| {
            if event.keystroke.key == "menu"
                || (event.keystroke.key == "f10" && event.keystroke.modifiers.shift)
            {
                let at = s.opener_bounds.get().bottom_left() + point(px(0.), px(4.));
                s.open(at, w, cx);
                w.prevent_default();
                cx.stop_propagation();
            }
        }))
        .child(
            card::root("project", "Field notes")
                .border_1()
                .border_color(rgb(p.border))
                .rounded(px(p.corners.card as f32))
                .p_4()
                .child(
                    card::title("project-title", "Field notes")
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("Field notes"),
                )
                .child(
                    card::content()
                        .mt_2()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(format!(
                            "{}{}",
                            self.folder,
                            if self.pinned { " · Pinned" } else { "" }
                        ))
                        .child(actions),
                ),
        );
        menu::root()
            .p_4()
            .child(target)
            .when_some(self.position, |s, at| {
                let row_height = px(if ui::compact_layout(window) { 44. } else { 32. });
                let placement = menu_placement(at, window.viewport_size(), row_height, self.nested);
                let popup = div()
                    .id("menu-layer")
                    .track_focus(&self.focus)
                    .relative()
                    .w(placement.size.width)
                    .h(placement.size.height)
                    .occlude()
                    .on_mouse_down_out(cx.listener(|s, _, w, cx| {
                        s.close(w, cx);
                    }))
                    .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                        match e.keystroke.key.as_str() {
                            "escape" | "left" if s.nested => {
                                s.nested = false;
                                s.active = 1;
                                cx.notify();
                            }
                            "escape" | "tab" => s.close(w, cx),
                            "up" | "down" => {
                                s.active = 1 - s.active;
                                cx.notify();
                            }
                            "home" => {
                                s.active = 0;
                                cx.notify();
                            }
                            "end" => {
                                s.active = 1;
                                cx.notify();
                            }
                            "right" if !s.nested && s.active == 1 => s.choose(1, w, cx),
                            "enter" | "space" => s.choose(s.active, w, cx),
                            _ => return,
                        }
                        w.prevent_default();
                        cx.stop_propagation();
                    }))
                    .child(
                        div()
                            .absolute()
                            .left(placement.parent.x)
                            .top(placement.parent.y)
                            .child(self.popup(false, p, row_height, cx)),
                    )
                    .when(self.nested, |s| {
                        s.child(
                            menu::submenu_root()
                                .absolute()
                                .left(placement.child.x)
                                .top(placement.child.y)
                                .child(self.popup(true, p, row_height, cx)),
                        )
                    });
                s.child(deferred(anchored().position(placement.origin).child(popup)))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{MENU_MARGIN, menu_placement};
    use gpui::{point, px, size};

    #[test]
    fn submenus_stay_in_the_viewport_without_moving_the_parent() {
        for (width, height, x, y) in [
            (1120., 720., 420., 300.),
            (1120., 720., 1110., 710.),
            (360., 480., 340., 460.),
        ] {
            let viewport = size(px(width), px(height));
            let at = point(px(x), px(y));
            let closed = menu_placement(at, viewport, px(32.), false);
            let open = menu_placement(at, viewport, px(32.), true);
            assert_eq!(closed.origin + closed.parent, open.origin + open.parent);
            assert!(open.origin.x >= px(MENU_MARGIN) && open.origin.y >= px(MENU_MARGIN));
            assert!(open.origin.x + open.size.width <= px(width - MENU_MARGIN));
            assert!(open.origin.y + open.size.height <= px(height - MENU_MARGIN));
            if width == 1120. && x > 1000. {
                assert!(open.child.x < open.parent.x);
            }
        }
    }
}
