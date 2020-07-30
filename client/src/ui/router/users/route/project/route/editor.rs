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

#[derive(Debug)]
pub enum Msg {
    DescriptionChanged(String),
    NameChanged(String),
    Chapter(ChapterMsgWrapper),
    Submit(ProjectPath),
    Submited(Result<(), PermissionDenied>),
    SubmitFailed(String),
}

impl Msg {
    #[instrument(skip(model, orders))]
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = model.route_data.editor.as_mut().unwrap();
        match self {
            Self::DescriptionChanged(description) => inner_model.description = description,
            Self::NameChanged(name) => inner_model.name = name,
            Self::Chapter(msg) => msg.update(inner_model, orders),
            Self::Submit(project_path) => {
                orders.skip(); // No need to rerender
                shadow_clone!(inner_model);
                let login_token = model.login_token.clone();
                orders.perform_cmd(async move {
                    updates::Msg::from({
                        if let Some(login_token) = login_token {
                            if let Ok(response) = SaveEditor::fetch(Authenticated::new(
                                (project_path, inner_model),
                                login_token,
                            ))
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
            Self::Submited(result) => {}
            Self::SubmitFailed(reason) => error!(reason),
        }
    }
}
impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::Editor(msg)
    }
}

#[derive(Debug)]
pub struct ChapterMsgWrapper {
    key: i64,
    msg: ChapterMsg,
}
impl ChapterMsgWrapper {
    fn new(key: i64, msg: ChapterMsg) -> Self {
        Self { key, msg }
    }
}

impl From<ChapterMsgWrapper> for updates::Msg {
    fn from(msg: ChapterMsgWrapper) -> Self {
        Self::Editor(Msg::Chapter(msg))
    }
}
impl ChapterMsgWrapper {
    #[instrument(skip(inner_model, orders))]
    pub fn update(self, inner_model: &mut Project, orders: &mut impl Orders<updates::Msg>) {
        let chapter = inner_model.chapters.get_mut(&self.key);
        if let Some(chapter) = chapter {
            self.msg.update(chapter, orders)
        } else {
            error!("Attempt to edit chapter that does not exist");
        }
    }
}

#[derive(Debug)]
enum ChapterMsg {
    NameChanged(String),
    BodyChanged(String),
}

impl ChapterMsg {
    #[instrument(skip(orders))]
    pub fn update(self, chapter: &mut Chapter, orders: &mut impl Orders<updates::Msg>) {
        match self {
            Self::NameChanged(name) => chapter.heading = name,
            Self::BodyChanged(body) => chapter.body = body,
        }
    }
}
#[instrument(skip(model))]
pub fn view(model: &state::Model, project_path: ProjectPath) -> Node<updates::Msg> {
    info!("rendering project");
    trace!(project = format!("{:#?}", model.route_data.editor).as_str());
    match &model.route_data.editor {
        Ok(project) => div![div![
            s().display("flex")
                .align_items("center")
                .flex_direction("column")
                .margin("auto"),
            div![
                s().display_grid()
                    .grid_template_columns("auto auto")
                    .grid_gap(px(8))
                    .width(px(600)),
                ui::form::InputBuilder::text()
                    .value(&project.name)
                    .width(pc(100))
                    .view(model, |x| Some(Msg::NameChanged(x).into())),
                ui::form::InputBuilder::submit()
                    .value("Save")
                    .width(pc(100))
                    .view(model, move |_| Some(
                        Msg::Submit(project_path.clone()).into()
                    ))
            ],
            ui::form::InputBuilder::text_area()
                .value(&project.description)
                .label("Description")
                .view(model, |x| Some(Msg::DescriptionChanged(x).into())),
            label![
                s().margin("0")
                    .margin_bottom(px(-15))
                    .width(px(600))
                    .text_align_left()
                    .font_size(em(2.9))
                    .color(model.theme.text()),
                "Chapters"
            ],
            ui::Bordered::new(
                vec![project
                    .chapters
                    .iter()
                    .map(|(key, chapter)| (*key, chapter))
                    .map(chapters(model))
                    .collect::<Vec<Node<updates::Msg>>>(),]
                .into_iter()
                .flatten()
                .collect::<Vec<Node<updates::Msg>>>()
            )
            .inner(s().width(px(600)))
            .view(model),
            ui::form::InputBuilder::submit()
                .value("Add chapter")
                .view(model, |_| None)
        ]],
        Err(error) => div!["some error"],
    }
}
// TODO use type alias for `Node<updates::Msg>>`

#[instrument(skip(model))]
pub fn chapters<'a>(
    model: &'a state::Model,
) -> impl Fn((i64, &Chapter)) -> Node<updates::Msg> + 'a {
    fn chapter_event<'a>(
        func: &'a (dyn Fn(String) -> ChapterMsg + 'a),
        key: i64,
    ) -> impl Fn(String) -> Option<updates::Msg> + 'a + Clone {
        move |s| Some(ChapterMsgWrapper::new(key, func(s)).into())
    }

    move |(key, chapter)| {
        let chapter_event = |func| chapter_event(func, key);
        div![
            s().padding_left(px(8)).padding_right(px(8)),
            vec![div![
                s().display_grid()
                    .grid_template_columns("70px auto")
                    .grid_gap(px(8))
                    .width(pc(100)),
                ui::form::InputBuilder::text()
                    .value(&key)
                    .width(pc(100))
                    .view(model, |_| None),
                ui::form::InputBuilder::text()
                    .value(&chapter.heading)
                    .width(pc(100))
                    .view(model, chapter_event(&ChapterMsg::NameChanged))
            ]],
            ui::form::InputBuilder::text_area()
                .value(&chapter.body)
                .width(pc(100))
                .view(model, chapter_event(&ChapterMsg::BodyChanged)),
            vec![
                vec![label![
                    s().margin("0")
                        .margin_bottom(px(-15))
                        .width(px(600))
                        .text_align_left()
                        .font_size(em(2.9))
                        .color(model.theme.text()),
                    "Decisions"
                ]],
                ui::Bordered::new(
                    chapter
                        .decisions
                        .iter()
                        .map(decisions(model))
                        .collect::<Vec<Node<updates::Msg>>>()
                )
                .inner(s().width(pc(100)))
                .outer(s().padding("0"))
                .view(model),
                ui::form::InputBuilder::submit()
                    .value("Add decision")
                    .view(model, |_| None)
            ]
        ]
    }
}
#[instrument(skip(model))]
pub fn decisions<'a>(model: &'a state::Model) -> impl Fn(&Decision) -> Node<updates::Msg> + 'a {
    move |decision| {
        div![
            s().padding_left(px(8)).padding_right(px(8)),
            vec![div![
                s().display_grid()
                    .grid_template_columns("150px auto")
                    .grid_gap(px(8))
                    .width(pc(100)),
                if let Some(shared::data::Link::Chapter(goes_to)) = decision.goes_to {
                    nodes![
                        p![
                            s().font_size(em(2.9))
                                .margin("0")
                                .margin_bottom("auto")
                                .margin_top("auto"),
                            "goes_to"
                        ],
                        ui::form::InputBuilder::text()
                            .value(&goes_to)
                            .width(pc(100))
                            .view(model, |_| None),
                    ]
                } else {
                    vec![empty()]
                }
            ]],
            ui::form::InputBuilder::submit()
                .value(&decision.body)
                .width(pc(100))
                .view(model, |x| None),
        ]
    }
}
