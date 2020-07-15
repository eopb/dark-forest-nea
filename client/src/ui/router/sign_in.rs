use crate::{
    endpoint::{Get, Post},
    state, ui, updates,
    updates::sign_in::SignIn,
    RESPONSE_KIND,
};

use std::str::FromStr;

use seed::browser::web_storage::{LocalStorage, WebStorage};
use shared::data::sign_in::Credentials;
use {
    seed::{prelude::*, *},
    seed_style::*,
    shadow_clone::shadow_clone,
    shared::Endpoint,
    web_sys::RequestCredentials::SameOrigin,
};

#[derive(Clone, Default)]
pub struct Model {
    form: Credentials,
    pub error: Option<shared::data::sign_in::Fail>,
}

pub enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
}

impl Msg {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = &mut model.route_data.sign_in;
        match self {
            Self::UsernameChanged(user_name) => inner_model.form.user_name = user_name,
            Self::PasswordChanged(password) => inner_model.form.password = password,
            Self::Submit => {
                orders.skip(); // No need to rerender
                orders.send_msg(
                    SignIn::Submit(inner_model.form.clone(), shared::Route::Index).into(),
                );
                inner_model = &mut Model::default();
            }
        }
    }
}

impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::SignInMsg(msg)
    }
}

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::sign_in::Fail>,
) -> Node<updates::Msg> {
    let error = model.route_data.sign_in.error;
    use shared::data::sign_in::Fail::{IncorrectPassword, UserNotFound};
    let user_name = |err| {
        ui::form::InputBuilder::text()
            .id("user_name")
            .placeholder("Username...")
            .error(err)
            .view(model, |text| Some(Msg::UsernameChanged(text).into()))
    };
    let password = |err| {
        ui::form::InputBuilder::password()
            .id("password")
            .placeholder("Password...")
            .error(err)
            .view(model, |text| Some(Msg::PasswordChanged(text).into()))
    };
    ui::form::view(
        model,
        vec![
            user_name(&if let Some(UserNotFound) = error {
                error
            } else {
                None
            }),
            password(&if let Some(IncorrectPassword) = error {
                error
            } else {
                None
            }),
        ],
        "Sign In",
        vec![
            Node::from_html("Don't have an account? "),
            a![
                ui::style::button(model, 3),
                "Create account.",
                attrs! {At::Href => shared::Route::CreateAccount(None)}
            ]
            .into_nodes(),
        ],
        |_| Some(updates::Msg::from(Msg::Submit)),
    )
}
