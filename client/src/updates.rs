use crate::{routes::Route, state, Endpoint as _};
use {
    seed::{browser::fetch::FetchError, prelude::*},
    web_sys::Window,
};

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    ToggleTheme,
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
                web_sys::window()
                    .as_ref()
                    .and_then(Window::document)
                    .map(|doc| {
                        doc.set_title(
                            route
                                .0
                                .as_ref()
                                .map(glue::Route::title)
                                .unwrap_or("Page Not Found"),
                        )
                    });
            }
        }
        Msg::ToFetch(x) => {
            orders.perform_cmd(x.order());
            orders.skip();
        }
        Msg::DataFetched(x) => x.add_to(model),
    }
}

pub enum ToFetch {
    Hello,
    SignedIn,
}

impl ToFetch {
    async fn order(self) -> Msg {
        match self {
            Self::Hello => Msg::DataFetched(Fetched::Hello(glue::Hello::fetch().await)),
            Self::SignedIn => Msg::DataFetched(Fetched::SignedIn(glue::SignedIn::fetch().await)),
        }
    }
}

pub enum Fetched {
    Hello(Result<glue::Hello, FetchError>),
    SignedIn(Result<glue::SignedIn, FetchError>),
}

impl Fetched {
    fn add_to(self, model: &mut state::Model) {
        match self {
            Self::Hello(x) => model.server.hello = state::server::Fetch::Fetched(x),
            Self::SignedIn(x) => model.server.signed_in = state::server::Fetch::Fetched(x),
        }
    }
}
