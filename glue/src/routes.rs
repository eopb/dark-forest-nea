pub use crate::{data, qs};

use std::{fmt, string::ToString};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Route {
    Index,
    Explore,
    SignIn(Option<data::credentials::Fail>),
    CreateAccount(Option<data::create_account::Fail>),
    NewProject,
}

impl Default for Route {
    fn default() -> Self {
        Self::Index
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Index => "/".to_string(),
                Self::Explore => "/explore".to_string(),
                Self::SignIn(fail) => qs::with_enum("/sign-in", &fail),
                Self::CreateAccount(fail) => qs::with_enum("/create-account", &fail),
                Self::NewProject => "/new-project".to_string(),
            }
        )
    }
}
