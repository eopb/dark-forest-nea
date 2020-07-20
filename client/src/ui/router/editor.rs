use crate::{state, ui, updates};
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
                .view(model, |x| Some(Msg::DescriptionChanged(x).into()))
        ]],
        Err(error) => div!["some error"],
    }
}
