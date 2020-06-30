use std::convert::TryInto;

use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint,
    state::{database::Insert, State},
};

use shared::{data::create_account::Fail, data::validation::Post as _, Endpoint as _};

#[async_trait]
impl endpoint::Post for shared::CreateAccount {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let account_info: Self = req.body_form().await?;

        let validation = account_info.validate();

        Ok(if let Err(error) = validation {
            Redirect::new(shared::Route::CreateAccount(Some(Fail::InvalidField(error))).to_string())
                .into()
        } else {
            match req
                .state()
                .database()
                .add_user(account_info.try_into()?)
                .await?
            {
                Insert::Success => Redirect::temporary(shared::Credentials::PATH.to_owned()),
                Insert::AlreadyExists => Redirect::new(
                    shared::Route::CreateAccount(Some(Fail::AlreadyExists)).to_string(),
                ),
            }
            .into()
        })
    }
}
