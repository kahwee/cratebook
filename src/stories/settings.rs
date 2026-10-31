use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{field, fieldset, select, switch, toggle, toggle_group};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Settings",
        description: "Desktop preferences composed from Fieldset, Switch, Select and Toggle Group.",
        path: file!(),
        source: include_str!("settings.rs"),
        preview: cx.new(|_| Settings::default()).into(),
    }
}

#[derive(Default)]
struct Settings {
    menu_bar: bool,
    language: usize,
    choosing: bool,
    side: usize,
}

fn row(
    label: &'static str,
    description: &'static str,
    control: impl IntoElement,
    p: ui::Palette,
) -> impl IntoElement {
    field::root(label, label)
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .px_3()
        .py_2()
        .child(
            div()
                .min_w_0()
                .flex_1()
                .child(
                    field::label("label", label)
                        .font_weight(FontWeight::MEDIUM)
                        .child(label),
                )
                .child(
                    field::description("description", description)
                        .text_size(px(12.))
                        .text_color(rgb(p.muted))
                        .child(description),
                ),
        )
        .child(control)
}

impl Render for Settings {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let height = ui::control_height(window);
        let enabled = self.menu_bar;
        let menu = switch::root("menu", "Show in menu bar", enabled)
            .h(height)
            .min_w(px(44.))
            .flex()
            .items_center()
            .justify_end()
            .focus_visible(move |s| s.bg(rgb(p.selected)))
            .on_click(cx.listener(|s, _, _, cx| {
                s.menu_bar = !s.menu_bar;
                cx.notify();
            }))
            .child(
                div()
                    .w(px(30.))
                    .h(px(18.))
                    .p(px(2.))
                    .rounded_full()
                    .flex()
                    .when(enabled, |s| s.justify_end())
                    .bg(rgb(if enabled { 0xaa4071 } else { p.muted }))
                    .child(
                        switch::thumb()
                            .size(px(14.))
                            .rounded_full()
                            .bg(rgb(0xffffff)),
                    ),
            );
        let languages = ["English", "Deutsch", "Español"];
        let options = select::list("languages", "Language")
            .flex()
            .flex_col()
            .children(languages.into_iter().enumerate().map(|(i, name)| {
                select::item(("language", i), name, self.language == i)
                    .min_h(height)
                    .px_2()
                    .flex()
                    .items_center()
                    .focus_visible(|s| s.underline())
                    .when(self.language == i, |s| s.bg(rgb(p.selected)))
                    .on_click(cx.listener(move |s, _, _, cx| {
                        s.language = i;
                        s.choosing = false;
                        cx.notify();
                    }))
                    .child(select::item_text().child(name))
            }));
        let language = select::root()
            .child(
                select::trigger("choose", "Choose language", self.choosing)
                    .min_h(height)
                    .px_2()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(p.border))
                    .flex()
                    .items_center()
                    .gap_2()
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|s, _, _, cx| {
                        s.choosing = !s.choosing;
                        cx.notify();
                    }))
                    .child(select::value().child(languages[self.language]))
                    .child("⌄"),
            )
            .when(self.choosing, |s| s.child(select::popup().child(options)));
        let location = toggle_group::root("location", "Terminal location")
            .flex()
            .children(
                ["Bottom", "Right"]
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| {
                        toggle::root(("location", i), name, self.side == i)
                            .min_h(height)
                            .px_2()
                            .rounded_md()
                            .flex()
                            .items_center()
                            .when(self.side == i, |s| s.bg(rgb(p.selected)))
                            .focus_visible(|s| s.underline())
                            .on_click(cx.listener(move |s, _, _, cx| {
                                s.side = i;
                                cx.notify();
                            }))
                            .child(name)
                    }),
            );
        let line = || div().mx_3().h(px(1.)).bg(rgb(p.border));
        fieldset::root("general", "General")
            .p_4()
            .text_size(px(13.))
            .child(
                fieldset::legend("general-title", "General")
                    .mb_2()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("General"),
            )
            .child(
                div()
                    .border_1()
                    .border_color(rgb(p.border))
                    .rounded_lg()
                    .child(row("Language", "Language for the app UI", language, p))
                    .child(line())
                    .child(row(
                        "Show in menu bar",
                        "Keep a shortcut in the menu bar.",
                        menu,
                        p,
                    ))
                    .child(line())
                    .child(row(
                        "Terminal location",
                        "Where new terminals open.",
                        location,
                        p,
                    )),
            )
    }
}
