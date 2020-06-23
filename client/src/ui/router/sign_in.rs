use crate::{
    state,
    ui::{self, View},
    updates,
};
use seed::{prelude::*, *};

use seed_style::{em, px, vh, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    div![form![
        attrs! {At::Action => "/api/sign-in", At::Method => "post"},
        s().display("flex")
            .align_items("center")
            .flex_direction("column")
            .margin("auto"),
        ui::Bordered::new(p!["heloo"])
            .inner(s().width(px(600)))
            .view(model)
    ]]
}
