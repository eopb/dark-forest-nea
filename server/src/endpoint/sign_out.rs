use {
    async_trait::async_trait,
    cookie::Cookie,
    tide::{Redirect, Request, Response},
};

use crate::{endpoint, state::State};

#[async_trait]
impl endpoint::Get for glue::SignOut {
    async fn get(_: Request<State>) -> tide::Result<Response> {
        let mut res: Response = Redirect::new(glue::Route::CreateAccount.to_string()).into();
        res.remove_cookie(Cookie::named("login"));
        Ok(res)
    }
}
