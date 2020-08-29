use crate::{state, ui::View, updates};
use seed::{prelude::*, *};
use tracing::{info, instrument};

#[instrument(skip(model))]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    info!("hello my friend");
    div![
        crate::ui::title_card::view(
            "Dark Forest",
            Some("Play and create original interactive stories"),
        ),
        model.server.hello.view(model)
    ]
}
