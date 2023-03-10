use {
    async_trait::async_trait,
    tide::Request,
    tracing::{info, instrument},
};

use crate::{endpoint, security::jwt, state::State};

use shared::{
    endpoint::signed_in::{self, SignedIn},
    security::Token,
};

#[async_trait]
impl endpoint::Post for SignedIn {
    #[instrument(err)]
    async fn post(_: Request<State>, token: Token) -> tide::Result<signed_in::Res> {
        info!("Checking if user is singed in.");
        Ok(Self::get_user(&token).await)
    }
}

/// Server functionality extension for `SignedIn`
#[async_trait]
pub trait Ext: shared::Endpoint {
    #[allow(clippy::ptr_arg)]
    async fn get_user(token: &Token) -> signed_in::Res;
}

#[async_trait]
impl Ext for SignedIn {
    #[instrument(level = "trace")]
    async fn get_user(token: &Token) -> signed_in::Res {
        let user = jwt::Claims::decode_token(token)
            .map(|token| token.claims.sub)
            .ok();

        user.map_or(signed_in::Res::Not, signed_in::Res::As)
    }
}
