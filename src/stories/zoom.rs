use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{number_field, slider, state::Range};
use std::{cell::Cell, rc::Rc};

fn zoom_at_pointer(x: Pixels, bounds: Bounds<Pixels>) -> Option<f64> {
    let width = f32::from(bounds.size.width);
    if width <= 0. || !width.is_finite() || !f32::from(x).is_finite() {
        return None;
    }
    let fraction = (f32::from(x - bounds.left()) / width).clamp(0., 1.) as f64;
    Some((50. + (fraction * 150. / 10.).round() * 10.).clamp(50., 200.))
}

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Zoom controls",
        description: "Click or drag the slider, use the stepper, or focus the slider and use arrow and page keys.",
        path: file!(),
        source: include_str!("zoom.rs"),
        preview: cx
            .new(|cx| Preview {
                zoom: Range::new(50., 200., 100.).unwrap(),
                dragging: false,
                focus: cx.focus_handle(),
                track_bounds: Rc::new(Cell::new(Bounds::default())),
            })
            .into(),
    }
}
struct Preview {
    zoom: Range,
    dragging: bool,
    focus: FocusHandle,
    track_bounds: Rc<Cell<Bounds<Pixels>>>,
}
impl Preview {
    fn seek(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(value) = zoom_at_pointer(position.x, self.track_bounds.get()) {
            if self.zoom.value() != value {
                self.zoom.set(value);
                cx.notify();
            }
        }
    }
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        // Register during paint, before a drag starts. A fast drag can deliver
        // its release before another render, and can leave the track's hitbox.
        let entity = cx.entity().downgrade();
        let drag_handlers = canvas(
            |_, _, _| (),
            move |_, _, w, _| {
                let moving = entity.clone();
                w.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                    if phase != DispatchPhase::Capture {
                        return;
                    }
                    moving
                        .update(cx, |s, cx| {
                            if !s.dragging {
                                return;
                            }
                            if event.dragging() {
                                s.seek(event.position, cx);
                            } else {
                                s.dragging = false;
                                cx.notify();
                            }
                        })
                        .ok();
                });
                let releasing = entity.clone();
                w.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                    if phase != DispatchPhase::Capture || event.button != MouseButton::Left {
                        return;
                    }
                    releasing
                        .update(cx, |s, cx| {
                            if !s.dragging {
                                return;
                            }
                            s.seek(event.position, cx);
                            s.dragging = false;
                            cx.notify();
                        })
                        .ok();
                });
            },
        )
        .absolute()
        .size_full();
        let track_bounds = self.track_bounds.clone();
        let fraction = ((self.zoom.value() - 50.) / 150.) as f32;
        let adjust = |id, label, delta| {
            number_field::increment(id, label)
                .min_h(ui::control_height(w))
                .px_2()
                .flex()
                .items_center()
                .rounded(px(p.corners.control as f32))
                .border_1()
                .border_color(rgb(p.border))
                .bg(rgb(p.sidebar))
                .hover(move |s| s.bg(rgb(p.selected)))
                .focus_visible(|s| s.underline())
                .child(label)
                .on_click(cx.listener(move |s, _, _, cx| {
                    s.zoom.step(delta);
                    cx.notify();
                }))
        };
        div()
            .relative()
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(drag_handlers)
            .child(
                number_field::root("zoom", "Document zoom")
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(adjust("less", "Decrease", -10.))
                    .child(
                        div()
                            .min_w(px(54.))
                            .text_center()
                            .child(format!("{}%", self.zoom.value())),
                    )
                    .child(adjust("more", "Increase", 10.)),
            )
            .child(
                slider::root("slider", "Zoom").child(
                    slider::thumb("thumb", "Zoom percentage", &self.zoom)
                        .track_focus(&self.focus)
                        .rounded(px(p.corners.control as f32))
                        .focus_visible(move |s| s.bg(rgb(p.selected)))
                        .min_h(ui::control_height(w).max(px(36.)))
                        .px_2()
                        .flex()
                        .items_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|s, event: &MouseDownEvent, w, cx| {
                                s.dragging = true;
                                w.focus(&s.focus, cx);
                                s.seek(event.position, cx);
                                w.prevent_default();
                                cx.stop_propagation();
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .w_full()
                                .child(
                                    slider::track()
                                        .relative()
                                        .w_full()
                                        .h(px(6.))
                                        .rounded_full()
                                        .bg(rgb(p.border))
                                        .child(
                                            div()
                                                .h_full()
                                                .rounded_full()
                                                .w(relative(fraction))
                                                .bg(rgb(p.text)),
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .left(relative(fraction))
                                                .top(px(-5.))
                                                .ml(px(-8.))
                                                .size(px(16.))
                                                .rounded_full()
                                                .border_2()
                                                .border_color(rgb(p.text))
                                                .bg(rgb(p.canvas))
                                                .shadow_sm(),
                                        ),
                                )
                                .on_children_prepainted(move |bounds, _, _| {
                                    if let Some(bounds) = bounds.first() {
                                        track_bounds.set(*bounds);
                                    }
                                }),
                        )
                        .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                            if e.keystroke.modifiers.modified()
                                || !slider::apply_key(&mut s.zoom, &e.keystroke.key, 10.)
                            {
                                return;
                            }
                            cx.stop_propagation();
                            w.prevent_default();
                            cx.notify();
                        })),
                ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::zoom_at_pointer;
    use gpui::{Bounds, point, px, size};

    #[test]
    fn pointer_zoom_snaps_and_clamps_to_accessible_endpoints() {
        let bounds = Bounds {
            origin: point(px(100.), px(0.)),
            size: size(px(300.), px(6.)),
        };
        assert_eq!(zoom_at_pointer(px(-50.), bounds), Some(50.));
        assert_eq!(zoom_at_pointer(px(220.), bounds), Some(110.));
        assert_eq!(zoom_at_pointer(px(900.), bounds), Some(200.));
        assert_eq!(zoom_at_pointer(px(100.), Bounds::default()), None);
    }
}
