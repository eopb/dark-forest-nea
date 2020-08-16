pub mod editor;
pub mod player;

use crate::{state, updates};

use seed::prelude::*;

use shared::{endpoint::edit::ProjectPath, routes::ProjectRoute};

pub fn view(
    model: &state::Model,
    user_name: &str,
    project_name: &str,
    project_route: &ProjectRoute,
) -> Node<updates::Msg> {
    let project_path = ProjectPath {
        user_name: user_name.to_owned(),
        project_name: project_name.to_owned(),
    };
    match project_route {
        ProjectRoute::Edit => editor::view(model, project_path),
        ProjectRoute::Play => player::view(model, project_path),
    }
}
