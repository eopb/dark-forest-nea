use crate::Endpoint;
use serde::{Deserialize, Serialize};

/// Endpoint for getting a new jwt.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RefreshToken;

impl Endpoint for RefreshToken {
    const PATH: &'static str = "/refresh-token";
}
