use {
    async_trait::async_trait,
    tide::{Body, Request, Response},
};

use crate::{endpoint, security::jwt, state::State};

#[async_trait]
impl endpoint::Get for shared::SignedIn {
    async fn get(req: Request<State>) -> tide::Result<Response> {
        let user = req.cookie("login").and_then(|cookie| {
            jwt::Claims::decode_token(cookie.value())
                .map(|token| token.claims.sub)
                .ok()
        });

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&match user {
            Some(user) => Self::As(user),
            None => Self::Not,
        })?);
        Ok(res)
    }
}
