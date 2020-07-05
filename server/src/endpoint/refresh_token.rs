use {
    async_trait::async_trait,
    tide::{http::StatusCode, Request, Response},
};

use crate::{
    endpoint::{self, sign_in::unsafe_sign_in, signed_in::Ext, Endpoint},
    state::State,
};

use shared::data::ResponseKind;

impl Endpoint for shared::RefreshToken {}

#[async_trait]
impl endpoint::Get for shared::RefreshToken {
    async fn get(req: Request<State>, _: ResponseKind) -> tide::Result<Response> {
        let user = shared::SignedIn::get_user(&req).await;

        Ok(match user {
            shared::SignedIn::As(user) => unsafe_sign_in(Response::new(StatusCode::Ok), user)?,
            shared::SignedIn::Not => Response::new(StatusCode::Forbidden),
        })
    }
}
