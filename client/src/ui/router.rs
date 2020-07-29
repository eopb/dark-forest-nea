pub mod create_account;
pub mod index;
pub mod new_project;
pub mod not_found;
pub mod sign_in;
pub mod users;

use crate::{state, updates};

use seed::{prelude::*, *};
use tracing::{error, instrument};

use shared::Route;

/// Main router view showing items unique to a route.
#[instrument(skip(model))]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.0 {
        Some(ref route) => match route {
            Route::Index => index::view(model),
            Route::Explore => p!["explore"],
            Route::SignIn => sign_in::view(model),
            Route::CreateAccount => create_account::view(model),
            Route::NewProject => new_project::view(model),
            Route::Users { user_name, nest } => users::view(model, user_name, nest.as_ref()),
        },
        None => not_found::view(model),
    }
}
