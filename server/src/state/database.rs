pub mod project_store;
pub mod projects_list;
pub mod user;

pub use user::User;

use std::env;

use {once_cell::sync::Lazy, tracing::instrument};

/// Database URL has a password so we must store it as an environment variable.
static DB_URL: Lazy<String> = Lazy::new(|| env::var("DB_URL").unwrap());

/// Newtype for our `mongodb::Client` so we can implement our own methods.
#[derive(Debug, Clone)]
pub struct Database(mongodb::Client);

impl Database {
    /// Creates a connection with the database.
    #[instrument(level = "trace", err)]
    pub async fn new() -> tide::Result<Self> {
        let mongo = mongodb::Client::with_uri_str(&DB_URL).await?;
        Ok(Self(mongo))
    }
    /// Access to the underlying `mongodb` client.
    const fn mongo(&self) -> &mongodb::Client {
        &self.0
    }
    /// Get the main database.
    pub fn main(&self) -> mongodb::Database {
        self.mongo().database("testing-ground")
    }
}

/// The result of inserting an object to the database.
pub enum Insert {
    Success,
    AlreadyExists,
}
