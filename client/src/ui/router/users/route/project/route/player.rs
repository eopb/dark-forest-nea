use crate::{
    endpoint::Post,
    state,
    ui::{self, view::View},
    updates,
};

use shared::{
    data::{
        chapters::{Chapter, Decision, Link},
        Project,
    },
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
    position: Position,
}

impl From<Link> for Position {
    fn from(link: Link) -> Self {
        match link {
            Link::Chapter(c) => Self::Chapter(c),
            Link::End => Self::End,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Position {
    Start,
    /// Chapter with key.
    Chapter(u32),
    End,
}

impl Position {
    fn first_chapter() -> Self {
        Self::Chapter(1)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    ChangePosition(Position),
}

impl Msg {
    #[instrument(skip(model, orders))]
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = &mut model.route_data.player_state;
        match self {
            Self::ChangePosition(p) => inner_model.position = p,
        }
    }
}

impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::Player(msg)
    }
}

#[instrument(skip(model))]
pub fn view(model: &state::Model, project_path: ProjectPath) -> Node<updates::Msg> {
    let project = &model.route_data.project;
    let state = &model.route_data.player_state;
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
            match state.position {
                Position::Start => div![
                    p![
                        s().margin("0").margin_bottom(px(-15)).text_align_left(),
                        &project.description
                    ],
                    div![ui::form::InputBuilder::submit()
                        .value("Start")
                        .width(pc(100))
                        .font_size(em(1.2))
                        .view(model, move |_| Some(
                            Msg::ChangePosition(Position::first_chapter()).into()
                        ))]
                ],
                Position::Chapter(key) => {
                    let chapter = project
                        .chapters
                        .get_chapter_with_key(key)
                        .unwrap_or_else(|| {
                            panic!("Invalid chapter setup: No chapter at key: {}", key)
                        });
                    div![
                        h3![chapter.heading.to_owned()],
                        p![chapter.body.to_owned()],
                        chapter
                            .decisions
                            .iter()
                            .flat_map(decisions(model))
                            .collect::<Vec<Node<updates::Msg>>>()
                    ]
                }
                Position::End => p!["Thank you for playing"],
            }
        ],
    ]]
}

#[instrument(skip(model))]
pub fn decisions<'a>(
    model: &'a state::Model,
) -> impl Fn(&'a Decision) -> Vec<Node<updates::Msg>> + 'a {
    move |decision| {
        let msg = Msg::ChangePosition(decision.goes_to.clone().unwrap().into());
        ui::form::InputBuilder::submit()
            .value(decision.body.to_owned())
            .width(pc(100))
            .font_size(em(1.2))
            .view(model, move |_| Some(msg.clone().into()))
    }
}
