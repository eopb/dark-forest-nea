use std::convert::TryFrom;

use {
    bcrypt::BcryptError,
    bson::doc,
    serde::{Deserialize, Serialize},
};

use crate::{
    state::{database::Insert, Database},
    util::BsonDoc,
};

/// The data stored about a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// Primary key.
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

impl BsonDoc for User {}

impl TryFrom<shared::data::create_account::Details> for User {
    type Error = BcryptError;
    fn try_from(value: shared::data::create_account::Details) -> Result<Self, Self::Error> {
        Ok(Self {
            user_name: value.user_name,
            email: value.email,
            password_hash: bcrypt::hash(value.password, bcrypt::DEFAULT_COST)?,
        })
    }
}

impl User {
    pub fn verify_credentials(
        &self,
        credentials: &shared::data::sign_in::Credentials,
    ) -> Result<bool, BcryptError> {
        Ok(self.user_name == credentials.user_name
            && bcrypt::verify(&credentials.password, &self.password_hash)?)
    }
}

impl Database {
    /// Collection where to store basic user information.
    pub fn users(&self) -> mongodb::Collection {
        self.main().collection("users")
    }

    /// Add a new user to the database.
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

    /// Get user information for user attached to a username.
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
