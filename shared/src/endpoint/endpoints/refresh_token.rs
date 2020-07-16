use crate::{security::Token, Endpoint, PostEndpoint};
use serde::{Deserialize, Serialize};

/// Endpoint for getting a new jwt.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RefreshToken;

impl Endpoint for RefreshToken {
    type Response = Option<Token>;
    const PATH: &'static str = "/refresh-token";
}

impl PostEndpoint for RefreshToken {
    type Requires = Token;
}
