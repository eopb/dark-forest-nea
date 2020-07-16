#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn
)]

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

use shared::endpoint::{
    create_account::CreateAccount, hello::Hello, new_project::NewProject,
    refresh_token::RefreshToken, sign_in::SignIn, signed_in::SignedIn,
};

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    femme::with_level(log::LevelFilter::Debug);

    let state = State::new().await?;
    let mut app = tide::with_state(state);

    // By default all routes should be handled by the client if not specified
    // otherwise.
    app.at("/").get(routes::index);
    app.at("*").get(routes::index);

    // Static files.
    app.at("/pkg").serve_dir("../client/pkg")?;
    app.at("/fonts").serve_dir("../client/fonts")?;

    // Redirect hackers to YouTube.
    app.at("/.env")
        .get(Redirect::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));

    // REST endpoints.
    Hello::apply(&mut app);
    SignIn::apply(&mut app);
    CreateAccount::apply(&mut app);
    SignedIn::apply(&mut app);
    NewProject::apply(&mut app);
    RefreshToken::apply(&mut app);

    app.listen("localhost:8081").await?;

    Ok(())
}
