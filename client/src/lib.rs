#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::future_not_send,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn
)]

pub mod endpoint;
pub mod routes;
pub mod state;
pub mod ui;
pub mod updates;

pub use {endpoint::Endpoint, routes::Route};

use {
    seed::{app::subs::UrlChanged, prelude::*},
    time::Duration,
    tracing::Level,
    tracing_subscriber::fmt::format::FmtSpan,
    wasm_bindgen::JsValue,
    web_sys::console::{debug_1, error_1, info_1, log_1, warn_1},
};

use std::{
    convert::TryInto,
    io::{self, Write},
};

/// Key where to store the login token on `LocalStorage`.
pub static LOGIN_KEY: &str = "Login";

/// Setup process invoked when client is started.
fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> state::Model {
    ui::style::global::init();

    orders
        .subscribe(routes::Route::update)
        // Always refresh token on load to keep token update.
        .send_msg(updates::Msg::RefreshToken)
        .stream(streams::interval(
            Duration::minutes(14)
                .whole_milliseconds()
                .try_into()
                .unwrap(),
            || updates::Msg::RefreshToken,
        ))
        .notify(UrlChanged(url));

    state::Model::new()
}

/// This function is invoked by `init` function from Javascript and is the entry
/// point of our program.
#[wasm_bindgen(start)]
pub fn start() {
    if cfg!(debug_assertions) {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_span_events(FmtSpan::CLOSE)
            .without_time()
            .with_ansi(false)
            .with_writer(ConsoleWrite::new)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("no global subscriber has been set");
    }
    let _app = App::start("app", init, updates::update, ui::view);
}

/// Writer to log to the JS console.
#[derive(Default)]
pub struct ConsoleWrite {
    buffer: Vec<u8>,
}

impl ConsoleWrite {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Drop for ConsoleWrite {
    /// Ensure that all logs are printed before dropping the writer.
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

impl Write for ConsoleWrite {
    /// Write bytes to a buffer and log them when a `\n` is found.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let to_write = buf.len();
        for byte in buf {
            match byte {
                b'\n' => self.flush()?,
                _ => self.buffer.push(*byte),
            }
        }
        Ok(to_write)
    }

    /// Write buffer to JS console.
    /// Writes to different console methods when the log starts with certain
    /// strings.
    fn flush(&mut self) -> io::Result<()> {
        fn core_log(logger: impl Fn(&JsValue), text: &str) {
            if !text.is_empty() {
                logger(&JsValue::from_str(text))
            }
        }
        #[cfg(test)]
        fn log(logger: impl Fn(&JsValue), text: &str) {
            core_log(log_1, text)
        }
        #[cfg(not(test))]
        fn log(logger: impl Fn(&JsValue), text: &str) {
            core_log(logger, text)
        }
        let to_log = std::str::from_utf8(&self.buffer).unwrap().trim_start();

        if to_log.starts_with("INFO") {
            log(info_1, to_log)
        } else if to_log.starts_with("WARN") {
            log(warn_1, to_log)
        } else if to_log.starts_with("ERROR") && cfg!(not(test)) {
            log(error_1, to_log)
        } else if to_log.starts_with("DEBUG") {
            log(debug_1, to_log)
        } else {
            log(log_1, to_log)
        }

        self.buffer = Vec::new();

        Ok(())
    }
}
