#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn
)]

pub mod cookie;
pub mod endpoint;
mod routes;
pub mod security;
mod state;
pub mod util;

pub use state::State;

use {
    dotenv::dotenv,
    endpoint::{Get, Post},
    tide::Redirect,
};

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    femme::with_level(log::LevelFilter::Debug);

    let state = State::new().await?;
    let mut app = tide::with_state(state);

    app.at("/").get(routes::index);
    app.at("*").get(routes::index);

    app.at("/pkg").serve_dir("../client/pkg")?;
    app.at("/fonts").serve_dir("../client/fonts")?;

    // Redirect hackers to YouTube.
    app.at("/.env")
        .get(Redirect::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));

    shared::Hello::apply(&mut app);
    shared::Credentials::apply(&mut app);
    shared::CreateAccount::apply(&mut app);
    shared::SignedIn::apply(&mut app);
    shared::SignOut::apply(&mut app);
    shared::NewProject::apply(&mut app);
    shared::RefreshToken::apply(&mut app);

    app.listen("localhost:8081").await?;

    Ok(())
}
