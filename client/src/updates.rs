//! Update message definitions and handling.

// TODO maybe look into using trait objects for `ToFetch` and `Fetched`

pub mod sign_in;

use crate::{
    endpoint::{Get, Post},
    routes::Route,
    state, ui, LOGIN_KEY,
};

use sign_in::SignIn;

use {seed::prelude::*, web_sys::Window};

use shared::routes::SubRoute;

/// Describes the different events that can be invoked.
pub enum Msg {
    ToggleTheme,
    RefreshToken,
    ChangeRoute(Route),
    DataFetched(Fetched),
    ToFetch(ToFetch),
    SignIn(SignIn),
    SignInMsg(ui::router::sign_in::Msg),
    CreateAccountMsg(ui::router::create_account::Msg),
    NewProjectMsg(ui::router::new_project::Msg),
    SignOut,
}

/// Describes how to handle each `Msg` often by updating the model.
pub fn update(msg: Msg, model: &mut state::Model, orders: &mut impl Orders<Msg>) {
    #[allow(unused_must_use)] // The best thing to do with failed `LocalStorage` is to ignore.
    match msg {
        Msg::ToggleTheme => model.theme.toggle(),
        Msg::ChangeRoute(route) => {
            if route.0.is_none() || route != model.route {
                model.server = state::Server::default();
                model.route = route.clone();
                route.request_required_data(orders);
                if let Some(doc) = web_sys::window().as_ref().and_then(Window::document) {
                    doc.set_title(
                        &route
                            .0
                            .as_ref()
                            .map_or("Page Not Found".to_owned(), shared::Route::title),
                    )
                }
            }
        }
        Msg::ToFetch(x) => {
            //TODO This clone could be too expensie.
            orders.perform_cmd(x.order(model.login_token.clone()));
            orders.skip();
        }
        Msg::DataFetched(x) => x.add_to(model),
        Msg::RefreshToken => {
            use state::server::Fetch::Fetched;
            // Only run this When a user is signed in.
            if let Fetched(Ok(shared::data::signed_in::Res::As(_))) = model.server.signed_in {
                orders.send_msg(Msg::ToFetch(ToFetch::RefreshToken));
            }
        }
        Msg::SignInMsg(msg) => msg.update(model, orders),
        Msg::CreateAccountMsg(msg) => msg.update(model, orders),
        Msg::NewProjectMsg(msg) => msg.update(model, orders),
        Msg::SignOut => {
            model.login_token = None;
            LocalStorage::remove(LOGIN_KEY);
            orders.send_msg(Msg::ToFetch(ToFetch::SignedIn));
        }
        Msg::SignIn(x) => x.update(model, orders),
    }
}

/// An item that must be fetched.
pub enum ToFetch {
    Hello,
    SignedIn,
    RefreshToken,
}

impl ToFetch {
    /// Fetch an item and inform with a message.
    async fn order(self, login_token: Option<String>) -> Option<Msg> {
        Some(match self {
            Self::Hello => Msg::DataFetched(Fetched::Hello(shared::Hello::fetch().await)),
            Self::SignedIn => Msg::DataFetched(Fetched::SignedIn(
                shared::SignedIn::fetch(login_token.unwrap_or_default()).await,
            )),
            Self::RefreshToken => Msg::DataFetched(Fetched::RefreshToken(
                shared::RefreshToken::fetch(login_token?).await,
            )),
        })
    }
}

/// An item that has been fetched ready to be handled.
pub enum Fetched {
    Hello(anyhow::Result<<shared::Hello as shared::Endpoint>::Response>),
    SignedIn(anyhow::Result<<shared::SignedIn as shared::Endpoint>::Response>),
    RefreshToken(anyhow::Result<<shared::RefreshToken as shared::Endpoint>::Response>),
}

impl Fetched {
    /// Add a fetched item to the model.
    fn add_to(self, model: &mut state::Model) {
        match self {
            Self::Hello(x) => model.server.hello = state::server::Fetch::Fetched(x),
            Self::SignedIn(x) => model.server.signed_in = state::server::Fetch::Fetched(x),
            // Refresh token only affects cookies so it does not have to be handled here.
            Self::RefreshToken(_) => {}
        }
    }
}
