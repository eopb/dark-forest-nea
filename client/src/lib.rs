pub mod state;
pub mod ui;
pub mod updates;

use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<updates::Msg>) -> state::Model {
    state::Model::default()
}

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, updates::update, ui::view);
}
