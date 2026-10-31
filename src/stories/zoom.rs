use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{number_field, slider, state::Range};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Zoom controls",
        description: "Change the document zoom with the stepper, or focus the slider and use the arrow keys.",
        path: file!(),
        source: include_str!("zoom.rs"),
        preview: cx
            .new(|_| Preview {
                zoom: Range::new(50., 200., 100.).unwrap(),
            })
            .into(),
    }
}
struct Preview {
    zoom: Range,
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let adjust = |id, label, delta| {
            number_field::increment(id, label)
                .min_h(ui::control_height(w))
                .px_2()
                .flex()
                .items_center()
                .focus_visible(|s| s.underline())
                .child(label)
                .on_click(cx.listener(move |s, _, _, cx| {
                    s.zoom.step(delta);
                    cx.notify();
                }))
        };
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                number_field::root("zoom", "Document zoom")
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(adjust("less", "Decrease", -10.))
                    .child(format!("{}%", self.zoom.value()))
                    .child(adjust("more", "Increase", 10.)),
            )
            .child(
                slider::root("slider", "Zoom").child(
                    slider::thumb("thumb", "Zoom percentage", &self.zoom)
                        .focus_visible(move |s| s.bg(rgb(p.selected)))
                        .min_h(ui::control_height(w))
                        .flex()
                        .items_center()
                        .child(
                            slider::track().w_full().h(px(6.)).bg(rgb(p.border)).child(
                                div()
                                    .h_full()
                                    .w(relative(((self.zoom.value() - 50.) / 150.) as f32))
                                    .bg(rgb(p.text)),
                            ),
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
