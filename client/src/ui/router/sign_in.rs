use crate::{state, ui, updates, RESPONSE_KIND};

use seed::browser::web_storage::{LocalStorage, WebStorage};
use {
    seed::{prelude::*, *},
    seed_style::*,
    shared::Endpoint,
    web_sys::RequestCredentials::SameOrigin,
};

#[derive(Default)]
pub struct Model {
    form: shared::Credentials,
}

pub enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
    Submited(<shared::Credentials as Endpoint>::Response),
    SubmitFailed(String),
}

impl Msg {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = &mut model.route_data.sign_in;
        match self {
            Self::UsernameChanged(user_name) => inner_model.form.user_name = user_name,
            Self::PasswordChanged(password) => inner_model.form.password = password,
            Self::Submit => {
                log!("Hello");
                //   Url::go_and_load_with_str("/");
                orders.skip(); // No need to rerender

                log!("Hello");
                //   Url::go_and_load_with_str("/");
                //   Url::go_and_load_with_str("/");
                let request = Request::new("api/json/sign-in")
                    .method(Method::Post)
                    .header(Header::custom("Accept-Language", "en"))
                    .credentials(SameOrigin)
                    // .header(Header::content_type("application/x-www-form-urlencoded"))
                    .json(&inner_model.form)
                    .expect("Serialization failed");

                log!("Hello");
                orders.perform_cmd(async {
                    let response = fetch(request).await.expect("HTTP request failed");

                    updates::Msg::from(if response.status().is_ok() {
                        Msg::Submited(
                            serde_json::from_str(&response.text().await.unwrap()).unwrap(),
                        )
                    } else {
                        Msg::SubmitFailed(response.status().text)
                    })
                });
                log!("Hello");
            }
            Self::Submited(result) => {
                model.login_token = result.clone().ok();
                LocalStorage::insert("Login", &result.unwrap());
            }
            Self::SubmitFailed(reason) => log!("Hi"),
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
    error: Option<&shared::data::credentials::Fail>,
) -> Node<updates::Msg> {
    use shared::data::credentials::Fail::{IncorrectPassword, UserNotFound};
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
