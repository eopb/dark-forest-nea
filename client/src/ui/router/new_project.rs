use crate::{endpoint::Post, state, ui, updates};

use {
    seed::{log, prelude::*},
    shadow_clone::shadow_clone,
};

use shared::{
    endpoint::new_project::{self, NewProject},
    security::Authenticated,
};

#[derive(Clone, Default)]
pub struct Model {
    form: new_project::Details,
    error: Option<new_project::Fail>,
}

pub enum Msg {
    ProjectNameChanged(String),
    Submit,
    Submited(Result<(), new_project::Fail>),
    SubmitFailed(String),
}

impl Msg {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = &mut model.route_data.new_project;
        match self {
            Self::ProjectNameChanged(project_name) => inner_model.form.project_name = project_name,
            Self::Submit => {
                orders.skip(); // No need to rerender
                shadow_clone!(inner_model);
                let login_token = model.login_token.clone();
                orders.perform_cmd(async move {
                    updates::Msg::from({
                        if let Some(login_token) = login_token {
                            if let Ok(response) =
                                NewProject::fetch(Authenticated::new(inner_model.form, login_token))
                                    .await
                            {
                                Self::Submited(response)
                            } else {
                                Self::SubmitFailed("Http request failed".to_owned())
                            }
                        } else {
                            Self::SubmitFailed("No login token".to_owned())
                        }
                    })
                });
            }
            Self::Submited(result) => {
                if let Err(error) = result {
                    inner_model.error = Some(error)
                } else {
                    Url::go_and_load_with_str(
                        shared::Route::Users {
                            user_name: "ethanboxx".to_owned(),
                            nest: Some(shared::routes::UserRoute::Projects(Some(
                                shared::routes::Project {
                                    project_name: inner_model.form.project_name.clone(),
                                    nest: Some(shared::routes::ProjectRoute::Edit),
                                },
                            ))),
                        }
                        .to_string(),
                    );
                    *inner_model = Model::default();
                }
            }
            Self::SubmitFailed(reason) => log!(reason),
        }
    }
}
impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::NewProjectForm(msg)
    }
}
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    let error = model.route_data.new_project.error.as_ref();
    let project_name = |err| {
        ui::form::InputBuilder::text()
            .id("project_name")
            .placeholder("Project Name...")
            .error(err)
            .view(model, |text| Some(Msg::ProjectNameChanged(text).into()))
    };
    ui::form::view(
        model,
        project_name(&error.and_then(|error| match error {
            new_project::Fail::AlreadyExists => {
                Some("You already have a project under that name.".to_owned())
            }
            new_project::Fail::InvalidField(error) => error.project_name.map(|x| x.show("Project")),
        })),
        "Create Project",
        Vec::<Node<updates::Msg>>::new(),
        |_| Some(updates::Msg::from(Msg::Submit)),
    )
}
