use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    Success,
    UserNotFound,
    IncorrectPassword,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub user_name: String,
    pub password: String,
}

impl Endpoint for Credentials {
    const PATH: &'static str = "/api/sign-in";
}
