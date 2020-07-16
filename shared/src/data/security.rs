use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticated<T> {
    pub inner: T,
    pub token: String,
}

impl<T> Authenticated<T> {
    pub const fn new(inner: T, token: String) -> Self {
        Self { inner, token }
    }
}

pub type Token = String;
