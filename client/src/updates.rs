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

use shared::{
    endpoint::{
        edit::{init::StartEditor, ProjectPath},
        hello::Hello,
        refresh_token::RefreshToken,
        signed_in::{self, SignedIn},
    },
    routes::SubRoute,
    security::{Authenticated, Token},
};

/// Describes the different events that can be invoked.
pub enum Msg {
    ToggleTheme,
    RefreshToken,
    ChangeRoute(Route),
    DataFetched(Fetched),
    ToFetch(ToFetch),
    SignIn(SignIn),
    SignInForm(ui::router::sign_in::Msg),
    CreateAccountForm(ui::router::create_account::Msg),
    NewProjectForm(ui::router::new_project::Msg),
    Editor(ui::router::users::route::project::route::editor::Msg),
    ClearRouteData,
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
            orders.perform_cmd(x.order(model.login_token.clone()));
            orders.skip();
        }
        Msg::DataFetched(x) => x.add_to(model),
        Msg::RefreshToken => {
            use state::server::Fetch::Fetched;
            // Only run this When a user is signed in.
            if let Fetched(Ok(signed_in::Res::As(_))) = model.server.signed_in {
                orders.send_msg(Msg::ToFetch(ToFetch::RefreshToken));
            }
        }
        Msg::SignInForm(msg) => msg.update(model, orders),
        Msg::CreateAccountForm(msg) => msg.update(model, orders),
        Msg::NewProjectForm(msg) => msg.update(model, orders),
        Msg::SignOut => {
            model.login_token = None;
            LocalStorage::remove(LOGIN_KEY);
            orders.send_msg(Msg::ToFetch(ToFetch::SignedIn));
        }
        Msg::SignIn(x) => x.update(model, orders),
        Msg::Editor(x) => x.update(model, orders),
        Msg::ClearRouteData => model.route_data = state::RouteData::default(),
    }
}

/// An item that must be fetched.
pub enum ToFetch {
    Hello,
    SignedIn,
    RefreshToken,
    Editor(ProjectPath),
}

impl ToFetch {
    /// Fetch an item and inform with a message.
    async fn order(self, login_token: Option<Token>) -> Option<Msg> {
        Some(match self {
            Self::Hello => Msg::DataFetched(Fetched::Hello(Hello::fetch().await)),
            Self::SignedIn => Msg::DataFetched(Fetched::SignedIn(
                SignedIn::fetch(login_token.unwrap_or_default()).await,
            )),
            Self::RefreshToken => Msg::DataFetched(Fetched::RefreshToken(
                RefreshToken::fetch(login_token?).await,
            )),
            Self::Editor(path) => Msg::DataFetched(Fetched::Editor(
                StartEditor::fetch(Authenticated::new(path, login_token?)).await,
            )),
        })
    }
}

/// An item that has been fetched ready to be handled.
pub enum Fetched {
    Hello(anyhow::Result<<Hello as shared::Endpoint>::Response>),
    SignedIn(anyhow::Result<<SignedIn as shared::Endpoint>::Response>),
    RefreshToken(anyhow::Result<<RefreshToken as shared::Endpoint>::Response>),
    Editor(anyhow::Result<<StartEditor as shared::Endpoint>::Response>),
}

impl Fetched {
    /// Add a fetched item to the model.
    fn add_to(self, model: &mut state::Model) {
        match self {
            Self::Hello(x) => model.server.hello = state::server::Fetch::Fetched(x),
            Self::SignedIn(x) => model.server.signed_in = state::server::Fetch::Fetched(x),
            Self::Editor(x) => model.route_data.editor = x.unwrap(),
            //TODO handle refresh tokens!
            Self::RefreshToken(_) => {}
        }
    }
}
