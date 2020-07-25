pub mod editor;

use crate::{state, updates};

use seed::prelude::*;

use shared::{endpoint::edit::ProjectPath, routes::ProjectRoute};

pub fn view(
    model: &state::Model,
    user_name: &str,
    project_name: &str,
    project_route: &ProjectRoute,
) -> Node<updates::Msg> {
    match project_route {
        ProjectRoute::Edit => editor::view(model, ProjectPath {
            user_name: user_name.to_owned(),
            project_name: project_name.to_owned(),
        }),
    }
}
