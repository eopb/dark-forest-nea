//! API endpoints

mod create_account;
mod new_project;
mod refresh_token;
pub mod sign_in;
mod signed_in;

use crate::State;

use std::{future::Future, thread, time};

use {
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    shared::data::{
        DataKind::{self, Binary, Json},
        DataKinds,
    },
    tide::{Body, Request, Response, Server},
};

/// API endpoint that uses the `post` HTTP verb.
#[async_trait]
pub trait Post: shared::PostEndpoint {
    /// Invoked on `post`.
    async fn post(
        req: Request<State>,
        data: <Self as shared::PostEndpoint>::Requires,
    ) -> tide::Result<<Self as shared::Endpoint>::Response>;
    /// Add an endpoint as a `post` to a server.
    fn apply(app: &mut Server<State>) {
        for res_kind in DataKinds::possible() {
            app.at(&Self::path(*res_kind))
                .post(move |req| post_response(req, *res_kind, Self::post));
        }
    }
}

/// API endpoint that uses the `get` HTTP verb.
#[async_trait]
pub trait Get: shared::Endpoint {
    /// Invoked on `get`.
    async fn get(req: Request<State>) -> tide::Result<<Self as shared::Endpoint>::Response>;
    /// Add an endpoint as a `get` to a server.
    fn apply(app: &mut Server<State>) {
        for res_kind in DataKinds::possible() {
            app.at(&Self::path(*res_kind))
                .get(move |req| get_response(req, *res_kind, Self::get));
        }
    }
}

async fn get_response<Func, Output, Fut>(
    req: Request<State>,
    res_kind: DataKinds,
    endpoint: Func,
) -> tide::Result<Response>
where
    Func: Send + Sync + 'static + Fn(Request<State>) -> Fut,
    Fut: Future<Output = tide::Result<Output>> + Send + 'static,
    Output: Serialize,
{
    let value = endpoint(req).await?;

    response(value, res_kind)
}
async fn post_response<Func, Output, Fut, Data>(
    mut req: Request<State>,
    res_kind: DataKinds,
    endpoint: Func,
) -> tide::Result<Response>
where
    Func: Send + Sync + 'static + Fn(Request<State>, Data) -> Fut,
    Fut: Future<Output = tide::Result<Output>> + Send + 'static,
    Output: Serialize,
    Data: for<'a> Deserialize<'a> + Send + Sync,
{
    let data = match res_kind.server_requires {
        Binary => bincode::deserialize(&req.body_bytes().await?)?,
        Json => req.body_json().await?,
    };
    let value = endpoint(req, data).await?;

    response(value, res_kind)
}

fn response(value: impl Serialize, res_kind: DataKinds) -> tide::Result<Response> {
    let mut res = Response::new(200);

    res.set_body(match res_kind.server_response {
        Binary => Body::from_bytes(bincode::serialize(&value)?),
        Json => Body::from_json(&value)?,
    });
    Ok(res)
}
#[async_trait]
impl Get for shared::Hello {
    async fn get(_: Request<State>) -> tide::Result<<Self as shared::Endpoint>::Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        Ok(shared::data::hello::Res {
            msg: String::from("Hi peeps"),
        })
    }
}
