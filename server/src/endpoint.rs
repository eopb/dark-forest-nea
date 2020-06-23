use crate::State;
use std::{thread, time};

use {
    async_trait::async_trait,
    tide::{Body, Request, Response, Server},
};
#[async_trait]
pub trait Endpoint: glue::Endpoint + 'static {
    async fn endpoint(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).get(Self::endpoint);
    }
}

#[async_trait]
impl Endpoint for glue::Hello {
    async fn endpoint(_req: Request<State>) -> tide::Result<Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&Self {
            msg: String::from("Hi peeps"),
        })?);
        Ok(res)
    }
}
