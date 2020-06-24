use crate::{state, ui, updates};
use seed::{prelude::*, *};

use seed_style::*;

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/create-account",
        vec![
            ui::form::text(model, "user_name", "Username..."),
            ui::form::email(model, "email", "Email..."),
            ui::form::password(model, "password", "Password..."),
        ],
        "Create Account",
        vec![
            Node::from_html("Already have account? "),
            a![
                ui::style::button(model, 3),
                "Sign In.",
                attrs! {At::Href => glue::Route::SignIn}
            ]
            .into_nodes(),
        ],
    )
}
