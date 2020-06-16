use seed_style::*;

use crate::{state, updates};
use seed::{prelude::*, *};

// `view` describes what to display.
pub fn view(model: &state::Model) -> impl View<updates::Msg> {
    div![
        nav(model),
        button![model, ev(Ev::Click, |_| updates::Msg::Increment),],
        title_card(model),
        footer(model)
    ]
}

fn title_card(model: &state::Model) -> Node<updates::Msg> {
    header![
        h1!["Dark Forest"],
        p!["Play and create original interactive stories"],
    ]
}

fn footer(model: &state::Model) -> Node<updates::Msg> {
    footer![p![
        "A project created by ",
        a![
            "Ethan Brierley",
            attrs! {At::Href => "https://github.com/ethanboxx"}
        ],
    ]]
}

fn nav(model: &state::Model) -> Node<updates::Msg> {
    nav![
        div![
            a![
                "Home",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ],
            a![
                "Explore",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ]
        ],
        div![
            a![
                "New Project",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ],
            a![
                "Sign In",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ],
            a![
                "Light Mode",
                attrs! {At::Href => "https://github.com/ethanboxx"}
            ]
        ]
    ]
}
