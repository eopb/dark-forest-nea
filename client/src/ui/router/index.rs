use crate::{routes::Route, state, updates};
use seed::{prelude::*, *};
use seed_style::{em, px, vh, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    crate::ui::title_card::view(
        "Dark Forest",
        "Play and create original interactive stories",
    )
}
