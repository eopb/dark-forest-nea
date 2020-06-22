use crate::{routes::Route, state};
use seed::{prelude::*, *};

#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    ToggleTheme,
    ChangeRoute(Route),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut state::Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleTheme => model.theme.toggle(),
        Msg::ChangeRoute(route) => (*model).route = route,
    }
}
