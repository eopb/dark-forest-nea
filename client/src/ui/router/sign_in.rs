use crate::{state, ui, updates};
use seed::{prelude::*, *};

use seed_style::{em, px, *};

use glue::data;

pub fn view(model: &state::Model, error: Option<data::credentials::Fail>) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/sign-in",
        vec![
            ui::form::text_with_error(
                model,
                "user_name",
                "Username...",
                if_equal_display(error, data::credentials::Fail::UserNotFound),
            ),
            ui::form::password_with_error(
                model,
                "password",
                "Password...",
                if_equal_display(error, data::credentials::Fail::IncorrectPassword),
            ),
        ],
        "Sign In",
        vec![
            Node::from_html("Don't have an account? "),
            a![
                ui::style::button(model, 3),
                "Create account.",
                attrs! {At::Href => glue::Route::CreateAccount}
            ]
            .into_nodes(),
        ],
    )
}

fn if_equal_display<T: ToString + PartialEq>(option: Option<T>, eq_to: T) -> Option<String> {
    option.and_then(|x| {
        if x == eq_to {
            Some(x.to_string())
        } else {
            None
        }
    })
}
