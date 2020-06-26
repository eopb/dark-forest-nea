use std::convert::TryInto;

use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint,
    state::{database::Insert, State},
};

use glue::Endpoint as _;

#[async_trait]
impl endpoint::Post for glue::CreateAccount {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: Self = req.body_form().await?;

        Ok(Redirect::<String>::new(
            match req
                .state()
                .database()
                .add_user(account_info.try_into()?)
                .await?
            {
                Insert::Success => glue::Credentials::PATH.to_string(),
                Insert::AlreadyExists => glue::Route::CreateAccount.to_string(),
            },
        )
        .into())
    }
}
