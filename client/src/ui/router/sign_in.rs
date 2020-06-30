use crate::{state, ui, updates};

use {
    seed::{prelude::*, *},
    seed_style::*,
};

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::credentials::Fail>,
) -> Node<updates::Msg> {
    use shared::data::credentials::Fail::{IncorrectPassword, UserNotFound};
    let user_name = |err| ui::form::text_with_error(model, "user_name", "Username...", err);
    let password = |err| ui::form::password_with_error(model, "password", "Password...", err);
    ui::form::view(
        model,
        "/api/sign-in",
        vec![
            user_name(if let Some(UserNotFound) = error {
                error
            } else {
                None
            }),
            password(if let Some(IncorrectPassword) = error {
                error
            } else {
                None
            }),
        ],
        "Sign In",
        vec![
            Node::from_html("Don't have an account? "),
            a![
                ui::style::button(model, 3),
                "Create account.",
                attrs! {At::Href => shared::Route::CreateAccount(None)}
            ]
            .into_nodes(),
        ],
    )
}
