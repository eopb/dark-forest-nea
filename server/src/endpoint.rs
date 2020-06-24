mod create_account;

use crate::State;
use std::{thread, time};

use {
    async_trait::async_trait,
    tide::{Body, Redirect, Request, Response, Server},
};

#[async_trait]
pub trait PostEndpoint: glue::Endpoint + 'static {
    async fn post(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).post(Self::post);
    }
}
#[async_trait]
pub trait GetEndpoint: glue::Endpoint + 'static {
    async fn get(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).get(Self::get);
    }
}

#[async_trait]
impl GetEndpoint for glue::Hello {
    async fn get(_: Request<State>) -> tide::Result<Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&Self {
            msg: String::from("Hi peeps"),
        })?);
        Ok(res)
    }
}

#[async_trait]
impl PostEndpoint for glue::Credentials {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let credentials: glue::Credentials = req.body_form().await?;
        dbg!(credentials);

        Ok(Redirect::<&str>::new(glue::Route::Index.into()).into())
    }
}
