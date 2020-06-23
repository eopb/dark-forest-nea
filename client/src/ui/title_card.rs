use crate::updates;
use seed::{prelude::*, *};
use seed_style::{em, px, *};

pub fn view(header: &str, subheading: &str) -> Node<updates::Msg> {
    header![
        h1![
            s().font_family("bitlimt")
                .font_size(em(6))
                .margin_bottom(px(5)),
            header
        ],
        p![s().margin_top(px(5)).font_size(em(2.9)), subheading],
    ]
}
