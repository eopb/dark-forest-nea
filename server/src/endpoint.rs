mod create_account;
mod sign_in;
mod sign_out;
mod signed_in;

use crate::State;
use std::{thread, time};

use {
    async_trait::async_trait,
    tide::{Body, Request, Response, Server},
};

#[async_trait]
pub trait Post: shared::Endpoint + 'static {
    async fn post(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).post(Self::post);
    }
}
#[async_trait]
pub trait Get: shared::Endpoint + 'static {
    async fn get(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).get(Self::get);
    }
}

#[async_trait]
impl Get for shared::Hello {
    async fn get(_: Request<State>) -> tide::Result<Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&Self {
            msg: String::from("Hi peeps"),
        })?);
        Ok(res)
    }
}
