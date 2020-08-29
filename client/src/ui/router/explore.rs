use crate::{state, ui::View, updates};
use seed::{prelude::*, *};
use tracing::{info, instrument};

#[instrument(skip(model))]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    div![crate::ui::title_card::view("Explore", None)]
}
