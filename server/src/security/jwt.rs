//! Dark forest uses JSON web tokens for authentication.

use std::env;

use {
    bson::doc,
    chrono::{offset::Utc, Duration},
    jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation},
    once_cell::sync::Lazy,
    secrecy::{ExposeSecret, Secret, SecretVec},
    serde::{Deserialize, Serialize},
    tracing::instrument,
};

use shared::security::Token;

/// Secret bytes used to create tokens. These are stored as an environment
/// variable.
static SECRET: Lazy<SecretVec<u8>> =
    Lazy::new(|| Secret::new(env::var("SECRET").unwrap().as_bytes().to_vec()));

/// `Claims` is the data we are going to encode in our tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Sub is where we are going to store the username of the user.
    pub sub: String,
    company: String,
    // must have this rename for `jsonwebtoken` to validate correctly.
    #[serde(rename(serialize = "exp", deserialize = "exp"))]
    pub expires: i64,
}

impl Claims {
    /// Create a token for a user.
    ///
    /// Only use for authenticated users.
    #[instrument(level = "trace")]
    pub fn new(user: String) -> Self {
        Self {
            sub: user,
            company: "dark_forest".to_owned(),
            expires: (Utc::now() + Duration::minutes(Self::max_age_minutes())).timestamp(),
        }
    }
    /// Encodes a claim into a token string.
    #[instrument(level = "trace")]
    pub fn get_token(&self) -> jsonwebtoken::errors::Result<Token> {
        Ok(encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(SECRET.expose_secret()),
        )?
        .into())
    }
    /// Decodes a token to produce the underlying claim.
    #[instrument(level = "trace", err)]
    pub fn decode_token(token: &Token) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        decode::<Self>(
            token.expose_secret(),
            &DecodingKey::from_secret(SECRET.expose_secret()),
            &Validation::default(),
        )
    }
    /// Can't simply return duration due to time crate version miss-match with
    /// `chrono` and `cookie`
    pub const fn max_age_minutes() -> i64 {
        15
    }
}
