pub use crate::data;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::string::ToString;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct Single<T> {
    pub value: T,
}
impl<T> Single<T> {
    fn new(value: T) -> Self {
        value.into()
    }
}

impl<T> From<T> for Single<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Route {
    Index,
    Explore,
    SignIn(Option<data::credentials::Fail>),
    CreateAccount,
    NewProject,
    NotFound,
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
            Self::SignIn(fail) => with_enum_qs("/sign-in", &fail),
            Self::CreateAccount => "/create-account".to_string(),
            Self::NewProject => "/new-project".to_string(),
            Self::NotFound => panic!("Can not go to 404 route"),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path: String = (*self).into();
        write!(f, "{}", path)
    }
}

pub(crate) fn with_qs<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    match qs {
        Some(qs) => format!("{}?{}", base, {
            serde_qs::to_string(qs).expect("failed to parse qs")
        }),
        None => base.to_string(),
    }
}

pub(crate) fn with_enum_qs<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    with_qs(base, &qs.as_ref().map(Single::new))
}

pub fn get_enum_qs<T: for<'a> Deserialize<'a>>(qs: &str) -> Option<T> {
    if !qs.is_empty() {
        Some(serde_qs::from_str::<Single<T>>(qs).ok()?.value)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn with_qs_t() {
        assert_eq!(
            with_enum_qs("/epic/path", &Some(data::credentials::Fail::Success)),
            "/epic/path?value=Success".to_string()
        );
    }
}
