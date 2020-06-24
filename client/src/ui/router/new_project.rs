use crate::{state, ui, updates};
use seed::prelude::*;

pub fn view(model: &state::Model) -> Node<updates::Msg> {
    ui::form::view(
        model,
        "/api/new-project",
        ui::form::text(model, "project_name", "Project Name..."),
        "Create Project",
        Vec::<Node<updates::Msg>>::new(),
    )
}
