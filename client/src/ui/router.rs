mod index;
mod not_found;
mod sign_in;

use crate::{routes::Route, state, updates};
use seed::{prelude::*, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route {
        Route::Index => index::view(model),
        Route::Explore => p!["explore"],
        Route::SignIn => sign_in::view(model),
        Route::NotFound => not_found::view(model),
    }
}
