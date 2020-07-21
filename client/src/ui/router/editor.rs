use crate::{
    state,
    ui::{self, view::View},
    updates,
};
use seed::{prelude::*, *};
use seed_style::{em, pc, px, *};

pub enum Msg {
    DescriptionChanged(String),
    NameChanged(String),
}

impl Msg {
    pub fn update(self, model: &mut state::Model, orders: &mut impl Orders<updates::Msg>) {
        let mut inner_model = model.route_data.editor.as_mut().unwrap();
        match self {
            Self::DescriptionChanged(description) => inner_model.description = description,
            Self::NameChanged(name) => inner_model.name = name,
        }
    }
}
impl From<Msg> for updates::Msg {
    fn from(msg: Msg) -> Self {
        Self::Editor(msg)
    }
}

#[allow(clippy::too_many_lines)]
pub fn view(model: &state::Model) -> Node<updates::Msg> {
    log!(model.route_data.editor);
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
                    .view(model, |_| None)
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
                    .map(|(key, chapter)| div![
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
                                .view(model, |_| None),
                        ]],
                        ui::form::InputBuilder::text_area()
                            .value(&chapter.body)
                            .width(pc(100))
                            .view(model, |x| None),
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
                                    .map(|decision| div![
                                        s().padding_left(px(8)).padding_right(px(8)),
                                        vec![div![
                                            s().display_grid()
                                                .grid_template_columns("150px auto")
                                                .grid_gap(px(8))
                                                .width(pc(100)),
                                            if let Some(shared::data::Link::Chapter(goes_to)) =
                                                decision.goes_to
                                            {
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
                                    ])
                                    .collect::<Vec<Node<updates::Msg>>>()
                            )
                            .inner(s().width(pc(100)))
                            .outer(s().padding("0"))
                            .view(model),
                            ui::form::InputBuilder::submit()
                                .value("Add decision")
                                .view(model, |_| None)
                        ]
                    ])
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
