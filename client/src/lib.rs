#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::future_not_send,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn
)]

pub mod endpoint;
pub mod routes;
pub mod state;
pub mod ui;
pub mod updates;

pub use {endpoint::Endpoint, routes::Route};

use {
    seed::{app::subs::UrlChanged, prelude::*},
    time::Duration,
};

use std::convert::TryInto;

use shared::data::ResponseKind::{self, Json};

/// The kind of response body to expect from server endpoints.
pub const RESPONSE_KIND: ResponseKind = Json;

/// Setup process invoked when client is started.
fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> state::Model {
    ui::style::global::init();

    orders
        .subscribe(routes::Route::update)
        // Always refresh token on load to keep token update.
        .send_msg(updates::Msg::RefreshToken)
        .stream(streams::interval(
            Duration::minutes(14)
                .whole_milliseconds()
                .try_into()
                .unwrap(),
            || updates::Msg::RefreshToken,
        ))
        .notify(UrlChanged(url));

    state::Model::new()
}

/// This function is invoked by `init` function from Javascript and is the entry
/// point of our program.
#[wasm_bindgen(start)]
pub fn start() {
    let _app = App::start("app", init, updates::update, ui::view);
}
