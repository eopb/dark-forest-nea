use std::convert::TryInto;

use {
    async_trait::async_trait,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint::{self, Endpoint},
    state::{database::Insert, State},
};

use shared::{
    data::validation::Post as _,
    data::{
        create_account::{self, Fail},
        ResponseKind,
    },
    Endpoint as _,
};

#[async_trait]
impl endpoint::Post for shared::CreateAccount {
    async fn post(
        req: Request<State>,
        account_info: create_account::Details,
    ) -> tide::Result<Result<(), create_account::Fail>> {
        let validation = account_info.validate();

        Ok(if let Err(error) = validation {
            Err(Fail::InvalidField(error))
        } else {
            match req
                .state()
                .database()
                .add_user(account_info.try_into()?)
                .await?
            {
                Insert::Success => Ok(()),
                Insert::AlreadyExists => Err(Fail::AlreadyExists),
            }
            .into()
        })
    }
}
