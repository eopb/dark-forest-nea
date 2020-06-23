use crate::{routes::Route, state, Endpoint as _};
use seed::{browser::fetch::FetchError, prelude::*};

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
            if route != model.route {
                (*model).server = state::Server::default();
                (*model).route = route;
                route.request_required_data(orders)
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
}

impl ToFetch {
    async fn order(self) -> Msg {
        match self {
            Self::Hello => Msg::DataFetched(Fetched::Hello(glue::Hello::fetch().await)),
        }
    }
}

pub enum Fetched {
    Hello(Result<glue::Hello, FetchError>),
}

impl Fetched {
    fn add_to(self, model: &mut state::Model) {
        match self {
            Self::Hello(x) => model.server.hello = state::server::Fetch::Fetched(x),
        }
    }
}
