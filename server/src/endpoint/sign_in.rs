use {
    async_trait::async_trait,
    cookie::Cookie,
    shared::data::credentials::Fail::{IncorrectPassword, UserNotFound},
    tide::{Redirect, Request, Response},
    time::Duration,
};

use crate::{endpoint, security, state::State};

#[async_trait]
impl endpoint::Post for shared::Credentials {
    async fn post(mut req: Request<State>) -> tide::Result<Response> {
        let credentials: Self = req.body_form().await?;

        let stored_user = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        Ok(if let Some(stored_user) = stored_user {
            if stored_user.verify_credentials(&credentials)? {
                let mut response: Response = Redirect::new(shared::Route::Index.to_string()).into();
                response.insert_cookie(
                    Cookie::build(
                        "login",
                        security::jwt::Claims::new(credentials.user_name).get_token()?,
                    )
                    .max_age(Duration::days(security::jwt::Claims::max_age_days()))
                    .secure(true)
                    .http_only(true)
                    .finish(),
                );
                response
            } else {
                Redirect::new(shared::Route::SignIn(Some(IncorrectPassword)).to_string()).into()
            }
        } else {
            Redirect::new(shared::Route::SignIn(Some(UserNotFound)).to_string()).into()
        })
    }
}
