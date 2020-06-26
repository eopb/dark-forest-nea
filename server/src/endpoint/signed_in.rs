use {
    async_trait::async_trait,
    jsonwebtoken::{decode, DecodingKey, Validation},
    tide::{Body, Request, Response},
};

use std::env;

use crate::{endpoint, security, state::State};

#[async_trait]
impl endpoint::Get for glue::SingedIn {
    async fn get(req: Request<State>) -> tide::Result<Response> {
        let user = req.cookie("login").and_then(|cookie| {
            decode::<security::jwt::Claims>(
                cookie.value(),
                &DecodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
                &Validation::default(),
            )
            .ok()
            .map(|token| token.claims.sub)
        });

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&match user {
            Some(user) => Self::As(user),
            None => Self::Not,
        })?);
        Ok(res)
    }
}
