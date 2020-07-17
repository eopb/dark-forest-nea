use crate::{
    data::Project, security::Authenticated, validation, Endpoint, PostEndpoint, Validation,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StartEditor;

impl Endpoint for StartEditor {
    type Response = Result<Project, Fail>;
    const PATH: &'static str = "/start-editor";
}

impl PostEndpoint for StartEditor {
    type Requires = Authenticated<ProjectPath>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPath {
    pub user_name: String,
    pub project_name: String,
}

/// Reasons creating a new project may fail.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Fail {
    DoesNotExist,
    PermissionDenied,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Invalid {
    pub project_name: Option<validation::Fail>,
}
