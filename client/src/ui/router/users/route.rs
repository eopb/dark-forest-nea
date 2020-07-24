pub mod project;

use crate::{state, updates};

use seed::prelude::*;

use shared::routes::UserRoute;

pub fn view(model: &state::Model, user_name: &str, user_route: &UserRoute) -> Node<updates::Msg> {
    match user_route {
        UserRoute::Projects(Some(project)) => project::view(model, user_name, project),
        UserRoute::Projects(None) => todo!(),
    }
}
