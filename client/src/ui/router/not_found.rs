use crate::{state, updates};
use seed::prelude::*;

pub fn view(_: &state::Model) -> Node<updates::Msg> {
    crate::ui::title_card::view("404", Some("Page not found"))
}
