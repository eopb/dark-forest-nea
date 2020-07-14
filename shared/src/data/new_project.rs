use crate::{
    data::{validation, Validation},
    Endpoint,
};

use serde::{Deserialize, Serialize};

/// Data sent when a user creates an account.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NewProject {
    pub project_name: String,
}

impl Endpoint for NewProject {
    type Response = ();
    const PATH: &'static str = "/new-project";
}

impl validation::Post for NewProject {
    type Invalid = Invalid;
    fn validate(&self) -> Result<(), Self::Invalid> {
        let project_name = Validation {
            must_be_ascii: true,
            ..Validation::default()
        }
        .of(&self.project_name)
        .err();

        if project_name.is_none() {
            Ok(())
        } else {
            Err(Invalid { project_name })
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
    pub project_name: Option<validation::Fail>,
}
