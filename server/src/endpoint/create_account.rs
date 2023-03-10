use std::convert::TryInto;

use {async_trait::async_trait, tide::Request};

use crate::{
    endpoint::{self},
    state::{database::Insert, State},
};

use shared::{
    endpoint::create_account::{self, CreateAccount, Fail},
    validation::Post as _,
};

#[async_trait]
impl endpoint::Post for CreateAccount {
    #[tracing::instrument(err, skip(req))]
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
        })
    }
}
