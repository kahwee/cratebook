//! Each module constructs a named example and owns its live preview state.
mod accordion;
mod alert_dialog;
mod board;
mod button;
mod calendar;
mod card_menu;
mod checkbox;
mod contacts;
mod dialog;
mod disclosure;
mod drawer;
mod kitchen_sink;
mod meter;
mod planner;
mod popover;
mod preview_card;
mod progress;
mod project_form;
mod radio_group;
mod scroll_area;
mod search;
mod settings;
mod table;
mod tabs;
mod toast;
mod tool_hint;
mod workbench;
mod workspace_nav;
mod zoom;

use gpui::{AnyView, App};

pub(crate) struct Story {
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub source: &'static str,
    pub preview: AnyView,
}

pub(crate) fn catalog(cx: &mut App) -> Vec<Story> {
    let [combobox, autocomplete] = search::stories(cx);
    vec![
        kitchen_sink::story(cx),
        contacts::story(cx),
        planner::story(cx),
        board::story(cx),
        calendar::story(cx),
        card_menu::story(cx),
        settings::story(cx),
        tabs::story(cx),
        table::story(cx),
        button::story(cx),
        radio_group::story(cx),
        disclosure::story(cx),
        accordion::story(cx),
        checkbox::story(cx),
        progress::story(cx),
        scroll_area::story(cx),
        meter::story(cx),
        toast::story(cx),
        workspace_nav::story(cx),
        project_form::story(cx),
        zoom::story(cx),
        dialog::story(cx),
        alert_dialog::story(cx),
        drawer::story(cx),
        popover::story(cx),
        preview_card::story(cx),
        tool_hint::story(cx),
        combobox,
        autocomplete,
    ]
}
