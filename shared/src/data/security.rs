use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticated<T> {
    inner: T,
    token: String,
}

impl<T> Authenticated<T> {
    fn new(inner: T, token: String) -> Self {
        Self { inner, token }
    }
}

pub type Token = String;
