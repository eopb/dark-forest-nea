#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc, clippy::similar_names)]

mod endpoint;
#[allow(clippy::must_use_candidate)]
mod routes;
mod state;
pub mod util;

pub use endpoint::{GetEndpoint, PostEndpoint};
pub use state::State;

use {dotenv::dotenv, tide::Redirect};

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

    glue::Hello::apply(&mut app);
    glue::Credentials::apply(&mut app);
    glue::CreateAccount::apply(&mut app);

    app.listen("localhost:8081").await?;

    Ok(())
}
