use {
    ::cookie::Cookie,
    async_trait::async_trait,
    shared::data::sign_in::Fail::{IncorrectPassword, UserNotFound},
    tide::{Redirect, Request, Response},
    time::Duration,
};

use crate::{
    cookie,
    endpoint::{self, Endpoint},
    security,
    state::State,
};

use shared::data::{sign_in, ResponseKind};

#[async_trait]
impl endpoint::Post for shared::SignIn {
    async fn post(
        req: Request<State>,
        credentials: <Self as shared::PostEndpoint>::Requires,
    ) -> tide::Result<<Self as shared::Endpoint>::Response> {
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

#[allow(clippy::module_name_repetitions)]
pub fn unsafe_sign_in(mut res: Response, user: String) -> tide::Result<Response> {
    let claims = security::jwt::Claims::new(user);
    res.insert_cookie(
        Cookie::build(cookie::LOGIN, claims.get_token()?)
            .max_age(Duration::minutes(security::jwt::Claims::max_age_minutes()))
            .secure(true)
            .http_only(true)
            .finish(),
    );
    Ok(res)
}
