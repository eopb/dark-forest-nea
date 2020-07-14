use crate::Endpoint;

use serde::{Deserialize, Serialize};

/// Endpoint used by the client to check if a user is signed-in.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum SignedIn {
    As(String),
    Not,
}

impl Endpoint for SignedIn {
    type Response = ();
    const PATH: &'static str = "/signed-in";
}

impl SignedIn {
    pub fn is_signed_in(&self) -> bool {
        self != &Self::Not
    }
}
