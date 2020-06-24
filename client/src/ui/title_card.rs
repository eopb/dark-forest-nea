use crate::ui;
use seed::{prelude::*, *};
use seed_style::{em, px, *};

pub fn view<Msg>(header: &str, subheading: &str) -> Node<Msg> {
    header![
        h1![
            s().font_family("bitlimt")
                .font_size(em(6))
                .margin_bottom(px(5)),
            header
        ],
        ui::subheading(subheading),
    ]
}
