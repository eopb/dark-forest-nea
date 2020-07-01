use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint::{self, signed_in::Ext},
    state::{database::Insert, State},
};

use shared::{data::new_project::Fail, data::validation::Post as _};

#[async_trait]
impl endpoint::Post for shared::NewProject {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let new_project: Self = req.body_form().await?;

        let user = shared::SignedIn::get_user(&req).await;

        let validation = new_project.validate();

        Ok(Redirect::new(
            if let Err(error) = validation {
                shared::Route::NewProject(Some(Fail::InvalidField(error)))
            } else if let shared::SignedIn::As(user) = user {
                match req
                    .state()
                    .database()
                    .add_project(user.clone(), new_project.project_name.clone())
                    .await?
                {
                    Insert::Success => shared::Route::Users {
                        user_name: user,
                        nest: Some(shared::routes::UserRoute::Projects(Some(
                            shared::routes::Project {
                                project_name: new_project.project_name,
                                nest: Some(shared::routes::ProjectRoute::Edit),
                            },
                        ))),
                    },
                    Insert::AlreadyExists => shared::Route::NewProject(Some(Fail::AlreadyExists)),
                }
            } else {
                shared::Route::SignIn(None)
            }
            .to_string(),
        )
        .into())
    }
}
