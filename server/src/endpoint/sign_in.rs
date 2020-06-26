use {
    async_trait::async_trait,
    glue::data::credentials::Fail::{IncorrectPassword, UserNotFound},
    tide::{Redirect, Request, Response},
};

use crate::{endpoint, state::State};

#[async_trait]
impl endpoint::Post for glue::Credentials {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let credentials: Self = req.body_form().await?;

        let stored_user = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        Ok(Redirect::new(
            if let Some(stored_user) = stored_user {
                if stored_user.verify(&credentials)? {
                    glue::Route::Index
                } else {
                    glue::Route::SignIn(Some(IncorrectPassword))
                }
            } else {
                glue::Route::SignIn(Some(UserNotFound))
            }
            .to_string(),
        )
        .into())
    }
}
