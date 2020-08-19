//! Server handled user facing endpoints.

use super::State;

use {
    http_types::mime,
    maud::{html, PreEscaped, DOCTYPE},
    tide::{Request, Response, StatusCode},
    tracing::{instrument, trace},
};use horrorshow::prelude::*;
use horrorshow::html;
use horrorshow::helper::doctype;


use shared::routes::SubRoute;

/// This index endpoint returns the `html` needed to start our client-side
/// application.
#[instrument(level = "trace")]
pub async fn index(_: Request<State>) -> tide::Result<impl Into<Response>> {
    trace!("Rendering index page",);
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
                    title { (shared::Route::Index.title()) }
                    style {
                        (PreEscaped(r#"
                            @font-face {
                                font-family: bitlimt;
                                src: url("/fonts/8-bit-limit/8bitlim.ttf");
                                font-weight: bold;
                            }
                            @font-face {
                                font-family: adobedia;
                                src: url("/fonts/levi-adobe-dia/adobedia.ttf");
                                font-weight: bold;
                            }
                            @font-face {
                                font-family: prstart;
                                src: url("/fonts/press-start/prstart.ttf");
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
