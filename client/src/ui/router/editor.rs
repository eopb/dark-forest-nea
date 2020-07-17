use crate::{state, ui::View, updates};
use seed::{prelude::*, *};

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    match &model.route_data.editor {
        Ok(project) => div![h2![project.name.clone()]],
        Err(error) => div!["some error"],
    }
}
