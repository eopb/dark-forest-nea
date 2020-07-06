mod create_account;
mod index;
mod new_project;
mod not_found;
mod sign_in;

use crate::{state, updates};

use seed::{prelude::*, *};

/// Main router view showing items unique to a route.
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.0 {
        Some(ref route) => match route {
            shared::Route::Index => index::view(model),
            shared::Route::Explore => p!["explore"],
            shared::Route::SignIn(error) => sign_in::view(model, error.as_ref()),
            shared::Route::CreateAccount(error) => create_account::view(model, error.as_ref()),
            shared::Route::NewProject(error) => new_project::view(model, error.as_ref()),
            shared::Route::Users { .. } => todo!(),
        },
        None => not_found::view(model),
    }
}
