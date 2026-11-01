//! A release desk. The checklist, filter and review share one state.
use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{
    card, checkbox, checkbox_group, dialog, progress, state::Range, toggle_group, toolbar,
};

const CHECKS: [&str; 4] = [
    "Review the changelog",
    "Run native tests",
    "Inspect the phone layout",
    "Verify the package",
];

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Kitchen sink",
        description: "Work through a release checklist, review the candidate, and sign off.",
        path: file!(),
        source: include_str!("kitchen_sink.rs"),
        preview: cx
            .new(|_| Desk {
                checked: [true, true, false, false],
                pending_only: false,
                reviewing: false,
                approved: false,
            })
            .into(),
    }
}

struct Desk {
    checked: [bool; CHECKS.len()],
    pending_only: bool,
    reviewing: bool,
    approved: bool,
}

impl Render for Desk {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let height = ui::control_height(window);
        let done = self.checked.iter().filter(|&&v| v).count();
        let total = CHECKS.len();
        let range = Range::new(0., total as f64, done as f64).unwrap();
        let summary = card::root("candidate", "Release candidate")
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .rounded(px(p.corners.card as f32))
            .bg(rgb(p.sidebar))
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(rgb(p.muted))
                    .child("ENGINEERING   /   RELEASES"),
            )
            .child(
                card::title("candidate-title", "Desktop release")
                    .text_size(px(20.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Desktop release"),
            )
            .child(
                progress::root("readiness", "Checks completed", &range)
                    .flex()
                    .gap_1()
                    .children((0..total).map(|i| {
                        div().flex_1().h(px(4.)).rounded_full().bg(rgb(if i < done {
                            p.text
                        } else {
                            p.border
                        }))
                    })),
            )
            .child(format!("{done} of {total} checks complete"))
            .child(div().text_color(rgb(p.muted)).child(if self.approved {
                "Signed off"
            } else {
                "Awaiting sign-off"
            }));
        let filter = toolbar::root("check-tools", "Checklist tools").child(
            toggle_group::root("filter", "Show checks")
                .flex()
                .gap_1()
                .children([(false, "All checks"), (true, "Pending")].into_iter().map(
                    |(pending, label)| {
                        let active = pending == self.pending_only;
                        toggle_group::item(label, label, active)
                            .min_h(height)
                            .px_2()
                            .rounded(px(p.corners.control as f32))
                            .flex()
                            .items_center()
                            .when(active, |s| s.bg(rgb(p.selected)))
                            .focus_visible(|s| s.underline())
                            .child(label)
                            .on_click(cx.listener(move |s, _, _, cx| {
                                s.pending_only = pending;
                                cx.notify();
                            }))
                    },
                )),
        );
        let checklist = checkbox_group::root("checks", "Release checklist")
            .flex()
            .flex_col()
            .children(
                CHECKS
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| !self.pending_only || !self.checked[*i])
                    .map(|(i, label)| {
                        let checked = self.checked[i];
                        checkbox::root(("check", i), label, checked)
                            .min_h(height)
                            .flex()
                            .items_center()
                            .gap_2()
                            .focus_visible(|s| s.underline())
                            .child(
                                checkbox::indicator()
                                    .size(px(12.))
                                    .rounded(px(gpui_jig::corners::Corners::inset(
                                        p.corners.control,
                                        3,
                                    ) as f32))
                                    .border_1()
                                    .border_color(rgb(p.muted))
                                    .when(checked, |s| {
                                        s.bg(rgb(p.text))
                                            .p(px(3.))
                                            .child(div().size_full().bg(rgb(p.canvas)))
                                    }),
                            )
                            .child(label)
                            .on_click(cx.listener(move |s, _, _, cx| {
                                s.checked[i] = !s.checked[i];
                                s.approved = false;
                                cx.notify();
                            }))
                    }),
            )
            .when(self.pending_only && done == total, |s| {
                s.child("All checks complete.")
            });
        let review = dialog::root()
            .when(!self.reviewing && !self.approved, |s| {
                s.child(
                    dialog::trigger("review", "Review candidate", false)
                        .min_h(height)
                        .flex()
                        .items_center()
                        .px_2()
                        .rounded(px(p.corners.control as f32))
                        .bg(rgb(p.selected))
                        .focus_visible(|s| s.underline())
                        .child("Review candidate")
                        .on_click(cx.listener(|s, _, _, cx| {
                            s.reviewing = true;
                            cx.notify();
                        })),
                )
            })
            .when(self.reviewing, |s| {
                s.child(
                    dialog::popup("review-panel", "Candidate review")
                        .flex()
                        .flex_col()
                        .gap_2()
                        .p_3()
                        .border_1()
                        .border_color(rgb(p.border))
                        .rounded(px(p.corners.dialog as f32))
                        .child("Candidate review")
                        .child(if done == total {
                            "All checks passed. Ready to sign off.".to_owned()
                        } else {
                            format!("{} checks still need attention.", total - done)
                        })
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(ui::action(
                                    "Back to checklist",
                                    window,
                                    cx,
                                    |s: &mut Desk| s.reviewing = false,
                                ))
                                .when(done == total, |row| {
                                    row.child(ui::action("Sign off", window, cx, |s: &mut Desk| {
                                        s.approved = true;
                                        s.reviewing = false;
                                    }))
                                }),
                        ),
                )
            });
        div()
            .flex()
            .p_3()
            .gap_3()
            .when(ui::compact_layout(window), |s| s.flex_col())
            .child(summary.when(!ui::compact_layout(window), |s| {
                s.w(px(180.)).flex_shrink_0()
            }))
            .child(
                div()
                    .min_w_0()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .when(!self.reviewing, |s| s.child(filter).child(checklist))
                    .child(review),
            )
    }
}
