pub mod router;
pub mod style;
pub mod title_card;

use seed_style::{em, px, vh, *};

use crate::{routes::Route, state, updates, View};
use seed::{prelude::*, *};

// `view` describes what to display.
pub fn view(model: &state::Model) -> impl IntoNodes<updates::Msg> {
    div![
        s().background_color(model.theme.background())
            .color(model.theme.text())
            .text_align("center")
            .min_height(vh(100))
            .padding(px(8))
            .display("flex")
            .flex_direction("column"),
        div![
            s().flex("1")
                .display("grid")
                .grid_auto_flow("row")
                .grid_template_rows("min-content auto min-content")
                .padding(px(0))
                .min_height("100%")
                .border(AsRef::<str>::as_ref(&format!(
                    "6px solid {}",
                    model.theme.text()
                )))
                .position("relative"),
            style::pixel_cutouts(model),
            nav(model),
            router::view(model),
            footer(model)
        ],
    ]
}

fn footer(model: &state::Model) -> Node<updates::Msg> {
    footer![p![
        s().margin(px(2)).font_size(em(2)),
        "A project created by ",
        a![
            style::button(model, 2),
            "Ethan Brierley",
            attrs! {At::Href => "https://github.com/ethanboxx"}
        ],
    ]]
}

fn nav(model: &state::Model) -> Node<updates::Msg> {
    pub fn a() -> seed_style::Style {
        s().margin(px(14)).margin_top(px(8))
    }

    let button = |model| style::button(model, 5);

    nav![
        s().display("flex")
            .justify_content("space-between")
            .font_size(em(3)),
        div![
            if model.route != Route::Index {
                a![
                    a(),
                    button(model),
                    "Home",
                    attrs! {At::Href => Route::Index.go_to()}
                ]
            } else {
                empty()
            },
            if model.route != Route::Explore {
                a![
                    a(),
                    button(model),
                    "Explore",
                    attrs! {At::Href => Route::Explore.go_to()}
                ]
            } else {
                empty()
            }
        ],
        div![
            a![
                a(),
                button(model),
                "New Project",
                attrs! {At::Href => "/new-project"}
            ],
            a![
                a(),
                button(model),
                "Sign In",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ],
            a![
                a(),
                button(model),
                s().color(model.theme.toggle_buttons_color()),
                s().after()
                    .background_color(model.theme.toggle_buttons_color()),
                "Light Mode",
                attrs! {At::Href => "#"},
                ev(Ev::Click, |_| updates::Msg::ToggleTheme)
            ]
        ]
    ]
}
