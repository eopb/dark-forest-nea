#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::future_not_send,
    clippy::must_use_candidate,
    clippy::empty_enum,
    clippy::missing_const_for_fn
)]

mod console;
pub mod endpoint;
pub mod routes;
pub mod state;
pub mod ui;
pub mod updates;

pub use {endpoint::Endpoint, routes::Route};

use {
    seed::{app::subs::UrlChanged, prelude::*},
    time::Duration,
    tracing::Level,
    tracing_subscriber::fmt::format::FmtSpan,
};

use std::convert::TryInto;

/// Key where to store the login token on `LocalStorage`.
pub static LOGIN_KEY: &str = "Login";

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
    if cfg!(debug_assertions) {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_span_events(FmtSpan::CLOSE)
            .without_time()
            .with_ansi(false)
            .with_writer(console::Write::new)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("no global subscriber has been set");
    }
    let _app = App::start("app", init, updates::update, ui::view);
}
