pub mod global;
use crate::{state, updates};
use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};
use seed_style::LocalUpdateEl;
use seed_style::*;
use seed_style::{px, *};

pub fn pixel_cutouts(model: &state::Model) -> Vec<seed_style::Style> {
    vec![
        s().before().content("\"\"").top(px(-6)).left(px(-6)),
        s().after().content("\"\"").bottom(px(-6)).right(px(-6)),
    ]
    .into_iter()
    .map(|s| {
        s.background_color(model.theme.background())
            .display("block")
            .height(px(6))
            .width(px(6))
            .position("absolute")
    })
    .collect()
}

pub fn button(model: &state::Model, height: i32) -> Vec<seed_style::Style> {
    vec![
        s().display("inline-block")
            .position("relative")
            .text_decoration("none")
            .color(model.theme.text()),
        s().after()
            .content("\"\"")
            .display("block")
            .height(px(height))
            .background_color(model.theme.text())
            .position("absolute")
            .width("100%")
            .bottom(px(4))
            .left(px(0)),
        s().pseudo(":hover::after").bottom(px(4 + height)),
    ]
}
