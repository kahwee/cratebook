//! Gallery styling and compatibility exports for the shared primitives.
use gpui::{Div, ElementId, SharedString, Stateful, prelude::*};
pub use gpui_jig::{
    button::root as button, collapsible::trigger as collapsible_trigger, radio_group::RadioGroup,
};

/// Document tab surface shared by the workspace and the component gallery.
/// The owner supplies the title, close action and navigation behavior.
pub fn document_tab(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    active: bool,
    canvas: u32,
    sidebar: u32,
    border: u32,
    focus: u32,
) -> Stateful<Div> {
    button(id, label)
        .focus_visible(move |style| style.bg(gpui::rgb(focus)))
        .flex()
        .items_center()
        .gap_2()
        .h_full()
        .flex_shrink_0()
        .whitespace_nowrap()
        .px_3()
        .border_r_1()
        .border_color(gpui::rgb(border))
        .bg(gpui::rgb(if active { canvas } else { sidebar }))
}

mod chrome;
pub use chrome::Palette;
pub(crate) use chrome::{Accent, action, pane_header, preview_palette};

pub(crate) fn compact_layout(window: &gpui::Window) -> bool {
    window.viewport_size().width < gpui::px(680.)
}

pub(crate) fn control_height(window: &gpui::Window) -> gpui::Pixels {
    gpui::px(if compact_layout(window) { 44. } else { 28. })
}
