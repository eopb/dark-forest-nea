//! Application endpoints.

use maud::{html, PreEscaped, DOCTYPE};

use super::State;

use {
    http_types::mime,
    tide::{Request, Response, StatusCode},
};

pub async fn index(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok({
        let mut res = Response::new(StatusCode::Ok);
        res.set_content_type(mime::HTML);
        res.set_body(
            html! {
                (DOCTYPE)
                head {
                    meta charset="utf-8";
                    meta
                        name="viewport"
                        content="width=device-width, initial-scale=1, shrink-to-fit=no";
                    style {
                        (PreEscaped(r#"
                            @font-face {
                                font-family: bitlimt;
                                src: url("fonts/8-bit-limit/8bitlim.ttf");
                                font-weight: bold;
                            }
                            @font-face {
                                font-family: adobedia;
                                src: url("fonts/levi-adobe-dia/adobedia.ttf");
                                font-weight: bold;
                            }
                            @font-face {
                                font-family: prstart;
                                src: url("fonts/press-start/prstart.ttf");
                                font-weight: bold;
                            }
                        "#))
                    }
                }
                body {
                    section id="app" {}
                    script type="module" {
                        (PreEscaped(r#"
                            import init from "/pkg/package.js";
                            init("/pkg/package_bg.wasm");
                        "#))
                    }
                }
            }
            .into_string(),
        );
        res
    })
}
