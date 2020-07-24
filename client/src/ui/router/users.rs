pub mod route;

use crate::{state, updates};

use seed::prelude::*;

use shared::routes::UserRoute;

pub fn view(
    model: &state::Model,
    user_name: &str,
    user_route: Option<&UserRoute>,
) -> Node<updates::Msg> {
    match user_route {
        Some(user_route) => route::view(model, user_name, user_route),
        // TODO
        None => todo!(),
    }
}
