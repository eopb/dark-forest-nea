//! Application endpoints.

use super::state::State;

use std::str::FromStr;

use {
    http_types::mime,
    tide::{Request, Response, StatusCode},
};

pub async fn index(req: Request<State>) -> tide::Result<impl Into<Response>> {
    println!("Hello");
    Ok({
        let mut res = Response::new(StatusCode::Ok);
        res.set_content_type(mime::HTML);
        res.set_body(include_str!("../../client/index.html"));
        res
    })
}

pub async fn hello(req: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok("hello client")
}
