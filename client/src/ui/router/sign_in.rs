use crate::{state, ui, updates};
use seed::{prelude::*, *};

use seed_style::*;

use glue::data;

pub fn view(model: &state::Model, error: Option<data::credentials::Fail>) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/sign-in",
        vec![
            vec![
                if let Some(data::credentials::Fail::UserNotFound) = error {
                    ui::subheading(data::credentials::Fail::UserNotFound.to_string())
                } else {
                    empty()
                }
                .into_nodes(),
                ui::form::text(model, "user_name", "Username..."),
            ],
            vec![
                if let Some(data::credentials::Fail::IncorrectPassword) = error {
                    ui::subheading(data::credentials::Fail::IncorrectPassword.to_string())
                } else {
                    empty()
                }
                .into_nodes(),
                ui::form::password(model, "password", "Password..."),
            ],
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
