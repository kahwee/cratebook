use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{card, context_menu as menu};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Card & context menu",
        description: "Right-click the project card or choose Actions. Move to opens a submenu.",
        path: file!(),
        source: include_str!("card_menu.rs"),
        preview: cx
            .new(|cx| Project {
                position: None,
                nested: false,
                pinned: false,
                folder: "Inbox",
                focus: cx.focus_handle(),
                opener: cx.focus_handle(),
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
    fn popup(&self, nested: bool, p: ui::Palette, cx: &Context<Self>) -> impl IntoElement + use<> {
        let entries = if nested {
            [(MenuIcon::Inbox, "Inbox"), (MenuIcon::Archive, "Archive")]
        } else {
            [
                (MenuIcon::Pin, if self.pinned { "Unpin" } else { "Pin" }),
                (MenuIcon::Chevron, "Move to"),
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
        .w(px(132.))
        .p_1()
        .rounded_md()
        .border_1()
        .border_color(rgb(p.border))
        .bg(rgb(p.canvas))
        .shadow_md()
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
                .h(px(32.))
                .rounded_sm()
                .flex()
                .items_center()
                .gap_2()
                .when(self.nested == nested && self.active == i, |s| {
                    s.bg(rgb(p.selected))
                })
                .hover(move |s| s.bg(rgb(p.selected)))
                .on_click(cx.listener(move |s, _, w, cx| {
                    s.nested = nested;
                    s.choose(i, w, cx);
                }))
                .child(icon.draw(p.muted))
                .child(label)
                .when(
                    (nested && self.folder == label) || (!nested && i == 0 && self.pinned),
                    |s| s.child(div().size(px(5.)).rounded_full().bg(rgb(p.text))),
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
            .rounded_md()
            .bg(rgb(p.sidebar))
            .flex()
            .items_center()
            .focus_visible(|s| s.underline())
            .on_click(cx.listener(|s, event: &ClickEvent, w, cx| {
                let at = event.position();
                s.open(at, w, cx)
            }))
            .child("Actions");
        let target = menu::trigger(
            "project-target",
            "Project card",
            cx.listener(|s, e: &MouseDownEvent, w, cx| s.open(e.position, w, cx)),
        )
        .child(
            card::root("project", "Field notes")
                .border_1()
                .border_color(rgb(p.border))
                .rounded_lg()
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
                let popup = div()
                    .id("menu-layer")
                    .track_focus(&self.focus)
                    .flex()
                    .items_start()
                    .occlude()
                    .on_mouse_down_out(cx.listener(|s, _, _, cx| {
                        s.position = None;
                        cx.notify();
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
                            "right" if !s.nested && s.active == 1 => s.choose(1, w, cx),
                            "enter" | "space" => s.choose(s.active, w, cx),
                            _ => return,
                        }
                        cx.stop_propagation();
                    }))
                    .child(self.popup(false, p, cx))
                    .when(self.nested, |s| {
                        s.child(menu::submenu_root().child(self.popup(true, p, cx)))
                    });
                s.child(menu::positioner(at, popup))
            })
    }
}
