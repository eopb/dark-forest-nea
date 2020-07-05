use {
    async_trait::async_trait,
    cookie::Cookie,
    shared::data::credentials::Fail::{IncorrectPassword, UserNotFound},
    tide::{Redirect, Request, Response},
    time::Duration,
};

use crate::{
    endpoint::{self, Endpoint},
    security,
    state::State,
};

use shared::data::ResponseKind;

impl Endpoint for shared::Credentials {}

#[async_trait]
impl endpoint::Post for shared::Credentials {
    async fn post(mut req: Request<State>, _: ResponseKind) -> tide::Result<Response> {
        let credentials: Self = req.body_form().await?;

        let stored_user = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        Ok(if let Some(stored_user) = stored_user {
            if stored_user.verify_credentials(&credentials)? {
                unsafe_sign_in(
                    Redirect::new(shared::Route::Index.to_string()).into(),
                    credentials.user_name,
                )?
            } else {
                Redirect::new(shared::Route::SignIn(Some(IncorrectPassword)).to_string()).into()
            }
        } else {
            Redirect::new(shared::Route::SignIn(Some(UserNotFound)).to_string()).into()
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn unsafe_sign_in(mut res: Response, user: String) -> tide::Result<Response> {
    let claims = security::jwt::Claims::new(user);
    res.insert_cookie(
        Cookie::build("login", claims.get_token()?)
            .max_age(Duration::minutes(security::jwt::Claims::max_age_minutes()))
            .secure(true)
            .http_only(true)
            .finish(),
    );
    Ok(res)
}
