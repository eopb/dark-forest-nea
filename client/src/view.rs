use seed::virtual_dom::update_el::UpdateEl;
use seed::{browser::fetch::FetchError, prelude::*, *};

use crate::state;

pub trait View<Msg, Model> {
    fn view(&self, model: &Model) -> Vec<Node<Msg>>;
}

impl<Msg, Model> View<Msg, Model> for glue::Hello {
    fn view(&self, _: &Model) -> Vec<Node<Msg>> {
        p![&self.msg].into_nodes()
    }
}

impl<Msg, Model, T: View<Msg, Model>> View<Msg, Model> for Option<T> {
    fn view(&self, model: &Model) -> Vec<Node<Msg>> {
        self.as_ref()
            .map(|x| x.view(model))
            .unwrap_or_else(|| empty().into_nodes())
    }
}

impl<Msg, Model> View<Msg, Model> for () {
    fn view(&self, _: &Model) -> Vec<Node<Msg>> {
        empty().into_nodes()
    }
}

impl<Msg, Model> View<Msg, Model> for FetchError {
    fn view(&self, _: &Model) -> Vec<Node<Msg>> {
        p![&format!("Failed to fetch info. \"Error Info: {:?}\"", self)].into_nodes()
    }
}
