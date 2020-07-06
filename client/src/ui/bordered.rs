//! Way of placing things in cool borders.

use crate::{state, ui, updates};

use {
    seed::{prelude::*, *},
    seed_style::{pc, px, CssBorder::Border, Style, *},
};

/// Builder for creating the boarders.
pub struct Bordered<Nodes> {
    /// Styles for the outer border `div`
    outer: Style,
    /// Styles for the inner border `div`
    inner: Style,
    /// Items to place in the border
    nodes: Nodes,
}

impl<Nodes> Bordered<Nodes> {
    pub fn new(nodes: Nodes) -> Self {
        Self {
            outer: s(),
            inner: s(),
            nodes,
        }
    }
}

impl<Nodes> Bordered<Nodes> {
    /// Set styles for the outer border `div`
    pub fn outer(mut self, outer: Style) -> Self {
        self.outer = outer;
        self
    }
    /// Set styles for the inner border `div`
    pub fn inner(mut self, inner: Style) -> Self {
        self.inner = inner;
        self
    }
}

impl<Nodes> ui::View<updates::Msg, state::Model> for Bordered<Nodes>
where
    Nodes: IntoNodes<updates::Msg> + Clone,
{
    fn view(&self, model: &state::Model) -> Vec<Node<updates::Msg>> {
        div![
            s().background_color(model.theme.background())
                .color(model.theme.text())
                .text_align_center()
                .padding(px(8))
                .display_flex()
                .flex_direction_column(),
            self.outer.clone(),
            div![
                s().flex("1")
                    .padding(px(0))
                    .min_height(pc(100))
                    // TODO fixme when https://github.com/rebo/seed_style_preview/issues/6
                    // is fixed
                    //// .border(Border(
                    ////     px(6).into(),
                    ////     CssBorderStyle::Solid,
                    ////     model.theme.text().into()
                    //// ))
                    .border(AsRef::<str>::as_ref(&format!(
                        "6px solid {}",
                        model.theme.text()
                    )))
                    .position_relative(),
                self.inner.clone(),
                ui::style::pixel_cutouts(model),
                self.nodes.clone().into_nodes()
            ],
        ]
        .into_nodes()
    }
}
