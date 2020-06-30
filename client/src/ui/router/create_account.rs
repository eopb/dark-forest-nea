use crate::{state, ui, updates};
use seed::{prelude::*, *};

use seed_style::*;

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::create_account::Fail>,
) -> Node<updates::Msg> {
    let user_name = |err| ui::form::text_with_error(model, "user_name", "Username...", err);
    let email = |err| ui::form::email_with_error(model, "email", "Email...", err);
    let password = |err| ui::form::password_with_error(model, "password", "Password...", err);
    ui::form::view(
        model,
        "/api/create-account",
        match error {
            Some(error) => match error {
                shared::data::create_account::Fail::AlreadyExists => vec![
                    user_name(Some("Username already taken.".to_owned())),
                    email(None),
                    password(None),
                ],
                shared::data::create_account::Fail::InvalidField(error) => vec![
                    user_name(error.user_name.map(|x| x.show("Username"))),
                    email(error.email.map(|x| x.show("Email"))),
                    password(error.password.map(|x| x.show("Password"))),
                ],
            },
            None => vec![user_name(None), email(None), password(None)],
        },
        "Create Account",
        vec![
            Node::from_html("Already have account? "),
            a![
                ui::style::button(model, 3),
                "Sign In.",
                attrs! {At::Href => shared::Route::SignIn(None)}
            ]
            .into_nodes(),
        ],
    )
}
