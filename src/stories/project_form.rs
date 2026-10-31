use super::Story;
use crate::ui;
use gpui::{prelude::*, *};
use gpui_jig::{form, input, otp_field};

pub(super) fn story(cx: &mut App) -> Story {
    Story {
        title: "Project form",
        description: "Type a name and four demo digits, then create a local project. Backspace removes the last character.",
        path: file!(),
        source: include_str!("project_form.rs"),
        preview: cx.new(|_| Project::default()).into(),
    }
}
#[derive(Default)]
struct Project {
    name: String,
    code: String,
    message: String,
}
impl Render for Project {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = ui::preview_palette(cx);
        let editor = |id| {
            div()
                .id(id)
                .focusable()
                .tab_stop(true)
                .min_h(ui::control_height(w))
                .px_2()
                .flex()
                .items_center()
                .rounded_md()
                .border_1()
                .border_color(rgb(p.border))
                .focus_visible(move |s| s.border_color(rgb(p.text)))
        };
        let name = input::control(editor("name"), "Project name")
            .child(if self.name.is_empty() {
                "Project name".to_owned()
            } else {
                self.name.clone()
            })
            .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                if e.keystroke.modifiers.control || e.keystroke.modifiers.platform {
                    return;
                }
                if e.keystroke.key == "backspace" {
                    s.name.pop();
                } else if let Some(ch) = &e.keystroke.key_char {
                    if ch.chars().any(char::is_control) {
                        return;
                    }
                    s.name.push_str(ch);
                } else {
                    return;
                }
                s.message.clear();
                w.prevent_default();
                cx.notify();
            }));
        let code = otp_field::input(editor("code"), "Four-digit demo code")
            .gap_2()
            .children((0..4).map(|i| {
                otp_field::slot().child(
                    self.code
                        .chars()
                        .nth(i)
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "–".into()),
                )
            }))
            .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                if e.keystroke.modifiers.modified() {
                    return;
                }
                if e.keystroke.key == "backspace" {
                    s.code.pop();
                } else if s.code.len() < 4
                    && e.keystroke.key.len() == 1
                    && e.keystroke.key.as_bytes()[0].is_ascii_digit()
                {
                    s.code.push_str(&e.keystroke.key);
                } else {
                    return;
                }
                s.message.clear();
                w.prevent_default();
                cx.notify();
            }));
        form::root("project-form", "Create project")
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child("Project name")
            .child(name)
            .child("Demo code")
            .child(otp_field::root("otp", "Demo code").child(code))
            .child(ui::action("Create project", w, cx, |s: &mut Project| {
                s.message = if s.name.trim().is_empty() || s.code.len() != 4 {
                    "Enter a project name and four digits.".into()
                } else {
                    format!("Created {} in this preview.", s.name.trim())
                }
            }))
            .child(self.message.clone())
    }
}
