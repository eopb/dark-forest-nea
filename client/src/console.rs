//! Utils for working with the JS console.

use std::io::{self, Write as _};

use {
    seed::prelude::*,
    wasm_bindgen::JsValue,
    web_sys::console::{debug_1, error_1, info_1, log_1, warn_1},
};
/// Writer to log to the JS console.
#[derive(Default)]
pub struct Write {
    buffer: Vec<u8>,
}

impl Write {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Drop for Write {
    /// Ensure that all logs are printed before dropping the writer.
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

impl io::Write for Write {
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
        fn log(_: impl Fn(&JsValue), text: &str) {
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
