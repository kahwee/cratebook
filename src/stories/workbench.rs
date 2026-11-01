//! Shared assembly for the two desktop app examples.
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{input, navigation_menu};

pub(super) fn panes(
    window: &Window,
    p: ui::Palette,
    nav: impl IntoElement,
    detail: impl IntoElement,
) -> Div {
    let compact = ui::compact_layout(window);
    div()
        .flex()
        .when(compact, |s| s.flex_col())
        .child(
            navigation_menu::root("navigation", "Browse")
                .flex()
                .flex_col()
                .gap_1()
                .p_3()
                .bg(rgb(p.sidebar))
                .when(!compact, |s| s.w(px(166.)).flex_shrink_0().border_r_1())
                .when(compact, |s| s.border_b_1())
                .border_color(rgb(p.border))
                .child(nav),
        )
        .child(div().flex_1().min_w_0().p_4().child(detail))
}

// Basic single-line entry for the gallery's in-memory fixtures.
pub(super) fn entry<V: 'static>(
    id: &'static str,
    label: &'static str,
    value: &str,
    window: &Window,
    cx: &Context<V>,
    edit: impl Fn(&mut V) -> &mut String + 'static,
) -> Stateful<Div> {
    let p = ui::preview_palette(cx);
    input::control(div().id(id).focusable().tab_stop(true), label)
        .min_h(ui::control_height(window))
        .flex()
        .items_center()
        .px_2()
        .border_1()
        .border_color(rgb(p.border))
        .rounded(px(p.corners.control as f32))
        .focus_visible(move |s| s.border_color(rgb(p.text)))
        .child(if value.is_empty() {
            label.to_owned()
        } else {
            value.to_owned()
        })
        .on_key_down(cx.listener(move |s, e: &KeyDownEvent, w, cx| {
            if let Some(text) = pasted_text(e, cx) {
                edit(s).push_str(&text);
                w.prevent_default();
                cx.stop_propagation();
                cx.notify();
                return;
            }
            if e.keystroke.modifiers.platform || e.keystroke.modifiers.control {
                return;
            }
            if e.keystroke.key == "backspace" {
                edit(s).pop();
            } else if let Some(text) = e
                .keystroke
                .key_char
                .as_ref()
                .filter(|text| !text.chars().any(char::is_control))
            {
                edit(s).push_str(text);
            } else {
                return;
            }
            w.prevent_default();
            cx.stop_propagation();
            cx.notify();
        }))
}

/// Single-line clipboard text for the gallery's append-only fixture editors.
pub(super) fn pasted_text(event: &KeyDownEvent, cx: &App) -> Option<String> {
    if !(event.keystroke.modifiers.control || event.keystroke.modifiers.platform)
        || event.keystroke.key != "v"
    {
        return None;
    }
    cx.read_from_clipboard()
        .and_then(|item| item.text())
        .map(|text| text.chars().filter(|ch| !ch.is_control()).collect())
}
