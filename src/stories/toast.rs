use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::toast;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Toast",
        description: "Save a draft and dismiss its confirmation.",
        path: file!(),
        source: include_str!("toast.rs"),
        preview: cx
            .new(|_| Draft {
                revision: usize::from(crate::capture_mode()),
                visible: crate::capture_mode(),
            })
            .into(),
    }
}
struct Draft {
    revision: usize,
    visible: bool,
}
impl Render for Draft {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(ui::action("Save draft", window, cx, |s: &mut Draft| {
                s.revision += 1;
                s.visible = true;
            }))
            .child(
                toast::viewport("notifications", "Notifications").when(self.visible, |s| {
                    s.child(
                        toast::root("saved", "Draft saved")
                            .rounded(px(p.corners.toast as f32))
                            .border_1()
                            .border_color(rgb(p.border))
                            .p_3()
                            .child(
                                toast::title()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Draft saved"),
                            )
                            .child(
                                toast::description()
                                    .child(format!("Saved revision {}.", self.revision)),
                            )
                            .child(
                                toast::close("close", "Dismiss notification")
                                    .min_h(ui::control_height(window))
                                    .flex()
                                    .items_center()
                                    .focus_visible(|s| s.underline())
                                    .child("Dismiss")
                                    .on_click(cx.listener(|s, _, _, cx| {
                                        s.visible = false;
                                        cx.notify();
                                    })),
                            ),
                    )
                }),
            )
    }
}
