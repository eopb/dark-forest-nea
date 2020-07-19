use crate::{state, ui, updates};
use seed::{prelude::*, *};
use seed_style::{em, px, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    log!("hi");
    match &model.route_data.editor {
        Ok(project) => div![
            h2![s().font_size(em(4)).margin(px(5)), project.name.clone()],
            ui::form::InputBuilder::text_area()
                .id("project_name")
                .placeholder(&project.description)
                .view(model, |_| None)
        ],
        Err(error) => div!["some error"],
    }
}
