use std::fmt;

use crate::{
    state,
    ui::{self, View},
    updates,
};
use seed::{prelude::*, *};

use seed_style::{em, px, vh, *};

#[derive(Eq, PartialEq)]
enum InputType {
    Text,
    Password,
    Submit,
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
            }
        )
    }
}

fn input(
    model: &state::Model,
    id: &str,
    input_type: InputType,
    placeholder: &str,
) -> Vec<Node<updates::Msg>> {
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
    .view(model)
}

pub fn text(model: &state::Model, id: &str, placeholder: &str) -> Vec<Node<updates::Msg>> {
    input(model, id, InputType::Text, placeholder)
}

pub fn password(model: &state::Model, id: &str, placeholder: &str) -> Vec<Node<updates::Msg>> {
    input(model, id, InputType::Password, placeholder)
}

pub fn submit(model: &state::Model, placeholder: &str) -> Vec<Node<updates::Msg>> {
    input(model, "", InputType::Submit, placeholder)
}
