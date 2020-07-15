use {
    ::cookie::Cookie,
    async_trait::async_trait,
    shared::data::{
        self,
        sign_in::{
            self,
            Fail::{IncorrectPassword, UserNotFound},
        },
        ResponseKind,
    },
    tide::{Redirect, Request, Response},
    time::Duration,
};

use crate::{
    cookie,
    endpoint::{self, Endpoint},
    security,
    state::State,
};

#[async_trait]
impl endpoint::Post for shared::SignIn {
    async fn post(
        req: Request<State>,
        credentials: sign_in::Credentials,
    ) -> tide::Result<Result<data::security::Token, sign_in::Fail>> {
        let stored_user = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        Ok({
            if let Some(stored_user) = stored_user {
                if stored_user.verify_credentials(&credentials)? {
                    Ok(security::jwt::Claims::new(credentials.user_name).get_token()?)
                } else {
                    Err(sign_in::Fail::IncorrectPassword)
                }
            } else {
                Err(sign_in::Fail::UserNotFound)
            }
        })
    }
}
