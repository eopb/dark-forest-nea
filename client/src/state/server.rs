use crate::ui;

use seed::virtual_dom::update_el::UpdateEl;
use seed::{browser::fetch::FetchError, prelude::*, *};

#[derive(Default)]
pub struct Server {
    pub hello: Fetch<glue::Hello, FetchError>,
    pub signed_in: Fetch<glue::SignedIn, FetchError>,
}

#[derive(Clone)]
pub enum Fetch<T, E> {
    Loading,
    Fetched(Result<T, E>),
}

impl<T, E> Default for Fetch<T, E> {
    fn default() -> Self {
        Self::Loading
    }
}

impl<T, E> Fetch<T, E> {
    pub fn ok(&self) -> Option<&T> {
        if let Self::Fetched(Ok(t)) = self {
            Some(t)
        } else {
            None
        }
    }
}

impl<T: ui::View<Msg, Model>, E: ui::View<Msg, Model>, Msg, Model> ui::View<Msg, Model>
    for Fetch<T, E>
{
    fn view(&self, model: &Model) -> Vec<Node<Msg>> {
        match self {
            Self::Fetched(Ok(x)) => x.view(model),
            Self::Fetched(Err(x)) => x.view(model),
            Self::Loading => p!["Loading ..."].into_nodes(), // To be replaced by nice animation.
        }
    }
}
