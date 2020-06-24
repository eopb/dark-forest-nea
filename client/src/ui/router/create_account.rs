use crate::{
    state,
    ui::{self, View},
    updates, Route,
};
use seed::{prelude::*, *};

use seed_style::{em, px, vh, *};

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
                attrs! {At::Href => Route::SignIn.go_to()}
            ]
            .into_nodes(),
        ],
    )
}
