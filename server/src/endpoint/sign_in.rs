use {async_trait::async_trait, tide::Request};

use crate::{endpoint, security, state::State};

use shared::{
    endpoint::sign_in::{self, SignIn},
    security::Token,
};

#[async_trait]
impl endpoint::Post for SignIn {
    async fn post(
        req: Request<State>,
        credentials: sign_in::Credentials,
    ) -> tide::Result<Result<Token, sign_in::Fail>> {
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
