use std::{convert::TryInto, env};

use {
    bson::doc,
    chrono::{offset::Utc, Duration},
    jsonwebtoken::{encode, EncodingKey, Header},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    pub fn new(user: String) -> Self {
        Self {
            sub: user,
            company: "dark_forest".to_owned(),
            exp: (Utc::now() + Duration::days(Self::max_age_days()))
                .timestamp()
                .try_into()
                .expect("Should be convertible for the next 100 years"),
        }
    }
    pub fn get_token(&self) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
        )
    }
    /// Can't simply return duration due to time crate version miss-match with `chrono` and `cookie`
    pub const fn max_age_days() -> i64 {
        1
    }
}
