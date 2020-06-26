use std::convert::TryFrom;

use crate::util::BsonDoc;

use {
    bcrypt::BcryptError,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

impl BsonDoc for User {}

impl TryFrom<glue::CreateAccount> for User {
    type Error = BcryptError;
    fn try_from(value: glue::CreateAccount) -> Result<Self, Self::Error> {
        Ok(Self {
            user_name: value.user_name,
            email: value.email,
            password_hash: bcrypt::hash(value.password, bcrypt::DEFAULT_COST)?,
        })
    }
}

impl User {
    pub fn verify_credentials(&self, credentials: &glue::Credentials) -> Result<bool, BcryptError> {
        Ok(self.user_name == credentials.user_name
            && bcrypt::verify(&credentials.password, &self.password_hash)?)
    }
}
