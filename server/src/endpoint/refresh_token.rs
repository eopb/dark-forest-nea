use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self, signed_in::Ext},
    security,
    state::State,
};

use shared::data::{security::Token, signed_in};

#[async_trait]
impl endpoint::Post for shared::RefreshToken {
    async fn post(_: Request<State>, token: Token) -> tide::Result<Option<Token>> {
        let user = shared::SignedIn::get_user(&token).await;

        Ok(match user {
            signed_in::Res::As(user) => Some(security::jwt::Claims::new(user).get_token()?),
            signed_in::Res::Not => None,
        })
    }
}
