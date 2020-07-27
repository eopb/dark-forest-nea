use crate::{state, ui::View, updates};
use seed::{prelude::*, *};
use tracing::{info, instrument};

#[instrument]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    info!("hello my friend");
    div![
        crate::ui::title_card::view(
            "Dark Forest",
            "Play and create original interactive stories",
        ),
        model.server.hello.view(model)
    ]
}
