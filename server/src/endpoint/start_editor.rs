use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self, signed_in::Ext},
    security,
    state::State,
};

use shared::{
    data::Project,
    endpoint::{
        edit::init::StartEditor,
        signed_in::{self, SignedIn},
    },
    security::Token,
};

#[async_trait]
impl endpoint::Get for StartEditor {
    async fn get(_: Request<State>) -> tide::Result<<Self as shared::Endpoint>::Response> {
        Ok(Ok(Project::example()))
    }
}
