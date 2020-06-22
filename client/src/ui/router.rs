mod index;
mod not_found;

use crate::{routes::Route, state, updates};
use seed::{prelude::*, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route {
        Route::Index => index::view(model),
        Route::Explore => p!["explore"],
        Route::NotFound => not_found::view(model),
    }
}
