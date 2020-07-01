use std::env;

use {
    bson::doc,
    chrono::{offset::Utc, Duration},
    jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation},
    serde::{Deserialize, Serialize},
};

// `Claims` is the data we are going to encode in our tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Sub is where we are going to store the username of the user.
    pub sub: String,
    company: String,
    // must have this rename for `jsonwebtoken` to validate correctly.
    #[serde(rename(serialize = "exp", deserialize = "exp"))]
    expires: i64,
}

impl Claims {
    pub fn new(user: String) -> Self {
        Self {
            sub: user,
            company: "dark_forest".to_owned(),
            expires: (Utc::now() + Duration::days(Self::max_age_days())).timestamp(),
        }
    }
    pub fn get_token(&self) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(Self::secret().as_bytes()),
        )
    }
    // Decodes a token to produce the underlying claim.
    pub fn decode_token(token: &str) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        decode::<Self>(
            token,
            &DecodingKey::from_secret(Self::secret().as_bytes()),
            &Validation::default(),
        )
    }
    /// Can't simply return duration due to time crate version miss-match with `chrono` and `cookie`
    pub const fn max_age_days() -> i64 {
        1
    }

    fn secret() -> String {
        env::var("SECRET").unwrap()
    }
}
