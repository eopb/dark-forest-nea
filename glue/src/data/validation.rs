use serde::{Deserialize, Serialize};

use crate::Endpoint;

pub trait Post: Endpoint {
    type Invalid: Serialize + for<'a> Deserialize<'a>;
    fn validate(&self) -> Result<(), Self::Invalid>;
}
pub struct Validation {
    pub max_length: Option<usize>,
    pub min_length: usize,
    pub must_be_ascii: bool,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum Error {
    TooLong,
    TooShort,
    NotAscii,
}
use Error::*;

impl Error {
    pub fn show(&self, field: &str) -> String {
        format!(
            "{} {}.",
            field,
            match self {
                TooLong => "must be shorter",
                TooShort => "must be longer",
                NotAscii => "must only contain ASCII characters",
            }
        )
    }
}

impl Default for Validation {
    fn default() -> Self {
        Self {
            max_length: Some(40),
            min_length: 1,
            must_be_ascii: false,
        }
    }
}

impl Validation {
    pub fn minimal() -> Self {
        Self {
            max_length: None,
            min_length: 0,
            must_be_ascii: false,
        }
    }
    pub fn of(&self, s: &str) -> Result<(), Error> {
        let length = s.len();
        if length < self.min_length {
            return Err(TooShort);
        };
        if let Some(max) = self.max_length {
            if length > max {
                return Err(TooLong);
            }
        }
        if self.must_be_ascii && !s.is_ascii() {
            return Err(NotAscii);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Error::*, Validation};
    #[test]
    fn length() {
        assert_eq!(Validation::default().of(""), Err(TooShort));
        assert_eq!(
            Validation {
                max_length: Some(5),
                ..Validation::default()
            }
            .of("Hello w"),
            Err(TooLong)
        );
    }
    #[test]
    fn ascii() {
        let v = Validation {
            must_be_ascii: true,
            ..Validation::default()
        };
        assert_eq!(v.of("τ > π"), Err(NotAscii));
        assert_eq!(v.of("tau > pi"), Ok(()));
    }
}
