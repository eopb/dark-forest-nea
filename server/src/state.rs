//! Shared application state.

pub mod database;

pub use database::Database;

/// Current state of the application.
#[derive(Debug)]
pub struct State {
    database: Database,
}

impl State {
    /// Create a new instance of `State`.
    pub async fn new() -> tide::Result<Self> {
        Ok(Self {
            database: Database::new().await?,
        })
    }
    pub fn database(&self) -> &Database {
        &self.database
    }
}
