#![allow(clippy::wildcard_imports, clippy::future_not_send)]

pub mod endpoint;
pub mod routes;
pub mod state;
pub mod ui;
pub mod updates;

pub use endpoint::Endpoint;

use seed::prelude::*;

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<updates::Msg>) -> AfterMount<state::Model> {
    ui::style::global::init();

    AfterMount::new(state::Model::default()).url_handling(UrlHandling::PassToRoutes)
}

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::builder(updates::update, ui::view)
        .routes(routes::Route::update)
        .after_mount(init)
        .build_and_start();
}
