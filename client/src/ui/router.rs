mod create_account;
mod index;
mod new_project;
mod not_found;
mod sign_in;

use crate::{state, updates};
use seed::{prelude::*, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.clone().into() {
        Ok(route) => match route {
            glue::Route::Index => index::view(model),
            glue::Route::Explore => p!["explore"],
            glue::Route::SignIn(error) => sign_in::view(model, error),
            glue::Route::CreateAccount(error) => create_account::view(model, error),
            glue::Route::NewProject => new_project::view(model),
        },
        Err(_) => not_found::view(model),
    }
}
