use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum SingedIn {
    As(String),
    Not,
}

impl Endpoint for SingedIn {
    const PATH: &'static str = "/api/signed-in";
}

impl SingedIn {
    pub fn is_signed_in(&self) -> bool {
        self != &Self::Not
    }
}
