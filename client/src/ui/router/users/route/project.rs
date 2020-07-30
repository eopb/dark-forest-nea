pub mod route;

use crate::{state, updates};

use seed::prelude::*;

use shared::routes::Project;

pub fn view(model: &state::Model, user_name: &str, project: &Project) -> Node<updates::Msg> {
    let Project { project_name, nest } = project;
    match nest {
        Some(project_route) => route::view(model, user_name, project_name, project_route),
        None => todo!(),
    }
}
