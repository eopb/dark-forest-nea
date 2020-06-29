use crate::Endpoint;

/// Endpoint for signing-out a user.
pub struct SignOut;

impl Endpoint for SignOut {
    const PATH: &'static str = "/api/sign-out";
}
