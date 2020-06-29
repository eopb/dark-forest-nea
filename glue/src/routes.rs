pub use crate::{data, qs};

use std::fmt;

/// An enum for all routes used by both server and client.
///
/// Routes can also store query strings.
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
                Self::Index => "/".to_owned(),
                Self::Explore => "/explore".to_owned(),
                Self::SignIn(fail) => qs::with_enum("/sign-in", fail),
                Self::CreateAccount(fail) => qs::with_enum("/create-account", fail),
                Self::NewProject => "/new-project".to_owned(),
            }
        )
    }
}
