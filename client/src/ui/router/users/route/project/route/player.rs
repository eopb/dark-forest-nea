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

#[derive(Debug, Default)]
pub struct State {
    chapter_key: Position,
}

#[derive(Debug)]
enum Position {
    Start,
    Chapter(usize),
    End,
}

impl Default for Position {
    fn default() -> Self {
        Self::Start
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
    let project = &model.route_data.project;
    info!("rendering player");
    trace!(project = format!("{:#?}", project).as_str());
    div![div![
        s().display("flex")
            .align_items("center")
            .flex_direction("column")
            .margin("auto"),
        div![
            s().font_size(em(2.9)),
            h2![
                s().margin("0").margin_bottom(px(-15)).text_align_left(),
                &project.name
            ],
            p![
                s().margin("0").margin_bottom(px(-15)).text_align_left(),
                &project.description
            ],
            div![ui::form::InputBuilder::submit()
                .value("Start")
                .width(pc(100))
                .font_size(em(1.2))
                .view(model, move |_| None)]
        ],
    ]]
}
