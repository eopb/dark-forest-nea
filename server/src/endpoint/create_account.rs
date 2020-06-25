use std::convert::TryInto;

use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint,
    state::{database::Insert, State},
};

#[async_trait]
impl endpoint::Post for glue::CreateAccount {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: glue::CreateAccount = req.body_form().await?;

        Ok(Redirect::<&str>::new(
            match req
                .state()
                .database()
                .add_user(account_info.try_into()?)
                .await?
            {
                Insert::Success => "/api/sign-in",
                Insert::AlreadyExists => glue::Route::CreateAccount.into(),
            },
        )
        .into())
    }
}
