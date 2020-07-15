pub mod create_account;
pub mod index;
pub mod new_project;
pub mod not_found;
pub mod sign_in;

use crate::{state, updates};

use seed::{prelude::*, *};

/// Main router view showing items unique to a route.
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.0 {
        Some(ref route) => match route {
            shared::Route::Index => index::view(model),
            shared::Route::Explore => p!["explore"],
            shared::Route::SignIn => sign_in::view(model),
            shared::Route::CreateAccount => create_account::view(model),
            shared::Route::NewProject => new_project::view(model),
            shared::Route::Users { .. } => todo!(),
        },
        None => not_found::view(model),
    }
}
