use crate::{state, ui, updates, Route};
use seed::{prelude::*, *};

use seed_style::*;

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/sign-in",
        vec![
            ui::form::text(model, "user_name", "Username..."),
            ui::form::password(model, "password", "Password..."),
        ],
        "Sign In",
        vec![
            Node::from_html("Don't have an account? "),
            a![
                ui::style::button(model, 3),
                "Create account.",
                attrs! {At::Href => Route::CreateAccount.go_to()}
            ]
            .into_nodes(),
        ],
    )
}
