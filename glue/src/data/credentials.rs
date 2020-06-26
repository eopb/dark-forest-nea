use crate::Endpoint;

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    UserNotFound,
    IncorrectPassword,
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UserNotFound => "User not found",
                Self::IncorrectPassword => "Incorrect password",
            }
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub user_name: String,
    pub password: String,
}

impl Endpoint for Credentials {
    const PATH: &'static str = "/api/sign-in";
}
