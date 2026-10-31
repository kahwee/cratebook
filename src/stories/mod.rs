//! Each module constructs a named example and owns its live preview state.
mod button;
mod card_menu;
mod disclosure;
mod radio_group;
mod settings;
mod table;
mod tabs;

use gpui::{AnyView, App};

pub(crate) struct Story {
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub source: &'static str,
    pub preview: AnyView,
}

pub(crate) fn catalog(cx: &mut App) -> Vec<Story> {
    vec![
        card_menu::story(cx),
        settings::story(cx),
        tabs::story(cx),
        table::story(cx),
        button::story(cx),
        radio_group::story(cx),
        disclosure::story(cx),
    ]
}
