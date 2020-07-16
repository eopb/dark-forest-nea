//! Security related data.

use serde::{Deserialize, Serialize};

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

pub type Token = String;
