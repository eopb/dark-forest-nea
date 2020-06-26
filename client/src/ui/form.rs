use std::fmt;

use crate::{
    state,
    ui::{self, View},
    updates,
};
use seed::{prelude::*, *};

use seed_style::{em, px, *};

#[derive(Eq, PartialEq, Copy, Clone)]
enum InputType {
    Text,
    Password,
    Submit,
    Email,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text => "text",
                Self::Password => "password",
                Self::Submit => "submit",
                Self::Email => "email",
            }
        )
    }
}

fn input(
    model: &state::Model,
    id: &str,
    input_type: InputType,
    placeholder: &str,
    error: Option<impl fmt::Display>,
) -> Vec<Node<updates::Msg>> {
    vec![
        vec![if let Some(error) = error {
            p![
                s().margin("0")
                    .margin_bottom(px(-15))
                    .width(px(600))
                    .text_align("left")
                    .font_size(em(2.9))
                    .color(model.theme.error()),
                error.to_string()
            ]
        } else {
            empty()
        }],
        ui::Bordered::new(input![
            attrs! {
                At::Id => id,
                At::Name => id,
                At::Type => input_type,

            },
            if InputType::Submit == input_type {
                attrs! {At::Value => placeholder}
            } else {
                attrs! {At::Placeholder => placeholder}
            },
            s().margin("0")
                .width("95%")
                .font_family("adobedia")
                .box_sizing("border-box")
                .border("none")
                .font_size(em(3))
                .background_color(model.theme.background())
                .color(model.theme.text()),
            if InputType::Password == input_type {
                s().pseudo(":not(:placeholder-shown)")
                    .font_size(em(1.5))
                    .font_family("prstart")
                    .margin("11px 7px")
            } else {
                s()
            }
        ])
        .inner(s().width(px(if InputType::Submit == input_type {
            300
        } else {
            600
        })))
        .view(model),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn text(model: &state::Model, id: &str, placeholder: &str) -> Vec<Node<updates::Msg>> {
    text_with_error(model, id, placeholder, Option::<String>::None)
}

pub fn password(model: &state::Model, id: &str, placeholder: &str) -> Vec<Node<updates::Msg>> {
    password_with_error(model, id, placeholder, Option::<String>::None)
}

pub fn email(model: &state::Model, id: &str, placeholder: &str) -> Vec<Node<updates::Msg>> {
    email_with_error(model, id, placeholder, Option::<String>::None)
}

pub fn text_with_error(
    model: &state::Model,
    id: &str,
    placeholder: &str,
    error: Option<impl fmt::Display>,
) -> Vec<Node<updates::Msg>> {
    input(model, id, InputType::Text, placeholder, error)
}

pub fn password_with_error(
    model: &state::Model,
    id: &str,
    placeholder: &str,
    error: Option<impl fmt::Display>,
) -> Vec<Node<updates::Msg>> {
    input(model, id, InputType::Password, placeholder, error)
}

pub fn email_with_error(
    model: &state::Model,
    id: &str,
    placeholder: &str,
    error: Option<impl fmt::Display>,
) -> Vec<Node<updates::Msg>> {
    input(model, id, InputType::Email, placeholder, error)
}

fn submit(model: &state::Model, placeholder: &str) -> Vec<Node<updates::Msg>> {
    input(
        model,
        "",
        InputType::Submit,
        placeholder,
        Option::<String>::None,
    )
}

pub fn view(
    model: &state::Model,
    action: impl fmt::Display,
    items: impl UpdateEl<updates::Msg>,
    submit_text: &str,
    note: impl UpdateEl<updates::Msg>,
) -> Node<updates::Msg> {
    div![form![
        attrs! {At::Action => action, At::Method => "post"},
        s().display("flex")
            .align_items("center")
            .flex_direction("column")
            .margin("auto"),
        items,
        submit(model, submit_text),
        ui::subheading(note)
    ]]
}
