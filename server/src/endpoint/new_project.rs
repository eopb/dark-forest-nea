use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self, signed_in::Ext},
    state::{database::Insert, State},
};

use shared::{
    data::validation::Post as _,
    data::{
        new_project::{self, Fail},
        security::Authenticated,
        signed_in,
    },
};

#[async_trait]
impl endpoint::Post for shared::NewProject {
    async fn post(
        req: Request<State>,
        Authenticated {
            inner: new_project,
            token,
        }: Authenticated<new_project::Details>,
    ) -> tide::Result<Result<(), new_project::Fail>> {
        let user = shared::SignedIn::get_user(&token).await;

        let validation = new_project.validate();

        Ok(if let Err(error) = validation {
            Err(Fail::InvalidField(error))
        } else if let signed_in::Res::As(user) = user {
            match req
                .state()
                .database()
                .add_project(user.clone(), new_project.project_name.clone())
                .await?
            {
                Insert::Success => Ok(()),

                Insert::AlreadyExists => Err(Fail::AlreadyExists),
            }
        } else {
            //TODO handel the not singed in case.
            Ok(())
        })
    }
}
