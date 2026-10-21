//! The `js` lines engines log (the app's `console` and its errors),
//! captured: the logger is the process's, shared by every run, so a run
//! reads the lines of its own engine's thread ([`printed_by`]).

use std::{sync::Mutex, thread::ThreadId};

/// What was logged: the thread that logged it, the line.
static LINES: Mutex<Vec<(ThreadId, String)>> = Mutex::new(Vec::new());

struct Capture;

impl log::Log for Capture {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        // `console.debug` is `Debug`: every line an app prints.
        metadata.target() == "js" && metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            LINES
                .lock()
                .unwrap()
                .push((std::thread::current().id(), record.args().to_string()));
        }
    }

    fn flush(&self) {}
}

static CAPTURE: Capture = Capture;

/// Captures from here on (installed once).
pub(crate) fn capture() {
    static INSTALL: std::sync::Once = std::sync::Once::new();
    INSTALL.call_once(|| {
        if log::set_logger(&CAPTURE).is_ok() {
            log::set_max_level(log::LevelFilter::Debug);
        }
    });
}

/// The lines of every level `thread` logged so far.
pub(crate) fn printed_by(thread: ThreadId) -> Vec<String> {
    LINES
        .lock()
        .unwrap()
        .iter()
        .filter(|(t, _)| *t == thread)
        .map(|(_, l)| l.clone())
        .collect()
}
