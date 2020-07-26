//! Shared application state.

use tracing::instrument;

pub mod database;

pub use database::Database;

/// Current state of the application.
#[derive(Debug, Clone)]
pub struct State {
    database: Database,
}

impl State {
    /// Create a new instance of `State`.
    #[instrument(level = "trace")]
    pub async fn new() -> tide::Result<Self> {
        Ok(Self {
            database: Database::new().await?,
        })
    }
    pub fn database(&self) -> &Database {
        &self.database
    }
}
