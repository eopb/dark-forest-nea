use crate::Endpoint;

/// Endpoint for signing-out a user.
pub struct SignOut;

impl Endpoint for SignOut {
    type Response = ();
    const PATH: &'static str = "/sign-out";
}
