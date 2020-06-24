pub mod bordered;
pub mod form;
pub mod router;
pub mod style;
pub mod title_card;
pub mod view;

pub use {bordered::Bordered, view::View};

use seed_style::{em, px, vh, *};

use crate::{state, updates};
use seed::{prelude::*, *};

// `view` describes what to display.
pub fn view(model: &state::Model) -> impl IntoNodes<updates::Msg> {
    Bordered::new(vec![nav(model), router::view(model), footer(model)])
        .outer(s().min_height(vh(100)))
        .inner(
            s().display("grid")
                .grid_auto_flow("row")
                .grid_template_rows("min-content auto min-content"),
        )
        .view(model)
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
            if model.route == glue::Route::Index.into() {
                empty()
            } else {
                a![
                    a(),
                    button(model),
                    "Home",
                    attrs! {At::Href => glue::Route::Index}
                ]
            },
            if model.route == glue::Route::Explore.into() {
                empty()
            } else {
                a![
                    a(),
                    button(model),
                    "Explore",
                    attrs! {At::Href => glue::Route::Explore}
                ]
            }
        ],
        div![
            a![
                a(),
                button(model),
                "New Project",
                attrs! {At::Href => glue::Route::NewProject}
            ],
            a![
                a(),
                button(model),
                "Sign In",
                attrs! {At::Href => glue::Route::SignIn}
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

pub fn subheading<Msg>(subheading: impl UpdateEl<Msg>) -> Node<Msg> {
    p![s().margin_top(px(5)).font_size(em(2.9)), subheading]
}
