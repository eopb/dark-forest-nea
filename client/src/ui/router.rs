mod create_account;
mod index;
mod new_project;
mod not_found;
mod sign_in;

use crate::{state, updates};
use seed::{prelude::*, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.into() {
        glue::Route::Index => index::view(model),
        glue::Route::Explore => p!["explore"],
        glue::Route::SignIn(_) => sign_in::view(model),
        glue::Route::NotFound => not_found::view(model),
        glue::Route::CreateAccount => create_account::view(model),
        glue::Route::NewProject => new_project::view(model),
    }
}
