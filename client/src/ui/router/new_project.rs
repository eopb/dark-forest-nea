use crate::{state, ui, updates, RESPONSE_KIND};

use seed::prelude::*;

use shared::Endpoint;

pub fn view(
    model: &state::Model,
    error: Option<&shared::data::new_project::Fail>,
) -> Node<updates::Msg> {
    let project_name =
        |error| ui::form::text_with_error(model, "project_name", "Project Name...", error);
    ui::form::view(
        model,
        shared::NewProject::path(RESPONSE_KIND),
        project_name(error.and_then(|error| match error {
            shared::data::new_project::Fail::AlreadyExists => {
                Some("You already have a project under that name.".to_owned())
            }
            shared::data::new_project::Fail::InvalidField(error) => {
                error.project_name.map(|x| x.show("Project"))
            }
        })),
        "Create Project",
        Vec::<Node<updates::Msg>>::new(),
    )
}
