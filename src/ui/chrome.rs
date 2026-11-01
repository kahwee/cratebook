use super::button;
use gpui::{prelude::*, *};

/// Workspace colors and dimensions follow the Commonleaf desktop shell.
#[derive(Clone, Copy, PartialEq)]
pub struct Palette {
    pub canvas: u32,
    pub sidebar: u32,
    pub border: u32,
    pub text: u32,
    pub muted: u32,
    pub selected: u32,
}
impl Global for Palette {}
impl Palette {
    pub const LIGHT: Self = Self {
        canvas: 0xffffff,
        sidebar: 0xf6f5f3,
        border: 0xdedddb,
        text: 0x292826,
        muted: 0x65635f,
        selected: 0xe7e5e2,
    };
    pub const DARK: Self = Self {
        canvas: 0x202024,
        sidebar: 0x27272b,
        border: 0x3d3d43,
        text: 0xe5e5e8,
        muted: 0xaaaab4,
        selected: 0x41414a,
    };
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Editor,
    Base,
    Mint,
    Tahoe,
}
impl Global for Theme {}
impl Theme {
    pub const ALL: &'static [Self] = &[Self::Editor, Self::Base, Self::Mint, Self::Tahoe];
    pub fn name(self) -> &'static str {
        match self {
            Self::Editor => "Editor",
            Self::Base => "Base",
            Self::Mint => "Mint",
            Self::Tahoe => "Tahoe",
        }
    }
    pub fn palette(self, p: Palette) -> Palette {
        match self {
            Self::Base => return p,
            Self::Editor => {
                return if p == Palette::LIGHT {
                    Palette {
                        canvas: 0xfafafa,
                        sidebar: 0xf0f0f1,
                        border: 0xd6d6d8,
                        text: 0x262629,
                        muted: 0x5e5e64,
                        selected: 0xdcdce0,
                    }
                } else {
                    Palette {
                        canvas: 0x202023,
                        sidebar: 0x28282b,
                        border: 0x404045,
                        text: 0xe4e4e6,
                        muted: 0xaeafb5,
                        selected: 0x414148,
                    }
                };
            }
            Self::Tahoe => {
                return if p == Palette::LIGHT {
                    Palette {
                        canvas: 0xf8fbff,
                        sidebar: 0xe5effb,
                        border: 0xbacde3,
                        text: 0x172a41,
                        muted: 0x496078,
                        selected: 0xc3dfff,
                    }
                } else {
                    Palette {
                        canvas: 0x152133,
                        sidebar: 0x1e3047,
                        border: 0x425972,
                        text: 0xf1f7ff,
                        muted: 0xb0c6df,
                        selected: 0x304f73,
                    }
                };
            }
            Self::Mint => {}
        }
        if p == Palette::LIGHT {
            Palette {
                canvas: 0xf4faf7,
                sidebar: 0xe0f1e9,
                selected: 0xa7dfc3,
                border: 0x74ae91,
                text: 0x153e30,
                ..p
            }
        } else {
            Palette {
                canvas: 0x172f28,
                sidebar: 0x233e33,
                selected: 0x35694e,
                border: 0x578568,
                text: 0xe0f5ea,
                ..p
            }
        }
    }
}
pub(crate) fn preview_palette(cx: &App) -> Palette {
    cx.global::<Theme>().palette(*cx.global::<Palette>())
}
pub(crate) fn pane_header(p: Palette) -> Stateful<Div> {
    gpui_jig::toolbar::root("gallery-toolbar", "Gallery controls")
        .flex()
        .items_center()
        .justify_between()
        .h(px(30.))
        .flex_shrink_0()
        .px_2()
        .bg(rgb(p.sidebar))
        .border_b_1()
        .border_color(rgb(p.border))
        .text_size(px(14.))
}
pub(crate) fn action<V: 'static>(
    label: &'static str,
    window: &Window,
    cx: &Context<V>,
    change: impl Fn(&mut V) + 'static,
) -> Stateful<Div> {
    let palette = preview_palette(cx);
    button(label, label)
        .min_h(super::control_height(window))
        .text_size(px(13.))
        .flex()
        .items_center()
        .px(px(10.))
        .border_1()
        .border_color(rgb(palette.border))
        .rounded_md()
        .when(*cx.global::<Theme>() == Theme::Tahoe, |s| {
            s.rounded_full().bg(rgb(palette.sidebar))
        })
        .hover(move |s| s.bg(rgb(palette.selected)))
        .focus_visible(|s| s.underline())
        .on_click(cx.listener(move |this, _, _, cx| {
            change(this);
            cx.notify();
        }))
        .child(label)
}
