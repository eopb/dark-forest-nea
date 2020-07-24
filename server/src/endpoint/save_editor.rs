use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self, signed_in::Ext},
    state::{database::project_store::ProjectStore, State},
};

use shared::{
    data::Project,
    endpoint::{
        edit::{
            save::{PermissionDenied, SaveEditor},
            ProjectPath,
        },
        signed_in::{self, SignedIn},
    },
    security::Authenticated,
};

#[async_trait]
impl endpoint::Post for SaveEditor {
    async fn post(
        req: Request<State>,
        req_body: Authenticated<(ProjectPath, Project)>,
    ) -> tide::Result<Result<(), PermissionDenied>> {
        Ok({
            let user = SignedIn::get_user(&req_body.token).await;
            let (path, project) = req_body.inner;
            if let signed_in::Res::As(user) = user {
                if user == path.user_name {
                    let uuid = req
                        .state()
                        .database()
                        .get_project_uuid(&path.user_name, &path.project_name)
                        .await?;
                    if let Some(uuid) = uuid {
                        req.state()
                            .database()
                            .save_project(ProjectStore::new(uuid, project))
                            .await?;
                        return Ok(Ok(()));
                    }
                }
            }
            Err(PermissionDenied)
        })
    }
}
