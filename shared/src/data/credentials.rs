use crate::Endpoint;

use std::fmt;

use serde::{Deserialize, Serialize};

/// A users credentials used to sign-in.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Credentials {
    pub user_name: String,
    pub password: String,
}

impl Endpoint for Credentials {
    type Response = Result<Token, Fail>;
    const PATH: &'static str = "/sign-in";
}


type Token = String;

/// Reasons signing-in may fail.
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
