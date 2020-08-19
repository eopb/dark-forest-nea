//! Validation and sanity checks for user input.

use {
    regex::Regex,
    serde::{Deserialize, Serialize},
};

/// An endpoints required data that can be validated.
pub trait Post {
    type Invalid: Serialize + for<'a> Deserialize<'a>;
    fn validate(&self) -> Result<(), Self::Invalid>;
}

/// Settings for validating `&str`s.
pub struct Validation {
    pub max_length: Option<usize>,
    pub min_length: usize,
    pub must_be_ascii: bool,
    pub must_be_email: bool,
}

impl Default for Validation {
    fn default() -> Self {
        Self {
            max_length: Some(40),
            min_length: 1,
            must_be_ascii: false,
            must_be_email: false,
        }
    }
}

impl Validation {
    pub const fn minimal() -> Self {
        Self {
            max_length: None,
            min_length: 0,
            must_be_ascii: false,
            must_be_email: false,
        }
    }
    /// Validates a string.
    pub fn of(&self, s: &str) -> Result<(), Fail> {
        {
            let length = s.len();

            if length < self.min_length {
                return Err(TooShort);
            };

            if let Some(max) = self.max_length {
                if length > max {
                    return Err(TooLong);
                }
            }
        }

        if self.must_be_ascii && !s.is_ascii() {
            return Err(NotAscii);
        }

        if self.must_be_email
            && !Regex::new("(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$)")
                .unwrap()
                .is_match(s)
        {
            return Err(InvalidEmail);
        }
        Ok(())
    }
}

/// The reason a validation failed.
#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Fail {
    TooLong,
    TooShort,
    NotAscii,
    InvalidEmail,
}
use Fail::*;

impl Fail {
    pub fn show(self, field: &str) -> String {
        format!(
            "{} {}.",
            field,
            match self {
                TooLong => "must be shorter",
                TooShort => "must be longer",
                NotAscii => "must only contain ASCII characters",
                InvalidEmail => "must be a valid email",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Fail::*, Validation};
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
        assert_eq!(v.of("\u{03c4} > \u{03c0}"), Err(NotAscii));
        assert_eq!(v.of("tau > pi"), Ok(()));
    }

    #[test]
    fn email() {
        let v = Validation {
            must_be_email: true,
            ..Validation::default()
        };

        assert_eq!(v.of("example@example.com"), Ok(()));

        assert_eq!(v.of("@example.com"), Err(InvalidEmail));
        assert_eq!(v.of("example@.com"), Err(InvalidEmail));
        assert_eq!(v.of("@example"), Err(InvalidEmail));
    }
}
