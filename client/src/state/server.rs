//! Data fetched from the server.

use crate::ui;

use seed::{prelude::*, virtual_dom::update_el::UpdateEl, *};

#[derive(Default)]
pub struct Server {
    pub hello: Fetch<shared::data::hello::Res, anyhow::Error>,
    pub signed_in: Fetch<shared::data::signed_in::Res, anyhow::Error>,
}

/// Fetching status.
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
    /// Get underlying `T` assuming their is no error.
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
