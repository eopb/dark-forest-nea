use crate::{ui, updates};
use gloo_timers::callback::Timeout;
use seed::{prelude::*, *};
use seed_hooks::*;
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

fn typewriter(input: &str) -> Node<updates::Msg> {
    let input = use_state(|| input.chars().rev().collect::<String>());
    let current_display = use_state(|| "".to_string());
    let remaining = input.set(
        current_display
            .trim_start_matches(&current_display.get())
            .to_owned(),
    );

    let mut input_to_pop = input.get();
    let next_character = input_to_pop.pop();
    // input_to_pop.set(input_to_pop);

    if let Some(next_character) = next_character {
        current_display.update(|current_display| current_display.push(next_character));
        span![current_display]
    } else {
        // we are finished
        let timeout = Timeout::new(100, move || {
            // Do something...\
        });
        span![current_display]
    }
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
        view("Dark Forest", "A cool subheading").update_el(&mut element);
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
