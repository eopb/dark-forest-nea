use crate::{
    data::{security::Authenticated, validation, Validation},
    Endpoint, PostEndpoint,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NewProject;

/// Data sent when a user creates an account.
#[derive(Clone, Default, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Details {
    pub project_name: String,
}

impl Endpoint for NewProject {
    type Response = Result<(), Fail>;
    const PATH: &'static str = "/new-project";
}

impl PostEndpoint for NewProject {
    type Requires = Authenticated<Details>;
}

impl validation::Post for Details {
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

/// Reasons creating a new project may fail.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    AlreadyExists,
    InvalidField(Invalid),
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Invalid {
    pub project_name: Option<validation::Fail>,
}
