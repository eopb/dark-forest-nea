pub mod user;

pub use user::User;

use {bson::doc, std::env};

use crate::util::BsonDoc;

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
    pub fn users(&self) -> mongodb::Collection {
        self.main().collection("users")
    }

    pub async fn add_user(&self, user: User) -> tide::Result<Insert> {
        let filter = doc! { "_id": &user.user_name};
        let cursor = self.users().find_one(filter, None).await?;

        Ok(if cursor.is_none() {
            self.users().insert_one(user.as_bson()?, None).await?;
            Insert::Success
        } else {
            Insert::AlreadyExists
        })
    }

    pub async fn get_user(&self, user_name: &str) -> tide::Result<Option<User>> {
        Ok(
            if let Some(x) = self
                .users()
                .find_one(doc! { "_id": user_name}, None)
                .await?
            {
                Some(bson::from_bson(bson::Bson::Document(x))?)
            } else {
                None
            },
        )
    }
}

pub enum Insert {
    Success,
    AlreadyExists,
}
