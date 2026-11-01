use super::{Story, workbench};
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{avatar, button, card, toggle};

struct Person {
    name: &'static str,
    initials: &'static str,
    team: &'static str,
    email: &'static str,
}
const PEOPLE: [Person; 4] = [
    Person {
        name: "Ada Chen",
        initials: "AC",
        team: "Design",
        email: "ada@example.org",
    },
    Person {
        name: "Eli Brooks",
        initials: "EB",
        team: "Research",
        email: "eli@example.org",
    },
    Person {
        name: "Mira Shah",
        initials: "MS",
        team: "Engineering",
        email: "mira@example.org",
    },
    Person {
        name: "Noah Reyes",
        initials: "NR",
        team: "Design",
        email: "noah@example.org",
    },
];
struct Contacts {
    query: String,
    favorites: [bool; 4],
    only_favorites: bool,
    selected: usize,
}
pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Contacts",
        description: "Find a person by name or team, browse favorites, and copy their email.",
        path: file!(),
        source: include_str!("contacts.rs"),
        preview: cx
            .new(|_| Contacts {
                query: String::new(),
                favorites: [true, false, true, false],
                only_favorites: false,
                selected: 0,
            })
            .into(),
    }
}
impl Render for Contacts {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let query = self.query.trim().to_lowercase();
        let visible: Vec<_> = PEOPLE
            .iter()
            .enumerate()
            .filter(|(i, person)| {
                (!self.only_favorites || self.favorites[*i])
                    && (person.name.to_lowercase().contains(&query)
                        || person.team.to_lowercase().contains(&query))
            })
            .map(|(i, _)| i)
            .collect();
        let selected = visible
            .iter()
            .copied()
            .find(|&i| i == self.selected)
            .or_else(|| visible.first().copied());
        let nav = div()
            .flex()
            .flex_col()
            .gap_2()
            .child("Address book")
            .child(workbench::entry(
                "search",
                "Search contacts",
                &self.query,
                w,
                cx,
                |s| &mut s.query,
            ))
            .child(
                toggle::root("fav-filter", "Favorites only", self.only_favorites)
                    .min_h(ui::control_height(w))
                    .px_2()
                    .flex()
                    .items_center()
                    .focus_visible(|s| s.underline())
                    .when(self.only_favorites, |s| s.bg(rgb(p.selected)))
                    .child("Favorites only")
                    .on_click(cx.listener(|s, _, _, cx| {
                        s.only_favorites = !s.only_favorites;
                        cx.notify();
                    })),
            )
            .children(visible.into_iter().map(|i| {
                button::root(("person", i), PEOPLE[i].name)
                    .min_h(ui::control_height(w))
                    .px_2()
                    .flex()
                    .items_center()
                    .rounded(px(p.corners.control as f32))
                    .when(selected == Some(i), |s| s.bg(rgb(p.selected)))
                    .focus_visible(|s| s.underline())
                    .child(PEOPLE[i].name)
                    .on_click(cx.listener(move |s, _, _, cx| {
                        s.selected = i;
                        cx.notify();
                    }))
            }));
        let detail = div()
            .when(selected.is_none(), |s| s.child("No matching contacts"))
            .when_some(selected, |s, i| {
                let person = &PEOPLE[i];
                s.child(
                    card::root("person-detail", person.name)
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            avatar::root("portrait", person.name)
                                .size(px(52.))
                                .rounded_full()
                                .bg(rgb(p.selected))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(avatar::fallback().child(person.initials)),
                        )
                        .child(
                            card::title("name", person.name)
                                .text_size(px(22.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(person.name),
                        )
                        .child(div().text_color(rgb(p.muted)).child(person.team))
                        .child(
                            card::content()
                                .p_3()
                                .rounded(px(p.corners.card as f32))
                                .bg(rgb(p.sidebar))
                                .child("Work email")
                                .child(div().child(person.email)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(ui::action(
                                    if self.favorites[i] {
                                        "Remove favorite"
                                    } else {
                                        "Add favorite"
                                    },
                                    w,
                                    cx,
                                    move |s: &mut Contacts| s.favorites[i] = !s.favorites[i],
                                ))
                                .child(
                                    button::root("copy", "Copy email")
                                        .min_h(ui::control_height(w))
                                        .px_2()
                                        .flex()
                                        .items_center()
                                        .focus_visible(|s| s.underline())
                                        .child("Copy email")
                                        .on_click(move |_, _, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                person.email.to_owned(),
                                            ))
                                        }),
                                ),
                        ),
                )
            });
        workbench::panes(w, p, nav, detail)
    }
}
