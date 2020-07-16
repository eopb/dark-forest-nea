use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self, signed_in::Ext},
    security,
    state::State,
};

use shared::{
    endpoint::{
        refresh_token::RefreshToken,
        signed_in::{self, SignedIn},
    },
    security::Token,
};

#[async_trait]
impl endpoint::Post for RefreshToken {
    async fn post(_: Request<State>, token: Token) -> tide::Result<Option<Token>> {
        let user = SignedIn::get_user(&token).await;

        Ok(match user {
            signed_in::Res::As(user) => Some(security::jwt::Claims::new(user).get_token()?),
            signed_in::Res::Not => None,
        })
    }
}
