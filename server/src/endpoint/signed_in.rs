use {
    async_trait::async_trait,
    tide::{Request, Response},
};

use crate::{
    cookie,
    endpoint::{self, Endpoint},
    security::jwt,
    state::State,
};

use shared::data::ResponseKind;

impl Endpoint for shared::SignedIn {}

#[async_trait]
impl endpoint::Get for shared::SignedIn {
    async fn get(req: Request<State>, res_kind: ResponseKind) -> tide::Result<Response> {
        let mut res = Response::new(200);
        res.set_body(Self::body_from(&Self::get_user(&req).await, res_kind)?);
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
        let user = req.cookie(cookie::LOGIN).and_then(|cookie| {
            jwt::Claims::decode_token(cookie.value())
                .map(|token| token.claims.sub)
                .ok()
        });

        user.map_or(Self::Not, Self::As)
    }
}
