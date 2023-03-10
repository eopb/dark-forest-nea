use crate::{endpoint::Post, state, ui, updates, updates::sign_in::SignIn};

use {
    secrecy::{ExposeSecret, Secret, SecretString},
    seed::{prelude::*, *},
    seed_style::*,
    shadow_clone::shadow_clone,
    tracing::instrument,
};

use shared::endpoint::create_account::{self, CreateAccount};
#[derive(Clone, Default, Debug)]
pub struct Model {
    form: create_account::Details,
    error: Option<create_account::Fail>,
}

#[derive(Debug)]
pub enum Msg {
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(SecretString),
    Submit,
    Submited(<CreateAccount as shared::Endpoint>::Response),
    SubmitFailed(String),
}

impl Msg {
    #[instrument(skip(model, orders))]
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = &mut model.route_data.create_account;
        match self {
            Self::UsernameChanged(user_name) => inner_model.form.user_name = user_name,
            Self::EmailChanged(email) => inner_model.form.email = email,
            Self::PasswordChanged(password) => inner_model.form.password = password,
            Self::Submit => {
                orders.skip(); // No need to rerender
                shadow_clone!(inner_model);
                orders.perform_cmd(async move {
                    updates::Msg::from(
                        if let Ok(response) = CreateAccount::fetch(inner_model.form).await {
                            Self::Submited(response)
                        } else {
                            Self::SubmitFailed("Http request failed".to_owned())
                        },
                    )
                });
            }
            Self::Submited(result) => {
                if let Err(error) = result {
                    inner_model.error = Some(error)
                } else {
                    orders.send_msg(
                        SignIn::Start {
                            credentials: inner_model.form.clone().into(),
                            goes_to: shared::Route::Index,
                        }
                        .into(),
                    );
                    *inner_model = Model::default();
                }
            }
            Self::SubmitFailed(reason) => log!(reason),
        }
    }
}
impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::CreateAccountForm(msg)
    }
}

#[instrument(skip(model), name = "create_account view")]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    let form = &model.route_data.create_account.form;
    let error = model.route_data.create_account.error.as_ref();
    let user_name = |err| {
        ui::form::InputBuilder::text()
            .id("user_name")
            .placeholder("Username...")
            .value(&form.user_name)
            .error(err)
            .view(model, |text| Some(Msg::UsernameChanged(text).into()))
    };
    let email = |err| {
        ui::form::InputBuilder::email()
            .id("email")
            .placeholder("Email...")
            .value(&form.email)
            .error(err)
            .view(model, |text| Some(Msg::EmailChanged(text).into()))
    };
    let password = |err| {
        ui::form::InputBuilder::password()
            .id("password")
            .placeholder("Password...")
            .value(form.password.expose_secret())
            .error(err)
            .view(model, |text| {
                Some(Msg::PasswordChanged(Secret::new(text)).into())
            })
    };
    ui::form::view(
        model,
        match error {
            Some(error) => match error {
                create_account::Fail::AlreadyExists => vec![
                    user_name(&Some("Username already taken.".to_owned())),
                    email(&None),
                    password(&None),
                ],
                create_account::Fail::InvalidField(error) => vec![
                    user_name(&error.user_name.map(|x| x.show("Username"))),
                    email(&error.email.map(|x| x.show("Email"))),
                    password(&error.password.map(|x| x.show("Password"))),
                ],
            },
            None => vec![user_name(&None), email(&None), password(&None)],
        },
        "Create Account",
        vec![
            Node::from_html("Already have account? "),
            a![
                ui::style::button(model, 3),
                "Sign In.",
                attrs! {At::Href => shared::Route::SignIn}
            ]
            .into_nodes(),
        ],
        |_| Some(updates::Msg::from(Msg::Submit)),
    )
}
