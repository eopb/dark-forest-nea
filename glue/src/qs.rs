pub use crate::data;

use serde::{Deserialize, Serialize};

use std::string::ToString;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Debug)]
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

pub(crate) fn with<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    match qs {
        Some(qs) => format!("{}?{}", base, {
            serde_qs::to_string(qs).expect("failed to parse qs")
        }),
        None => base.to_string(),
    }
}

pub(crate) fn with_enum<T: Serialize>(base: &str, qs: &Option<T>) -> String {
    with(base, &qs.as_ref().map(Single::new))
}

pub fn get_enum<T: for<'a> Deserialize<'a>>(qs: &str) -> Option<T> {
    get::<Single<T>>(qs).map(|x| x.value)
}

pub fn get<T: for<'a> Deserialize<'a>>(qs: &str) -> Option<T> {
    let config = serde_qs::Config::new(50, false);
    if !qs.is_empty() {
        Some(config.deserialize_str::<T>(qs).ok()?)
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
            with_enum(
                "/epic/path",
                &Some(data::credentials::Fail::IncorrectPassword)
            ),
            "/epic/path?value=IncorrectPassword".to_string()
        );
    }
}
