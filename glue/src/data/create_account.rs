use crate::{
    data::{validation, Validation},
    Endpoint,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateAccount {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    AlreadyExists,
    InvalidField(Invalid),
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Invalid {
    pub user_name: Option<validation::Error>,
    pub email: Option<validation::Error>,
    pub password: Option<validation::Error>,
}

impl Endpoint for CreateAccount {
    const PATH: &'static str = "/api/create-account";
}

impl validation::Post for CreateAccount {
    type Invalid = Invalid;
    fn validate(&self) -> Result<(), Self::Invalid> {
        let validation = Validation {
            must_be_ascii: true,
            ..Validation::default()
        };
        let user_name = validation.of(&self.user_name).err();
        let email = validation.of(&self.email).err();
        let password = Validation {
            min_length: 1,
            max_length: Some(1_000),
            ..Validation::minimal()
        }
        .of(&self.password)
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
