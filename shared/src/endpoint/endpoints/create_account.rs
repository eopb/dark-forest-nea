use crate::{security::serialize_secret, validation, Endpoint, PostEndpoint, Validation};

use {
    secrecy::{ExposeSecret, Secret, SecretString},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CreateAccount;

impl Endpoint for CreateAccount {
    type Response = Result<(), Fail>;

    const PATH: &'static str = "/create-account";
}

impl PostEndpoint for CreateAccount {
    type Requires = Details;
}

/// Data sent when a user creates an account.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Details {
    pub user_name: String,
    pub email: String,
    #[serde(serialize_with = "serialize_secret")]
    pub password: SecretString,
}

impl Default for Details {
    fn default() -> Self {
        Self {
            user_name: String::new(),
            email: String::new(),
            password: Secret::new(String::new()),
        }
    }
}

impl validation::Post for Details {
    type Invalid = Invalid;
    fn validate(&self) -> Result<(), Self::Invalid> {
        let user_name = Validation {
            must_be_ascii: true,
            ..Validation::default()
        }
        .of(&self.user_name)
        .err();

        let email = Validation {
            must_be_ascii: true,
            must_be_email: true,
            ..Validation::default()
        }
        .of(&self.email)
        .err();

        let password = Validation {
            min_length: 1,
            max_length: Some(1_000),
            ..Validation::minimal()
        }
        .of(self.password.expose_secret())
        .err();

        if user_name.is_none() && email.is_none() && password.is_none() {
            Ok(())
        } else {
            Err(Invalid {
                user_name,
                email,
                password,
            })
        }
    }
}

/// Reasons creating an account may fail.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    AlreadyExists,
    InvalidField(Invalid),
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Invalid {
    pub user_name: Option<validation::Fail>,
    pub email: Option<validation::Fail>,
    pub password: Option<validation::Fail>,
}

#[cfg(test)]
impl Details {
    pub fn mock() -> Self {
        Self {
            user_name: "Ethan".to_owned(),
            email: "example@example.com".to_owned(),
            password: Secret::new("hunter2".to_owned()),
        }
    }
}
