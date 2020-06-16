use crate::{state, updates};
use seed::{prelude::*, *};

// `view` describes what to display.
pub fn view(model: &state::Model) -> impl IntoNodes<updates::Msg> {
    div![
        style! {
            St::Color => "green";
        },
        id!("unique-element"),
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| updates::Msg::Increment),]
    ]
}
