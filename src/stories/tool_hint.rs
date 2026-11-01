use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::tooltip;

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Tooltip",
        description: "Hover over the sync button to inspect its help text, or activate it to sync.",
        path: file!(),
        source: include_str!("tool_hint.rs"),
        preview: cx
            .new(|_| Preview {
                hovered: false,
                synced: false,
            })
            .into(),
    }
}
struct Preview {
    hovered: bool,
    synced: bool,
}
impl Render for Preview {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                tooltip::trigger("sync", "Sync workspace")
                    .min_h(ui::control_height(w))
                    .flex()
                    .items_center()
                    .px_2()
                    .rounded(px(p.corners.control as f32))
                    .bg(rgb(p.sidebar))
                    .focus_visible(|s| s.underline())
                    .on_hover(cx.listener(|s, hovered: &bool, _, cx| {
                        s.hovered = *hovered;
                        cx.notify();
                    }))
                    .on_click(cx.listener(|s, _, _, cx| {
                        s.synced = true;
                        cx.notify();
                    }))
                    .child(if self.synced {
                        "Workspace synced"
                    } else {
                        "Sync workspace"
                    }),
            )
            .when(self.hovered || crate::capture_mode(), |s| {
                s.child(
                    tooltip::popup("hint", "Sync hint")
                        .p_2()
                        .rounded(px(p.corners.control as f32))
                        .bg(rgb(p.selected))
                        .child("Refresh local project metadata."),
                )
            })
    }
}
