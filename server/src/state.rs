//! Shared application state.

use std::env;

#[derive(Debug)]
pub struct State {
    pub db: mongodb::Client,
}

impl State {
    /// Create a new instance of `State`.
    pub async fn new() -> tide::Result<Self> {
        let mongo = mongodb::Client::with_uri_str(&env::var("DB_URL").unwrap()).await?;
        Ok(Self { db: mongo })
    }
    /// Access the mongodb client.
    const fn mongo(&self) -> &mongodb::Client {
        &self.db
    }
    pub fn db(&self) -> mongodb::Database {
        self.mongo().database("testing-ground")
    }
}
