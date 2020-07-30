use crate::{
    endpoint::Post,
    state,
    ui::{self, view::View},
    updates,
};

use shared::{
    data::{Chapter, Decision, Project},
    endpoint::edit::{
        save::{PermissionDenied, SaveEditor},
        ProjectPath,
    },
    security::Authenticated,
};

use {
    seed::{prelude::*, *},
    seed_style::{em, pc, px, *},
    shadow_clone::shadow_clone,
    tracing::{info, instrument, trace},
};

type Model = State;

#[derive(Debug)]
pub struct State {
    chapter_key: usize,
}

impl Default for State {
    /// Start on the first chapter.
    fn default() -> Self {
        Self { chapter_key: 1 }
    }
}

#[derive(Debug)]
pub enum Msg {}

impl Msg {
    #[instrument(skip(model, orders))]
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        match self {}
    }
}
#[instrument(skip(model))]
pub fn view(model: &state::Model, project_path: ProjectPath) -> Node<updates::Msg> {
    info!("rendering player");
    trace!(project = format!("{:#?}", model.route_data.project).as_str());
    div!["hello from the player"]
}
