use crate::{state, ui, updates, RESPONSE_KIND};

use seed::prelude::*;

use shared::Endpoint;

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::new_project::Fail>,
) -> Node<updates::Msg> {
    let project_name = |err| {
        ui::form::InputBuilder::text()
            .id("project_name")
            .placeholder("Project Name...")
            .error(err)
            .view(model, |_| None)
    };
    ui::form::view(
        model,
        project_name(&error.and_then(|error| match error {
            shared::data::new_project::Fail::AlreadyExists => {
                Some("You already have a project under that name.".to_owned())
            }
            shared::data::new_project::Fail::InvalidField(error) => {
                error.project_name.map(|x| x.show("Project"))
            }
        })),
        "Create Project",
        Vec::<Node<updates::Msg>>::new(),
        |_| None,
    )
}
