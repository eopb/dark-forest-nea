use {
    async_trait::async_trait,
    tide::{Body, Request, Response},
};

use crate::{endpoint, security::jwt, state::State};

#[async_trait]
impl endpoint::Get for shared::SignedIn {
    async fn get(req: Request<State>) -> tide::Result<Response> {
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&Self::get_user(&req).await)?);
        Ok(res)
    }
}

#[async_trait]
pub trait Ext {
    async fn get_user(req: &Request<State>) -> Self;
}

#[async_trait]
impl Ext for shared::SignedIn {
    async fn get_user(req: &Request<State>) -> Self {
        let user = req.cookie("login").and_then(|cookie| {
            jwt::Claims::decode_token(cookie.value())
                .map(|token| token.claims.sub)
                .ok()
        });

        user.map_or(Self::Not, Self::As)
    }
}
