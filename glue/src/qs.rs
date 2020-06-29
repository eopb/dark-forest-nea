//! Code wrapping `serde_qs` to provide better support for enums and more query string related things.

pub use crate::data;

use serde::{Deserialize, Serialize};

/// `serde_qs` only supports `struct` at the top level so this type wraps a type in a struct.
#[derive(Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Debug)]
struct Single<T> {
    pub value: T,
}
impl<T> Single<T> {
    /// Wraps a type in a `Single`
    fn new(value: T) -> Self {
        value.into()
    }
}

impl<T> From<T> for Single<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

/// Adds query string to URL
///
/// # Panics
/// This will panic if `T` is an `enum`.
pub(crate) fn with<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    match qs {
        Some(qs) => format!("{}?{}", base, {
            serde_qs::to_string(qs).expect("failed to parse qs")
        }),
        None => base.to_owned(),
    }
}

/// Adds query string to URL wrapping the query string in `Single` to allow support for enums
pub(crate) fn with_enum<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    with(base, &qs.as_ref().map(Single::new))
}

/// Deserializes a query string.
pub fn get<T: for<'a> Deserialize<'a>>(qs: &str) -> Option<T> {
    let config = serde_qs::Config::new(50, false);
    if !qs.is_empty() {
        Some(config.deserialize_str::<T>(qs).ok()?)
    } else {
        None
    }
}

/// Deserializes a query string that was wrapped in `Single`.
pub fn get_enum<T: for<'a> Deserialize<'a>>(qs: &str) -> Option<T> {
    Some(get::<Single<T>>(qs)?.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::create_account::CreateAccount;

    #[test]
    fn with_() {
        assert_eq!(
            with_enum(
                "/epic/path",
                &Some(data::credentials::Fail::IncorrectPassword)
            ),
            "/epic/path?value=IncorrectPassword".to_owned()
        );
        // Don't worry I never serialize passwords to qs in real code.
        assert_eq!(
            with("/epic/path", &Some(CreateAccount::mock())),
            "/epic/path?user_name=Ethan&email=example%40example.com&password=hunter2".to_owned()
        );
    }
    #[test]
    fn get_() {
        assert_eq!(
            get_enum("value=IncorrectPassword"),
            Some(data::credentials::Fail::IncorrectPassword)
        );
        // Don't worry I never serialize passwords to qs in real code.
        assert_eq!(
            get("user_name=Ethan&email=example%40example.com&password=hunter2"),
            Some(CreateAccount::mock())
        );
    }
    #[test]
    fn empty() {
        assert_eq!(get_enum::<data::credentials::Fail>(""), None);
        assert_eq!(get::<CreateAccount>(""), None);
        assert_eq!(
            with_enum::<data::credentials::Fail>("/epic/path", &None),
            "/epic/path".to_owned()
        );
        assert_eq!(
            with::<CreateAccount>("/epic/path", &None),
            "/epic/path".to_owned()
        );
    }
}
