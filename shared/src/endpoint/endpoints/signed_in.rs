use crate::{security, Endpoint, PostEndpoint};

use serde::{Deserialize, Serialize};

/// Endpoint used by the client to check if a user is signed-in.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SignedIn;

impl Endpoint for SignedIn {
    type Response = Res;
    const PATH: &'static str = "/signed-in";
}

impl PostEndpoint for SignedIn {
    type Requires = security::Token;
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Res {
    As(String),
    Not,
}

impl Res {
    pub fn is_signed_in(&self) -> bool {
        self != &Self::Not
    }
}
