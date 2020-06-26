use crate::Endpoint;

pub struct SignOut;

impl Endpoint for SignOut {
    const PATH: &'static str = "/api/sign-out";
}
