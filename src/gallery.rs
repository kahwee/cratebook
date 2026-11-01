use crate::{
    stories::{self, Story},
    ui::{self, Palette},
};
use gpui::{prelude::*, *};
use gpui_jig::{card, navigation_menu, scroll_area, toggle};

#[derive(Clone, Copy, PartialEq)]
enum Panel {
    Stories,
    Canvas,
    Source,
}

const SIDEBAR_WIDTH: f32 = 218.;

pub(crate) struct Gallery {
    stories: Vec<Story>,
    selected: usize,
    panel: Panel,
    sidebar_visible: bool,
    split_titlebar: bool,
    theme_picker: ui::RadioGroup<{ ui::Theme::ALL.len() }>,
}

impl Gallery {
    pub(crate) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            stories: stories::catalog(cx),
            selected: 0,
            panel: Panel::Canvas,
            sidebar_visible: true,
            split_titlebar: true,
            theme_picker: ui::RadioGroup::new(cx),
        }
    }
    fn story_cover(&self, compact: bool, p: Palette, editor: bool) -> Div {
        let story = &self.stories[self.selected];
        if !compact || editor {
            return div()
                .pb(px(4.))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(p.muted))
                        .child("GPUI Jig  /  Components"),
                )
                .child(
                    div()
                        .mt(px(6.))
                        .text_size(px(24.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(story.title),
                )
                .child(
                    div()
                        .mt_1()
                        .text_size(px(14.))
                        .text_color(rgb(p.muted))
                        .child(story.description),
                );
        }
        div()
            .rounded(px(p.corners.card as f32))
            .bg(rgb(p.sidebar))
            .text_color(rgb(p.text))
            .p(if compact { px(20.) } else { px(30.) })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(p.muted))
                            .child("GPUI JIG   /   COMPONENT LIBRARY"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(p.muted))
                            .child(format!(
                                "{:02} / {:02}",
                                self.selected + 1,
                                self.stories.len()
                            )),
                    ),
            )
            .child(
                div()
                    .mt_5()
                    .text_size(px(if compact { 32. } else { 40. }))
                    .font_weight(FontWeight::BOLD)
                    .child(story.title),
            )
            .child(
                div()
                    .mt_1()
                    .max_w(px(520.))
                    .text_size(px(14.))
                    .text_color(rgb(p.text))
                    .child(story.description),
            )
    }

    fn brand(p: Palette) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .size(px(22.))
                    .rounded(px(p.corners.control as f32))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(2.))
                    .bg(rgb(0x173c34))
                    .children([10., 16., 12.].map(|height| {
                        div()
                            .w(px(3.))
                            .h(px(height))
                            .rounded(px(
                                gpui_jig::corners::Corners::inset(p.corners.control, 3) as f32
                            ))
                            .bg(rgb(0xbce6a3))
                    })),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .child("Cratebook"),
            )
    }
    fn library(&self, compact: bool, p: Palette, cx: &Context<Self>) -> Stateful<Div> {
        let editor = *cx.global::<ui::Theme>() == ui::Theme::Editor;
        let rows = self.stories.iter().enumerate().map(|(index, story)| {
            ui::button(story.title, story.title)
                .h(px(if compact {
                    48.
                } else if editor {
                    27.
                } else {
                    32.
                }))
                .rounded(px(p.corners.control as f32))
                .gap(px(8.))
                .flex()
                .items_center()
                .px_2()
                .when(index == self.selected, |s| s.bg(rgb(p.selected)))
                .focus_visible(|s| s.underline())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.selected = index;
                    if compact {
                        this.panel = Panel::Canvas;
                    }
                    cx.notify();
                }))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(p.muted))
                        .child(format!("{:02}", index + 1)),
                )
                .child(story.title)
        });
        let list = div()
            .flex()
            .flex_col()
            .px(px(if compact { 0. } else { 8. }))
            .py(px(6.))
            .gap(px(if editor && !compact { 1. } else { 4. }))
            .children(rows);
        navigation_menu::root("component-navigation", "Components")
            .flex()
            .flex_col()
            .flex_shrink_0()
            .when(!compact, |s| s.w(px(SIDEBAR_WIDTH)).border_l_1())
            .when(compact, |s| s.flex_1().min_h_0().px(px(16.)).py(px(12.)))
            .border_color(rgb(p.border))
            .bg(rgb(p.sidebar))
            .when(!compact && !self.split_titlebar, |s| {
                s.child(
                    div()
                        .px_4()
                        .pt(px(12.))
                        .text_color(rgb(p.muted))
                        .text_size(px(12.))
                        .child("Components"),
                )
            })
            .when(compact, |s| {
                s.child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(p.muted))
                        .child("GPUI JIG"),
                )
                .child(
                    div()
                        .text_size(px(26.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("Stories"),
                )
                .child(
                    div()
                        .py_2()
                        .text_size(px(14.))
                        .text_color(rgb(p.muted))
                        .child(format!("{} component examples", self.stories.len())),
                )
            })
            .child(
                scroll_area::viewport("library-list")
                    .flex_1()
                    .overflow_x_hidden()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(list),
            )
    }
    fn phone_navigation(&self, p: Palette, cx: &Context<Self>) -> Stateful<Div> {
        navigation_menu::root("phone-navigation", "Gallery views")
            .flex()
            .flex_shrink_0()
            .border_t_1()
            .border_color(rgb(p.border))
            .bg(rgb(p.canvas))
            .px(px(12.))
            .py(px(4.))
            .gap_1()
            .children(
                [
                    (Panel::Stories, "Stories"),
                    (Panel::Canvas, "Canvas"),
                    (Panel::Source, "Rust source"),
                ]
                .into_iter()
                .map(|(panel, label)| {
                    let current = panel == self.panel;
                    toggle::root(label, label, current)
                        .flex_1()
                        .h(px(44.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(14.))
                        .rounded(px(p.corners.control as f32))
                        .when(current, |s| {
                            s.bg(rgb(p.selected))
                                .text_color(rgb(p.text))
                                .font_weight(FontWeight::SEMIBOLD)
                        })
                        .focus_visible(|s| s.underline())
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.panel = panel;
                            cx.notify();
                        }))
                        .child(label)
                }),
            )
    }
    fn toolbar(
        &self,
        compact: bool,
        fullscreen: bool,
        p: Palette,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        let light = *cx.global::<Palette>() == Palette::LIGHT;
        let label = if light { "Dark mode" } else { "Light mode" };
        let toggle = ui::button("appearance", label)
            .px_2()
            .focus_visible(|s| s.underline())
            .on_click(cx.listener(move |_, _, _, cx| {
                cx.set_global(if light { Palette::DARK } else { Palette::LIGHT });
                cx.refresh_windows();
            }))
            .child(label);
        let split = self.split_titlebar && !compact && self.sidebar_visible;
        let inset = if cfg!(target_os = "macos") && !fullscreen {
            84.
        } else {
            12.
        };
        let sidebar_button = ui::button(
            "toggle-sidebar",
            if self.sidebar_visible {
                "Hide sidebar"
            } else {
                "Show sidebar"
            },
        )
        .aria_expanded(self.sidebar_visible)
        .w(px(32.))
        .h(px(32.))
        .mr(px(4.))
        .rounded(px(p.corners.control as f32))
        .flex()
        .items_center()
        .justify_center()
        .hover(move |s| s.bg(rgb(p.selected)))
        .focus_visible(|s| s.bg(rgb(p.selected)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.sidebar_visible = !this.sidebar_visible;
            cx.notify();
        }))
        .child(
            div()
                .w(px(17.))
                .h(px(13.))
                .rounded(px(
                    gpui_jig::corners::Corners::inset(p.corners.control, 3) as f32
                ))
                .border_1()
                .border_color(rgb(p.muted))
                .child(div().ml(px(10.)).w(px(1.)).h_full().bg(rgb(p.muted))),
        );
        let title = div()
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .items_center()
            .gap_3()
            .when(cfg!(target_os = "macos"), |s| {
                s.on_mouse_down(MouseButton::Left, |event, window, _| {
                    if event.click_count == 2 {
                        window.titlebar_double_click();
                    } else {
                        window.start_window_move();
                    }
                })
            })
            .when(!compact, |s| {
                s.child(Self::brand(p))
                    .child(div().w(px(1.)).h(px(18.)).bg(rgb(p.border)))
            })
            .child(
                div()
                    .min_w_0()
                    .truncate()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.stories[self.selected].title),
            );
        ui::pane_header(p)
            .h(px(44.))
            .pl(px(inset))
            .pr(px(if compact { 10. } else { 0. }))
            .when(split, |s| s.bg(rgb(p.canvas)))
            .child(title)
            .when(!compact, |s| {
                s.child(
                    gpui_jig::toggle::root(
                        "split-titlebar",
                        "Split title bar",
                        self.split_titlebar,
                    )
                    .px_2()
                    .py_1()
                    .rounded(px(p.corners.control as f32))
                    .when(self.split_titlebar, |s| s.bg(rgb(p.selected)))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.split_titlebar = !this.split_titlebar;
                        cx.notify();
                    }))
                    .mr(px(4.))
                    .child("Split title bar"),
                )
            })
            .child(
                toggle
                    .rounded(px(p.corners.control as f32))
                    .hover(move |s| s.bg(rgb(p.selected)))
                    .min_h(px(if compact { 44. } else { 28. }))
                    .flex()
                    .items_center(),
            )
            .when(!compact, |s| {
                s.child(
                    div()
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_end()
                        .pr_2()
                        .when(split, |s| {
                            s.w(px(SIDEBAR_WIDTH))
                                .flex_shrink_0()
                                .pl_4()
                                .justify_between()
                                .border_l_1()
                                .border_color(rgb(p.border))
                                .bg(rgb(p.sidebar))
                                .child("Components")
                        })
                        .child(sidebar_button),
                )
            })
    }
}
impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = ui::compact_layout(window);
        let selected_theme = *cx.global::<ui::Theme>();
        let tahoe = selected_theme == ui::Theme::Tahoe;
        let editor = selected_theme == ui::Theme::Editor;
        let p = ui::preview_palette(cx);
        let story = &self.stories[self.selected];
        let source = story.source;
        let preview = card::root("story-preview", "Live preview")
            .bg(rgb(p.canvas))
            .text_color(rgb(p.text))
            .min_w_0()
            .rounded(px(p.corners.card as f32))
            .when(tahoe, |s| s.shadow_sm())
            .overflow_hidden()
            .border_1()
            .border_color(rgb(p.border))
            .child(story.preview.clone());
        let content =
            scroll_area::viewport((story.title, usize::from(self.panel == Panel::Source)))
                .overflow_x_hidden()
                .flex_1()
                .overflow_y_scroll()
                .w_full()
                .when(!compact, |s| s.max_w(px(800.)).mx_auto())
                .px(px(if compact { 16. } else { 28. }))
                .pt(px(if compact { 18. } else { 24. }))
                .pb(px(if compact { 16. } else { 28. }))
                .flex()
                .flex_col()
                .gap(px(10.))
                .child(self.story_cover(compact, p, editor))
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .items_center()
                        .when(compact && self.panel == Panel::Canvas, |s| {
                            s.child(
                                div()
                                    .flex_1()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Live preview"),
                            )
                        })
                        .when(!compact, |s| {
                            let views =
                                [(Panel::Canvas, "Preview"), (Panel::Source, "Rust source")];
                            s.child(
                                gpui_jig::toggle_group::root("story-view", "Story view")
                                    .flex()
                                    .rounded(px(p.corners.card as f32))
                                    .bg(rgb(p.sidebar))
                                    .p(px(2.))
                                    .children(views.into_iter().map(|(panel, label)| {
                                        let current = if panel == Panel::Canvas {
                                            self.panel != Panel::Source
                                        } else {
                                            self.panel == panel
                                        };
                                        gpui_jig::toggle_group::item(
                                            ("story-view", panel as usize),
                                            label,
                                            current,
                                        )
                                        .min_h(ui::control_height(window))
                                        .px_2()
                                        .rounded(px(p.corners.control as f32))
                                        .flex()
                                        .items_center()
                                        .when(current, |s| {
                                            s.bg(rgb(p.selected)).font_weight(FontWeight::SEMIBOLD)
                                        })
                                        .focus_visible(|s| s.underline())
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.panel = panel;
                                            cx.notify();
                                        }))
                                        .child(label)
                                    })),
                            )
                        })
                        .when(!compact || self.panel == Panel::Canvas, |s| {
                            s.child(
                                self.theme_picker
                                    .root("theme-picker", "Gallery theme")
                                    .flex()
                                    .items_center()
                                    .rounded(px(p.corners.card as f32))
                                    .border_1()
                                    .border_color(rgb(p.border))
                                    .p(px(2.))
                                    .children(ui::Theme::ALL.iter().enumerate().map(
                                        |(i, &option)| {
                                            self.theme_picker
                                                .item(
                                                    ("theme", i),
                                                    option.name(),
                                                    i,
                                                    selected_theme == option,
                                                    |index, _, cx| {
                                                        cx.set_global(ui::Theme::ALL[*index]);
                                                        cx.refresh_windows();
                                                    },
                                                )
                                                .min_h(ui::control_height(window))
                                                .flex()
                                                .items_center()
                                                .px_2()
                                                .rounded(px(gpui_jig::corners::Corners::inset(
                                                    p.corners.card,
                                                    2,
                                                )
                                                    as f32))
                                                .when(selected_theme == option, |s| {
                                                    s.bg(rgb(p.selected))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                })
                                                .focus_visible(|s| s.underline())
                                                .child(option.name())
                                        },
                                    )),
                            )
                            .child({
                                let style = *cx.global::<ui::CornerStyle>();
                                ui::button("corner-style", style.label())
                                    .min_h(ui::control_height(window))
                                    .px_2()
                                    .rounded(px(p.corners.control as f32))
                                    .border_1()
                                    .border_color(rgb(p.border))
                                    .flex()
                                    .items_center()
                                    .focus_visible(|s| s.underline())
                                    .on_click(move |_, _, cx| {
                                        cx.set_global(style.next());
                                        cx.refresh_windows();
                                    })
                                    .child(style.label())
                            })
                        }),
                )
                .when(self.panel != Panel::Source, |s| s.child(preview))
                .when(self.panel == Panel::Source, |s| {
                    s.child(
                        div()
                            .rounded(px(p.corners.card as f32))
                            .p_4()
                            .bg(rgb(p.sidebar))
                            .text_color(rgb(p.text))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .items_center()
                                    .justify_between()
                                    .child(story.path)
                                    .child(
                                        ui::button("copy-source", "Copy Rust source")
                                            .min_h(ui::control_height(window))
                                            .flex()
                                            .items_center()
                                            .px_2()
                                            .focus_visible(|s| s.underline())
                                            .on_click(move |_, _, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    source.to_owned(),
                                                ))
                                            })
                                            .child("Copy code"),
                                    ),
                            )
                            .child(
                                div()
                                    .when(cfg!(target_os = "windows"), |s| {
                                        s.font_family("Consolas")
                                    })
                                    .when(cfg!(target_os = "macos"), |s| s.font_family("Menlo"))
                                    .text_size(px(13.))
                                    .child(source),
                            ),
                    )
                });
        let workspace = div()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .when(compact && self.panel == Panel::Stories, |s| {
                s.child(self.library(true, p, cx))
            })
            .when(!compact || self.panel != Panel::Stories, |s| {
                s.child(content)
            })
            .when(editor && !compact, |s| {
                s.child(
                    div()
                        .h(px(24.))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .px_3()
                        .border_t_1()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.sidebar))
                        .text_color(rgb(p.muted))
                        .text_size(px(11.))
                        .child(story.path),
                )
            })
            .when(compact, |s| s.child(self.phone_navigation(p, cx)));
        let body = div()
            .flex_1()
            .min_h_0()
            .min_w_0()
            .flex()
            .when(compact, |s| s.flex_col())
            .when(cfg!(target_family = "wasm"), |s| s.font_family("Noto Sans"))
            .bg(rgb(p.canvas))
            .text_color(rgb(p.text))
            .text_size(px(if compact { 16. } else { 13. }))
            .child(workspace)
            .when(!compact && self.sidebar_visible, |s| {
                s.child(self.library(false, p, cx))
            });
        div()
            .size_full()
            .when(cfg!(target_os = "windows"), |s| s.font_family("Segoe UI"))
            .flex()
            .flex_col()
            .bg(rgb(p.sidebar))
            .text_color(rgb(p.text))
            .when(cfg!(target_family = "wasm"), |s| s.font_family("Noto Sans"))
            .child(self.toolbar(compact, window.is_fullscreen(), p, cx))
            .child(body)
    }
}
