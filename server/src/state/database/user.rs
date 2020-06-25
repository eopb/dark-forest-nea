use std::convert::TryFrom;

use crate::util::BsonDoc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

impl BsonDoc for User {}

impl TryFrom<glue::CreateAccount> for User {
    type Error = ();
    fn try_from(value: glue::CreateAccount) -> Result<Self, Self::Error> {
        Ok(Self {
            user_name: value.user_name,
            email: value.email,
            password_hash: value.password,
        })
    }
}
