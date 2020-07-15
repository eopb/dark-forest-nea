use {async_trait::async_trait, tide::Request};

use crate::{endpoint, security::jwt, state::State};

use shared::data::signed_in;

#[async_trait]
impl endpoint::Post for shared::SignedIn {
    async fn post(
        _: Request<State>,
        token: <Self as shared::PostEndpoint>::Requires,
    ) -> tide::Result<<Self as shared::Endpoint>::Response> {
        Ok(Self::get_user(&token).await)
    }
}

#[async_trait]
pub trait Ext: shared::Endpoint {
    async fn get_user(token: &str) -> <Self as shared::Endpoint>::Response;
}

#[async_trait]
impl Ext for shared::SignedIn {
    async fn get_user(token: &str) -> <Self as shared::Endpoint>::Response {
        let user = jwt::Claims::decode_token(token)
            .map(|token| token.claims.sub)
            .ok();

        user.map_or(signed_in::Res::Not, signed_in::Res::As)
    }
}
