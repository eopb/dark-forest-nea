use crate::{
    state,
    ui::{self, View},
    updates,
};
use seed::{prelude::*, *};

use seed_style::{em, px, vh, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    let button = |model| ui::style::button(model, 3);

    div![form![
        attrs! {At::Action => "/api/sign-in", At::Method => "post"},
        s().display("flex")
            .align_items("center")
            .flex_direction("column")
            .margin("auto"),
        ui::form::text(model, "user_name", "Username..."),
        ui::form::password(model, "password", "Password..."),
        ui::form::submit(model, "Sign In"),
        ui::subheading(vec![
            Node::from_html("Don't have an account? "),
            a![button(model), "Create account.", attrs! {At::Href => "#"}].into_nodes()
        ])
    ]]
}
