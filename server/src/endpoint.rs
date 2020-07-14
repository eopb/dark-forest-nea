//! API endpoints

mod create_account;
mod new_project;
mod refresh_token;
pub mod sign_in;
mod sign_out;
mod signed_in;

use crate::State;

use std::{thread, time};

use {
    async_trait::async_trait,
    serde::Serialize,
    shared::data::ResponseKind::{self, Binary, Json},
    tide::{Body, Request, Response, Server},
};

/// Server extension to API endpoints.
pub trait Endpoint: shared::Endpoint + 'static {
    fn body_from(value: Self::Response, res_kind: ResponseKind) -> tide::Result<Body> {
        Self::_body_from(value, res_kind)
    }
    /// Create a HTTP response body with a given kind from a type.
    ///
    /// TODO remove this function as it is less typed.
    fn _body_from<T: Serialize>(value: T, res_kind: ResponseKind) -> tide::Result<Body> {
        Ok(match res_kind {
            Binary => Body::from_bytes(bincode::serialize(&value)?),
            Json => Body::from_json(&value)?,
        })
    }
}

/// API endpoint that uses the `post` HTTP verb.
#[async_trait]
pub trait Post: Endpoint {
    /// Invoked on `post`.
    async fn post(req: Request<State>, res_kind: ResponseKind) -> tide::Result<Response>;
    /// Add an endpoint as a `post` to a server.
    fn apply(app: &mut Server<State>) {
        for res_kind in &[Json, Binary] {
            app.at(&Self::path(*res_kind))
                .post(move |req| Self::post(req, *res_kind));
        }
    }
}

/// API endpoint that uses the `get` HTTP verb.
#[async_trait]
pub trait Get: Endpoint {
    /// Invoked on `get`.
    async fn get(req: Request<State>, res_kind: ResponseKind) -> tide::Result<Response>;
    /// Add an endpoint as a `get` to a server.
    fn apply(app: &mut Server<State>) {
        for res_kind in &[Json, Binary] {
            app.at(&Self::path(*res_kind))
                .get(move |req| Self::get(req, *res_kind));
        }
    }
}

impl Endpoint for shared::Hello {}

#[async_trait]
impl Get for shared::Hello {
    async fn get(_: Request<State>, res_kind: ResponseKind) -> tide::Result<Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        let mut res = Response::new(200);
        res.set_body(Self::_body_from(
            &Self {
                msg: String::from("Hi peeps"),
            },
            res_kind,
        )?);
        Ok(res)
    }
}
