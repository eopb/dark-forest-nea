use crate::State;
use std::{thread, time};

use {
    async_trait::async_trait,
    tide::{Body, Redirect, Request, Response, Server},
};
#[async_trait]
pub trait Endpoint: glue::Endpoint + 'static {
    async fn endpoint(req: Request<State>) -> tide::Result<Response>;
    fn apply(app: &mut Server<State>) {
        app.at(Self::PATH).get(Self::endpoint);
        app.at(Self::PATH).post(Self::endpoint);
    }
}

#[async_trait]
impl Endpoint for glue::Hello {
    async fn endpoint(_: Request<State>) -> tide::Result<Response> {
        thread::sleep(time::Duration::from_secs(1)); // Simulate slow response time.
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&Self {
            msg: String::from("Hi peeps"),
        })?);
        Ok(res)
    }
}

#[async_trait]
impl Endpoint for glue::Credentials {
    async fn endpoint(mut req: Request<State>) -> tide::Result<Response> {
        let credentials: glue::Credentials = req.body_form().await?;
        dbg!(credentials);

        Ok(Redirect::<&str>::new(glue::Route::Index.into()).into())
    }
}

#[async_trait]
impl Endpoint for glue::CreateAccount {
    async fn endpoint(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: glue::CreateAccount = req.body_form().await?;
        dbg!(account_info);

        Ok(Redirect::<&str>::new(glue::Route::Index.into()).into())
    }
}
