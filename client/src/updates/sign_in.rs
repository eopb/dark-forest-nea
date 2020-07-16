//! State machine for signing in users.

use crate::{endpoint::Post, state, updates, LOGIN_KEY};

use {
    seed::{
        browser::web_storage::{LocalStorage, WebStorage},
        prelude::*,
        *,
    },
    shadow_clone::shadow_clone,
};

use shared::{
    endpoint::sign_in::{self, Credentials},
    security, Route,
};

/// Sign in states.
pub enum SignIn {
    Start {
        credentials: Credentials,
        /// Route to redirect to when process finishes.
        goes_to: Route,
    },
    Responded {
        response: Result<security::Token, sign_in::Fail>,
        goes_to: Route,
    },
    Failed(String),
}

impl SignIn {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        #![allow(unused_must_use)] // The best thing to do with failed `LocalStorage` is to ignore.
        let mut inner_model = &mut model.route_data.sign_in;
        match self {
            Self::Start {
                credentials,
                goes_to,
            } => {
                orders.skip(); // No need to rerender
                shadow_clone!(inner_model);
                orders.perform_cmd(async move {
                    updates::Msg::from(
                        if let Ok(response) = sign_in::SignIn::fetch(credentials).await {
                            Self::Responded { response, goes_to }
                        } else {
                            Self::Failed("Http request failed".to_owned())
                        },
                    )
                });
            }
            Self::Responded { response, goes_to } => match response {
                Ok(result) => {
                    model.login_token = Some(result.clone());
                    LocalStorage::insert(LOGIN_KEY, &result);
                    Url::go_and_load_with_str(&goes_to.to_string());
                }
                Err(error) => inner_model.error = Some(error),
            },
            Self::Failed(reason) => log!(reason),
        }
    }
}

impl From<SignIn> for updates::Msg {
    fn from(msg: SignIn) -> Self {
        Self::SignIn(msg)
    }
}
