//! Application endpoints.

use super::State;

use {
    http_types::mime,
    tide::{Request, Response, StatusCode},
};

pub async fn index(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok({
        let mut res = Response::new(StatusCode::Ok);
        res.set_content_type(mime::HTML);
        res.set_body(include_str!("../../client/index.html"));
        res
    })
}
