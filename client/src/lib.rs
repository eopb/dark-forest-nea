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

use shared::data::ResponseKind::{self, Binary};

pub const RESPONSE_KIND: ResponseKind = Binary;

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> state::Model {
    ui::style::global::init();

    orders
        .subscribe(routes::Route::update)
        // Always refresh token on load to keep token update.
        .send_msg(updates::Msg::RefreshTokenIfNeed)
        .stream(streams::interval(
            Duration::minutes(14)
                .whole_milliseconds()
                .try_into()
                .unwrap(),
            || updates::Msg::RefreshTokenIfNeed,
        ))
        .notify(UrlChanged(url));

    state::Model::new()
}

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    let _app = App::start("app", init, updates::update, ui::view);
}
