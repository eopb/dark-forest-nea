use {async_trait::async_trait, tide::Request};

use crate::{endpoint, security::jwt, state::State};

use shared::data::{security::Token, signed_in};

#[async_trait]
impl endpoint::Post for shared::SignedIn {
    async fn post(_: Request<State>, token: Token) -> tide::Result<signed_in::Res> {
        Ok(Self::get_user(&token).await)
    }
}

/// Server functionality extension for `SignedIn`
#[async_trait]
pub trait Ext: shared::Endpoint {
    async fn get_user(token: &Token) -> signed_in::Res;
}

#[async_trait]
impl Ext for shared::SignedIn {
    async fn get_user(token: &Token) -> signed_in::Res {
        let user = jwt::Claims::decode_token(token)
            .map(|token| token.claims.sub)
            .ok();

        user.map_or(signed_in::Res::Not, signed_in::Res::As)
    }
}
