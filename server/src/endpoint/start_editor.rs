use {async_trait::async_trait, tide::Request};

use crate::{endpoint, state::State};

use shared::{
    data::Project,
    endpoint::edit::{
        init::{Fail, StartEditor},
        ProjectPath,
    },
    security::Authenticated,
};

#[async_trait]
impl endpoint::Post for StartEditor {
    async fn post(
        _: Request<State>,
        _: Authenticated<ProjectPath>,
    ) -> tide::Result<Result<Project, Fail>> {
        Ok(Ok(Project::example()))
    }
}
