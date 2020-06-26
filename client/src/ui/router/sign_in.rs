use crate::{
    state,
    ui::{self, form::if_equal_display},
    updates,
};
use seed::{prelude::*, *};

use seed_style::*;

use glue::data::credentials::Fail::{IncorrectPassword, UserNotFound};

pub fn view(
    model: &state::Model,
    error: Option<glue::data::credentials::Fail>,
) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/sign-in",
        vec![
            ui::form::text_with_error(
                model,
                "user_name",
                "Username...",
                if_equal_display(error, &UserNotFound),
            ),
            ui::form::password_with_error(
                model,
                "password",
                "Password...",
                if_equal_display(error, &IncorrectPassword),
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
