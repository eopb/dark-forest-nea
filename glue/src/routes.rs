pub use crate::{data, qs};

use std::{fmt, string::ToString};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Route {
    Index,
    Explore,
    SignIn(Option<data::credentials::Fail>),
    CreateAccount,
    NewProject,
    NotFound,
    Api,
}

impl Default for Route {
    fn default() -> Self {
        Self::NotFound
    }
}

impl Into<String> for Route {
    fn into(self) -> String {
        match self {
            Self::Index => "/".to_string(),
            Self::Explore => "/explore".to_string(),
            Self::SignIn(fail) => qs::with_enum("/sign-in", &fail),
            Self::CreateAccount => "/create-account".to_string(),
            Self::NewProject => "/new-project".to_string(),
            Self::NotFound => panic!("Can not go to 404 route"),
            Self::Api => panic!("Can not go to an API route"),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path: String = (*self).into();
        write!(f, "{}", path)
    }
}
