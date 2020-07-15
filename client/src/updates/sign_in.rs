use crate::{endpoint::Post, state, updates, LOGIN_KEY};

use {
    seed::{
        browser::web_storage::{LocalStorage, WebStorage},
        prelude::*,
        *,
    },
    shadow_clone::shadow_clone,
};

use shared::{data::sign_in::Credentials, Endpoint, Route};

pub enum SignIn {
    Submit(Credentials, Route),
    Submited(<shared::SignIn as Endpoint>::Response, Route),
    SubmitFailed(String),
}
impl SignIn {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        #![allow(unused_must_use)] // The best thing to do with failed `LocalStorage` is to ignore.
        let mut inner_model = &mut model.route_data.sign_in;
        match self {
            Self::Submit(credentials, route) => {
                orders.skip(); // No need to rerender
                shadow_clone!(inner_model);
                orders.perform_cmd(async move {
                    updates::Msg::from(
                        if let Some(response) = shared::SignIn::fetch(credentials).await.ok() {
                            SignIn::Submited(response, route)
                        } else {
                            SignIn::SubmitFailed("Http request failed".to_owned())
                        },
                    )
                });
            }
            Self::Submited(result, route) => match result {
                Ok(result) => {
                    model.login_token = Some(result.clone());
                    LocalStorage::insert(LOGIN_KEY, &result);
                    Url::go_and_load_with_str(&route.to_string());
                }
                Err(error) => inner_model.error = Some(error),
            },
            Self::SubmitFailed(reason) => log!(reason),
        }
    }
}

impl From<SignIn> for updates::Msg {
    fn from(msg: SignIn) -> Self {
        Self::SignIn(msg)
    }
}
