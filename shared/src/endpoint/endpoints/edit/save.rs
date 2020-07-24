use crate::{data::Project, security::Authenticated, validation, Endpoint, PostEndpoint};

use super::ProjectPath;

use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SaveEditor;

impl Endpoint for SaveEditor {
    type Response = Result<(), PermissionDenied>;
    const PATH: &'static str = "/save-editor";
}

impl PostEndpoint for SaveEditor {
    type Requires = Authenticated<(ProjectPath, Project)>;
}

/// Reasons saving a project may fail.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PermissionDenied;
