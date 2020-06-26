use std::convert::TryInto;

use {
    async_trait::async_trait,
    glue::data::credentials::Fail::*,
    tide::{Redirect, Request, Response},
};

use crate::{
    endpoint,
    state::{self, database::Insert, State},
};

#[async_trait]
impl endpoint::Post for glue::Credentials {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let credentials: Self = req.body_form().await?;

        let stored_user: Option<state::database::User> = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        let result = if let Some(stored_user) = stored_user {
            if stored_user.verify(&credentials)? {
                Success
            } else {
                IncorrectPassword
            }
        } else {
            UserNotFound
        };

        Ok(Redirect::<String>::new(match result {
            Success => glue::Route::Index.to_string(),
            IncorrectPassword | UserNotFound => glue::Route::SignIn(Some(result)).to_string(),
        })
        .into())
    }
}
