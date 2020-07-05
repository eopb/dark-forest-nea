use crate::{routes::Route, state, Endpoint as _};

use shared::routes::SubRoute;

use {seed::prelude::*, web_sys::Window};

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    ToggleTheme,
    RefreshTokenIfNeed,
    ChangeRoute(Route),
    DataFetched(Fetched),
    ToFetch(ToFetch),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut state::Model, orders: &mut impl Orders<Msg>) {
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
            orders.perform_cmd(x.order());
            orders.skip();
        }
        Msg::DataFetched(x) => x.add_to(model),
        Msg::RefreshTokenIfNeed => {
            use state::server::Fetch::Fetched;
            // Only run this When a user is signed in.
            if let Fetched(Ok(shared::SignedIn::As(_))) = model.server.signed_in {
                orders.send_msg(Msg::ToFetch(ToFetch::RefreshToken));
            }
        }
    }
}

pub enum ToFetch {
    Hello,
    SignedIn,
    RefreshToken,
}

impl ToFetch {
    async fn order(self) -> Msg {
        match self {
            Self::Hello => Msg::DataFetched(Fetched::Hello(shared::Hello::fetch().await)),
            Self::SignedIn => Msg::DataFetched(Fetched::SignedIn(shared::SignedIn::fetch().await)),
            Self::RefreshToken => {
                Msg::DataFetched(Fetched::RefreshToken(shared::RefreshToken::fetch().await))
            }
        }
    }
}

pub enum Fetched {
    Hello(anyhow::Result<shared::Hello>),
    SignedIn(anyhow::Result<shared::SignedIn>),
    RefreshToken(anyhow::Result<shared::RefreshToken>),
}

impl Fetched {
    fn add_to(self, model: &mut state::Model) {
        match self {
            Self::Hello(x) => model.server.hello = state::server::Fetch::Fetched(x),
            Self::SignedIn(x) => model.server.signed_in = state::server::Fetch::Fetched(x),
            // Refresh token only affects cookies so it does not have to be handled here.
            Self::RefreshToken(_) => {}
        }
    }
}
