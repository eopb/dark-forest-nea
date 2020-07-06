use {
    ::cookie::Cookie,
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    cookie,
    endpoint::{self, Endpoint},
    state::State,
};

use shared::data::ResponseKind;

impl Endpoint for shared::SignOut {}

#[async_trait]
impl endpoint::Get for shared::SignOut {
    async fn get(_: Request<State>, _: ResponseKind) -> tide::Result<Response> {
        let mut res: Response = Redirect::new(shared::Route::Index.to_string()).into();
        res.remove_cookie(Cookie::named(cookie::LOGIN));
        Ok(res)
    }
}
