pub mod create_account;
pub mod editor;
pub mod index;
pub mod new_project;
pub mod not_found;
pub mod sign_in;

use crate::{state, updates};

use seed::{prelude::*, *};

use shared::{
    endpoint::edit::ProjectPath,
    routes::{Project, ProjectRoute, UserRoute},
    Route,
};

/// Main router view showing items unique to a route.
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match model.route.0 {
        Some(ref route) => match route {
            Route::Index => index::view(model),
            Route::Explore => p!["explore"],
            Route::SignIn => sign_in::view(model),
            Route::CreateAccount => create_account::view(model),
            Route::NewProject => new_project::view(model),
            Route::Users {
                user_name,
                nest:
                    Some(UserRoute::Projects(Some(Project {
                        project_name,
                        nest: Some(ProjectRoute::Edit),
                    }))),
            } => editor::view(model, ProjectPath {
                user_name: user_name.to_owned(),
                project_name: project_name.to_owned(),
            }),
            Route::Users { .. } => todo!(),
        },
        None => not_found::view(model),
    }
}
