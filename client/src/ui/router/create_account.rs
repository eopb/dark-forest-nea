use crate::{state, ui, updates, RESPONSE_KIND};

use {
    seed::{prelude::*, *},
    seed_style::*,
};

use shared::Endpoint;

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::create_account::Fail>,
) -> Node<updates::Msg> {
    let user_name = |err| {
        ui::form::InputBuilder::text()
            .id("user_name")
            .placeholder("Username...")
            .error(err)
            .view(model, |_| None)
    };
    let email = |err| {
        ui::form::InputBuilder::email()
            .id("email")
            .placeholder("Email...")
            .error(err)
            .view(model, |_| None)
    };
    let password = |err| {
        ui::form::InputBuilder::password()
            .id("password")
            .placeholder("Password...")
            .error(err)
            .view(model, |_| None)
    };
    ui::form::view(
        model,
        match error {
            Some(error) => match error {
                shared::data::create_account::Fail::AlreadyExists => vec![
                    user_name(&Some("Username already taken.".to_owned())),
                    email(&None),
                    password(&None),
                ],
                shared::data::create_account::Fail::InvalidField(error) => vec![
                    user_name(&error.user_name.map(|x| x.show("Username"))),
                    email(&error.email.map(|x| x.show("Email"))),
                    password(&error.password.map(|x| x.show("Password"))),
                ],
            },
            None => vec![user_name(&None), email(&None), password(&None)],
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
        |_| None,
    )
}
