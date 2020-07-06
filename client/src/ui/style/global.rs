use seed_style::*;

/// Styles to be applied when client starts.
pub fn init() {
    GlobalStyle::default()
        .style("img", s().box_sizing_content_box())
        .style(
            "body *, body *:before, body *:after",
            s().box_sizing("inherit"),
        )
        .style(
            "body",
            s().box_sizing_border_box()
                .margin(px(0))
                .padding(px(0))
                .font_family("adobedia, Avenir, Helvetica, Arial, sans-serif"),
        )
        .activate_init_styles();
}
