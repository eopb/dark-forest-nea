//! Security related data.

use std::ops::Deref;
use {
    secrecy::{ExposeSecret, Secret, SecretString},
    serde::{Deserialize, Serialize, Serializer},
};

/// Wrap a type to add authentication to some data.
///
/// Useful for sending data over `POST` to endpoints that require an authorized
/// user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticated<T> {
    pub inner: T,
    pub token: Token,
}

impl<T> Authenticated<T> {
    pub const fn new(inner: T, token: Token) -> Self {
        Self { inner, token }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(#[serde(serialize_with = "serialize_secret")] SecretString);

pub(crate) fn serialize_secret<E: ExposeSecret<T>, T: Serialize, S: Serializer>(
    secret: &E,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    secret.expose_secret().serialize(serializer)
}

impl Deref for Token {
    type Target = SecretString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Token {
    fn default() -> Self {
        Self(Secret::new(String::new()))
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self(Secret::new(s))
    }
}
