use crate::{
    data::{create_account, security},
    Endpoint, PostEndpoint,
};

use std::fmt;

use serde::{Deserialize, Serialize};

pub struct SignIn;

impl Endpoint for SignIn {
    type Response = Result<security::Token, Fail>;
    const PATH: &'static str = "/sign-in";
}

impl PostEndpoint for SignIn {
    type Requires = Credentials;
}
/// A users credentials used to sign-in.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Credentials {
    pub user_name: String,
    pub password: String,
}

/// Reasons signing-in may fail.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    UserNotFound,
    IncorrectPassword,
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::UserNotFound => "User not found",
            Self::IncorrectPassword => "Incorrect password",
        })
    }
}

impl From<create_account::Details> for Credentials {
    fn from(
        create_account::Details {
            user_name,
            password,
            ..
        }: create_account::Details,
    ) -> Self {
        Self {
            user_name,
            password,
        }
    }
}
