use {
    ::cookie::Cookie,
    async_trait::async_trait,
    shared::data::credentials::Fail::{IncorrectPassword, UserNotFound},
    tide::{Redirect, Request, Response},
    time::Duration,
};

use crate::{
    cookie,
    endpoint::{self, Endpoint},
    security,
    state::State,
};

use shared::data::{credentials, ResponseKind};

impl Endpoint for shared::Credentials {}

#[async_trait]
impl endpoint::Post for shared::Credentials {
    async fn post(mut req: Request<State>, res_kind: ResponseKind) -> tide::Result<Response> {
        let credentials: Self = dbg!(req.body_json().await)?;

        let stored_user = req
            .state()
            .database()
            .get_user(&credentials.user_name)
            .await?;

        Ok({
            let mut res = Response::new(200);
            res.set_body(Self::body_from(
                if let Some(stored_user) = stored_user {
                    if stored_user.verify_credentials(&credentials)? {
                        Ok(security::jwt::Claims::new(credentials.user_name).get_token()?)
                    } else {
                        Err(credentials::Fail::IncorrectPassword)
                    }
                } else {
                    Err(credentials::Fail::UserNotFound)
                },
                res_kind,
            )?);
            res
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
