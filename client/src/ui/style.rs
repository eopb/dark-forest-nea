//! Useful styles to apply to elements.

pub mod global;
use crate::state;
use seed_style::{px, *};

/// Cutouts for borders using pseudo-elements
pub fn pixel_cutouts(model: &state::Model) -> Vec<seed_style::Style> {
    vec![
        s().before().content("\"\"").top(px(-6)).left(px(-6)),
        s().after().content("\"\"").bottom(px(-6)).right(px(-6)),
    ]
    .into_iter()
    .map(|s| {
        s.background_color(model.theme.background())
            .display_block()
            .height(px(6))
            .width(px(6))
            .position_absolute()
    })
    .collect()
}

pub fn button(model: &state::Model, height: i32) -> Vec<seed_style::Style> {
    vec![
        s().display_inline_block()
            .position_relative()
            .text_decoration_none()
            .color(model.theme.text()),
        s().after()
            .content("\"\"")
            .display_block()
            .height(px(height))
            .background_color(model.theme.text())
            .position_absolute()
            .width(pc(100))
            .bottom(px(4))
            .left(px(0)),
        s().pseudo(":hover::after").bottom(px(4 + height)),
    ]
}
