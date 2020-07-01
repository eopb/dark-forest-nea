pub mod projects_list;
pub mod user;

pub use user::User;

use std::env;

#[derive(Debug)]
pub struct Database(mongodb::Client);

impl Database {
    pub async fn new() -> tide::Result<Self> {
        let mongo = mongodb::Client::with_uri_str(&env::var("DB_URL").unwrap()).await?;
        Ok(Self(mongo))
    }
    /// Access the mongodb client.
    const fn mongo(&self) -> &mongodb::Client {
        &self.0
    }
    pub fn main(&self) -> mongodb::Database {
        self.mongo().database("testing-ground")
    }
}

pub enum Insert {
    Success,
    AlreadyExists,
}
