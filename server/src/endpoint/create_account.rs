use std::convert::TryInto;

use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    state::{database::Insert, State},
    PostEndpoint,
};

#[async_trait]
impl PostEndpoint for glue::CreateAccount {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: glue::CreateAccount = req.body_form().await?;

        Ok(
            match req
                .state()
                .database()
                .add_user(account_info.try_into()?)
                .await?
            {
                Insert::Success => Redirect::<&str>::new("/api/sign-in").into(),
                Insert::AlreadyExists => {
                    Redirect::<&str>::new(glue::Route::CreateAccount.into()).into()
                }
            },
        )
    }
}
