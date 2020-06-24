use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{PostEndpoint, State};

#[async_trait]
impl PostEndpoint for glue::CreateAccount {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: glue::CreateAccount = req.body_form().await?;
        dbg!(account_info);

        Ok(Redirect::<&str>::new(glue::Route::Index.into()).into())
    }
}
