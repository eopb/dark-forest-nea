//! Items used to create forms.

use std::{fmt, string::ToString};

use crate::{
    state,
    ui::{self, View},
    updates,
};

use {
    seed::{prelude::*, *},
    seed_style::{em, pc, px, *},
};

/// Builder for creating input boxes.
pub struct InputBuilder {
    input_type: InputType,
    id: Option<String>,
    placeholder: Option<String>,
    error: Option<String>,
    value: Option<String>,
    label: Option<String>,
    width: CssWidth,
    font_size: ExactLength,
}

impl InputBuilder {
    fn new(input_type: InputType) -> Self {
        Self {
            input_type,
            id: None,
            placeholder: None,
            error: None,
            value: None,
            label: None,
            width: px(if InputType::Submit == input_type {
                300
            } else {
                600
            })
            .into(),
            font_size: em(2.9),
        }
    }
    /// Text box.
    pub fn text() -> Self {
        Self::new(InputType::Text)
    }

    /// Textarea.
    pub fn text_area() -> Self {
        Self::new(InputType::TextArea)
    }

    /// Password input box.
    pub fn password() -> Self {
        Self::new(InputType::Password)
    }

    /// Email input box.
    pub fn email() -> Self {
        Self::new(InputType::Email)
    }

    /// Submit button.
    pub fn submit() -> Self {
        Self::new(InputType::Submit)
    }

    pub fn id(mut self, id: impl fmt::Display) -> Self {
        self.id = Some(id.to_string());
        self
    }
    pub fn placeholder(mut self, placeholder: impl fmt::Display) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }
    pub fn error(mut self, error: &Option<impl fmt::Display>) -> Self {
        self.error = error.as_ref().map(ToString::to_string);
        self
    }
    pub fn value(mut self, value: impl fmt::Display) -> Self {
        self.value = Some(value.to_string());
        self
    }
    pub fn label(mut self, label: impl fmt::Display) -> Self {
        self.label = Some(label.to_string());
        self
    }
    pub fn width(mut self, width: impl Into<CssWidth>) -> Self {
        self.width = width.into();
        self
    }
    pub fn font_size(mut self, font_size: impl Into<ExactLength>) -> Self {
        self.font_size = font_size.into();
        self
    }
    pub fn view(
        self,
        model: &state::Model,
        update_msg: impl Fn(String) -> Option<updates::Msg> + Clone + 'static,
    ) -> Vec<Node<updates::Msg>> {
        vec![
            vec![{
                let style = |color| {
                    s().margin("0")
                        .margin_bottom(px(-15))
                        .width(self.width.clone())
                        .text_align_left()
                        .font_size(self.font_size.clone())
                        .color(color)
                };

                match (&self.error, &self.label) {
                    (Some(error), _) => label![style(model.theme.error()), error],
                    (_, Some(label)) => label![style(model.theme.text()), label],
                    (None, None) => empty(),
                }
            }],
            ui::Bordered::new(custom![
                if InputType::TextArea == self.input_type {
                    Tag::TextArea
                } else {
                    Tag::Input
                },
                self.id.as_ref().map(|id| attrs! {
                        At::Id => id,
                        At::Name => id,
                }),
                if InputType::TextArea == self.input_type {
                    attrs! {}
                } else {
                    attrs! {
                        At::Type => self.input_type,
                    }
                },
                self.placeholder.as_ref().map(|placeholder| {
                    attrs! {At::Placeholder => placeholder}
                }),
                self.value.as_ref().map(|value| {
                    attrs! {At::Value => value}
                }),
                if InputType::Submit == self.input_type {
                    input_ev(Ev::Click, update_msg)
                } else {
                    input_ev(Ev::Input, update_msg)
                },
                s().margin("0")
                    .width(pc(95))
                    .font_family("adobedia")
                    .box_sizing_border_box()
                    .border("none")
                    .font_size(self.font_size)
                    .background_color(model.theme.background())
                    .color(model.theme.text())
                    .resize("vertical"),
                if InputType::Password == self.input_type {
                    s().pseudo(":not(:placeholder-shown)")
                        .font_size(em(1.5))
                        .font_family("prstart")
                        .margin("11px 7px")
                } else {
                    s()
                }
            ])
            .outer(s().padding_left(px(0)).padding_right(px(0)))
            .inner(s().width(self.width))
            .view(model),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// Type of item to accept.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum InputType {
    Text,
    Password,
    Submit,
    TextArea,
    Email,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Submit => "submit",
            Self::Email => "email",
            Self::TextArea => unreachable!("Text areas should not be used in that way."),
        })
    }
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}
/// Full form with support of multiple input boxes and a submit button.
pub fn view(
    model: &state::Model,
    items: impl UpdateEl<updates::Msg>,
    submit_text: &str,
    note: impl UpdateEl<updates::Msg>,
    update_msg: impl Fn(String) -> Option<updates::Msg> + Clone + 'static,
) -> Node<updates::Msg> {
    div![form![
        attrs! {At::OnSubmit => "return false;"},
        s().display("flex")
            .align_items("center")
            .flex_direction("column")
            .margin("auto"),
        items,
        InputBuilder::submit()
            .value(submit_text)
            .view(model, update_msg),
        ui::subheading(note)
    ]]
}
