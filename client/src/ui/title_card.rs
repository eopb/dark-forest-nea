use crate::ui;
use seed::{prelude::*, *};
use seed_style::{em, px, *};

pub fn view<Msg>(header: &str, subheading: Option<&str>) -> Node<Msg> {
    header![
        h1![
            s().font_family("bitlimt")
                .font_size(em(6))
                .margin_bottom(px(5)),
            header
        ],
        subheading.map(ui::subheading)
    ]
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    use crate::updates;

    #[wasm_bindgen_test]
    fn test() {
        let mut element = El::<updates::Msg>::empty(Tag::Div);
        view("Dark Forest", Some("A cool subheading")).update_el(&mut element);
        let children = element.children;
        let header = children[0].clone();

        assert!(matches!(
            header,
            Node::Element(El {
                tag: Tag::Header, ..
            })
        ))
    }
}
