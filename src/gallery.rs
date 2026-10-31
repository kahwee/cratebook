use crate::{
    stories::{self, Story},
    ui::{self, Palette},
};
use gpui::{prelude::*, *};

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
            split_titlebar: false,
            theme_picker: ui::RadioGroup::new(cx),
        }
    }
    fn story_cover(&self, compact: bool, p: Palette, tahoe: bool) -> Div {
        let story = &self.stories[self.selected];
        if !compact {
            return div()
                .pb_3()
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(p.muted))
                        .child("GPUI Jig  /  Components"),
                )
                .child(
                    div()
                        .mt_2()
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
            .rounded_xl()
            .bg(rgb(if tahoe { 0x254668 } else { 0x173c34 }))
            .text_color(rgb(0xf4fbef))
            .p(if compact { px(20.) } else { px(30.) })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(if tahoe { 0xc8e3ff } else { 0xb6d9bf }))
                            .child("GPUI JIG   /   COMPONENT LIBRARY"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(if tahoe { 0xc8e3ff } else { 0xc4e8b0 }))
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
                    .text_color(rgb(if tahoe { 0xdaeaff } else { 0xd2e6d9 }))
                    .child(story.description),
            )
    }

    fn brand() -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .size(px(22.))
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(2.))
                    .bg(rgb(0x173c34))
                    .children([10., 16., 12.].map(|height| {
                        div().w(px(3.)).h(px(height)).rounded_sm().bg(rgb(0xbce6a3))
                    })),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .child("Cratebook"),
            )
    }
    fn library(&self, compact: bool, p: Palette, cx: &Context<Self>) -> Div {
        let rows = self.stories.iter().enumerate().map(|(index, story)| {
            ui::button(story.title, story.title)
                .h(px(if compact { 54. } else { 32. }))
                .rounded_md()
                .gap_3()
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
        let list = div().flex().flex_col().p_2().gap_1().children(rows);
        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .when(!compact, |s| s.w(px(SIDEBAR_WIDTH)).border_r_1())
            .when(compact, |s| s.flex_1().min_h_0().p_4())
            .border_color(rgb(p.border))
            .bg(rgb(p.sidebar))
            .when(!compact, |s| {
                s.child(
                    div()
                        .px_4()
                        .pt_5()
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
                div()
                    .id("library-list")
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(list),
            )
    }
    fn phone_navigation(&self, p: Palette, cx: &Context<Self>) -> Div {
        let active = if *cx.global::<ui::Theme>() == ui::Theme::Tahoe {
            p.text
        } else if p == Palette::LIGHT {
            0x23553d
        } else {
            0xc6eeb9
        };
        div()
            .flex()
            .flex_shrink_0()
            .border_t_1()
            .border_color(rgb(p.border))
            .bg(rgb(p.canvas))
            .p_2()
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
                    ui::button(label, label)
                        .aria_toggled(current.into())
                        .flex_1()
                        .h(px(44.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(14.))
                        .rounded_md()
                        .when(current, |s| {
                            s.bg(rgb(p.selected))
                                .text_color(rgb(active))
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
    fn toolbar(&self, compact: bool, fullscreen: bool, p: Palette, cx: &Context<Self>) -> Div {
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
        .mr_2()
        .rounded_md()
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
                .rounded_sm()
                .border_1()
                .border_color(rgb(p.muted))
                .child(div().ml(px(4.)).w(px(1.)).h_full().bg(rgb(p.muted))),
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
            .when(!compact && !split, |s| {
                s.child(Self::brand())
                    .child(div().w(px(1.)).h(px(18.)).bg(rgb(p.border)))
            })
            .when(split, |s| s.pl_3())
            .child(
                div()
                    .min_w_0()
                    .truncate()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.stories[self.selected].title),
            );
        ui::pane_header(p)
            .h(px(44.))
            .pl(px(0.))
            .pr_3()
            .when(split, |s| s.bg(rgb(p.canvas)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .pl(px(inset))
                    .when(split, |s| {
                        s.w(px(SIDEBAR_WIDTH))
                            .flex_shrink_0()
                            .bg(rgb(p.sidebar))
                            .border_r_1()
                            .border_color(rgb(p.border))
                    })
                    .when(!compact, |s| s.child(sidebar_button))
                    .when(split, |s| s.child(Self::brand())),
            )
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
                    .rounded_md()
                    .when(self.split_titlebar, |s| s.bg(rgb(p.selected)))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.split_titlebar = !this.split_titlebar;
                        cx.notify();
                    }))
                    .child("Split title bar"),
                )
            })
            .child(
                toggle
                    .rounded_md()
                    .hover(move |s| s.bg(rgb(p.selected)))
                    .min_h(px(if compact { 44. } else { 28. }))
                    .flex()
                    .items_center(),
            )
    }
}
impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = ui::compact_layout(window);
        let appearance = *cx.global::<Palette>();
        let selected_theme = *cx.global::<ui::Theme>();
        let tahoe = selected_theme == ui::Theme::Tahoe;
        let p = if tahoe {
            selected_theme.palette(appearance)
        } else {
            appearance
        };
        let story = &self.stories[self.selected];
        let theme = ui::preview_palette(cx);
        let source = story.source;
        let preview = div()
            .bg(rgb(theme.canvas))
            .text_color(rgb(theme.text))
            .min_w_0()
            .rounded_lg()
            .when(tahoe, |s| s.rounded_xl().shadow_sm())
            .overflow_hidden()
            .border_1()
            .border_color(rgb(theme.border))
            .child(story.preview.clone());
        let content = div()
            .id((story.title, usize::from(self.panel == Panel::Source)))
            .flex_1()
            .overflow_y_scroll()
            .w_full()
            .when(!compact, |s| s.max_w(px(800.)).mx_auto())
            .p(if compact { px(14.) } else { px(36.) })
            .flex()
            .flex_col()
            .gap_3()
            .child(self.story_cover(compact, p, tahoe))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .flex_wrap()
                    .when(self.panel != Panel::Source, |s| {
                        s.items_center().child(
                            div()
                                .flex_1()
                                .text_size(px(14.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Live preview"),
                        )
                    })
                    .when(!compact, |s| {
                        s.child(ui::action(
                            if self.panel == Panel::Source {
                                "Show preview"
                            } else {
                                "Rust source"
                            },
                            window,
                            cx,
                            |s: &mut Gallery| {
                                s.panel = if s.panel == Panel::Source {
                                    Panel::Canvas
                                } else {
                                    Panel::Source
                                }
                            },
                        ))
                    })
                    .when(!compact || self.panel == Panel::Canvas, |s| {
                        s.child(
                            self.theme_picker
                                .root("theme-picker", "Preview theme")
                                .flex()
                                .items_center()
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(p.border))
                                .p(px(2.))
                                .children(ui::Theme::ALL.iter().enumerate().map(|(i, &option)| {
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
                                        .rounded_md()
                                        .when(selected_theme == option, |s| {
                                            s.bg(rgb(p.selected)).font_weight(FontWeight::SEMIBOLD)
                                        })
                                        .focus_visible(|s| s.underline())
                                        .child(option.name())
                                })),
                        )
                    }),
            )
            .when(self.panel != Panel::Source, |s| s.child(preview))
            .when(self.panel == Panel::Source, |s| {
                s.child(
                    div()
                        .rounded_lg()
                        .p_4()
                        .bg(rgb(0x192722))
                        .text_color(rgb(0xe0eee4))
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
                                .when(!cfg!(target_family = "wasm"), |s| s.font_family("Menlo"))
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
            .when(!compact && self.sidebar_visible, |s| {
                s.child(self.library(false, p, cx))
            })
            .child(workspace);
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(p.sidebar))
            .text_color(rgb(p.text))
            .when(cfg!(target_family = "wasm"), |s| s.font_family("Noto Sans"))
            .child(self.toolbar(compact, window.is_fullscreen(), p, cx))
            .child(body)
    }
}
