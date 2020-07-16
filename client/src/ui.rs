//! Visible items to be added to the DOM.

pub mod bordered;
pub mod form;
pub mod router;
pub mod style;
pub mod title_card;
pub mod view;

pub use {bordered::Bordered, view::View};

use crate::{state, updates};

use {
    seed::{prelude::*, *},
    seed_style::{em, px, vh, *},
};

/// Main view describing what to show for all routes.
pub fn view(model: &state::Model) -> impl IntoNodes<updates::Msg> {
    Bordered::new(vec![nav(model), router::view(model), footer(model)])
        .outer(s().min_height(vh(100)))
        .inner(
            s().display_grid()
                .grid_auto_flow_row()
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

/// Global navigation bar at top of page.
fn nav(model: &state::Model) -> Node<updates::Msg> {
    pub fn a() -> seed_style::Style {
        s().margin(px(14)).margin_top(px(8))
    }

    let button = |model| style::button(model, 5);

    nav![
        s().display_flex()
            .justify_content_space_between()
            .font_size(em(3)),
        div![
            if model.route == shared::Route::Index.into() {
                empty()
            } else {
                a![
                    a(),
                    button(model),
                    "Home",
                    attrs! {At::Href => shared::Route::Index}
                ]
            },
            if model.route == shared::Route::Explore.into() {
                empty()
            } else {
                a![
                    a(),
                    button(model),
                    "Explore",
                    attrs! {At::Href => shared::Route::Explore}
                ]
            }
        ],
        div![
            if let Some(signed_in) = model.server.signed_in.ok() {
                match signed_in {
                    shared::data::signed_in::Res::As(_) => vec![
                        a![
                            a(),
                            button(model),
                            "New Project",
                            attrs! {At::Href => shared::Route::NewProject}
                        ],
                        a![
                            a(),
                            button(model),
                            "Sign Out",
                            input_ev(Ev::Click, |_| updates::Msg::SignOut)
                        ],
                    ],
                    shared::data::signed_in::Res::Not => vec![a![
                        a(),
                        button(model),
                        "Sign In",
                        attrs! {At::Href => shared::Route::SignIn}
                    ]],
                }
            } else {
                empty().into_nodes()
            },
            a![
                a(),
                button(model),
                s().color(model.theme.toggle_button_color()),
                s().after()
                    .background_color(model.theme.toggle_button_color()),
                "Light Mode",
                attrs! {At::Href => "#"},
                ev(Ev::Click, |_| updates::Msg::ToggleTheme)
            ]
        ]
    ]
}

/// A subheading paragraph.
pub fn subheading<Msg>(subheading: impl UpdateEl<Msg>) -> Node<Msg> {
    p![
        s().margin_top(px(5))
            .margin_bottom(px(5))
            .font_size(em(2.9)),
        subheading
    ]
}
