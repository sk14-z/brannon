use crate::style::{PrintableStyle, color::Color, text::TextStyle};
use std::{fmt::Display, io::Write, path::Path};
use time::{OffsetDateTime, format_description};

// You can use less or cat to view a log file with color

pub enum LogFlag {
    None,
    Info,
    Debug,
    Warning,
    Error,
}

impl Display for LogFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogFlag::None => String::from("-"),
                LogFlag::Info => format!("{}info:", Color::Green.print()),
                LogFlag::Debug => format!("{}debug:", Color::Blue.print()),
                LogFlag::Warning => format!("{}warning:", Color::Yellow.print(),),
                LogFlag::Error => format!("{}error:", Color::Red.print()),
            }
        )
    }
}

pub struct Logger {
    stream: Box<dyn Write>,
}

impl Logger {
    pub fn new(stream: impl Write + 'static) -> Self {
        Logger {
            stream: Box::new(stream),
        }
    }

    pub fn file(path: impl AsRef<Path>) -> Option<Logger> {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            Ok(file) => Some(Logger::new(file)),
            Err(_) => None,
        }
    }

    pub fn log(&mut self, flag: LogFlag, msg: impl Display) {
        _ = writeln!(
            self.stream,
            "{}[{}] {}\x1b[0m {}",
            TextStyle::Bold.print(),
            OffsetDateTime::now_local()
                .unwrap()
                .format(&format_description::parse("[hour]:[minute]:[second]").unwrap())
                .unwrap(),
            flag,
            msg
        );
    }

    pub fn log_raw(&mut self, msg: impl Display) {
        self.log(LogFlag::None, msg);
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        _ = writeln!(self.stream, "\n----------END OF LOG----------\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    #[test]
    fn logflag_display_basic() {
        assert_eq!(format!("{}", LogFlag::None), "-");
        let info = format!("{}", LogFlag::Info);
        assert!(info.contains("info:"), "expected info: in {info}");
        let debug = format!("{}", LogFlag::Debug);
        assert!(debug.contains("debug:"), "expected debug: in {debug}");
        let warn = format!("{}", LogFlag::Warning);
        assert!(warn.contains("warning:"), "expected warning: in {warn}");
        let err = format!("{}", LogFlag::Error);
        assert!(err.contains("error:"), "expected error: in {err}");
    }

    struct SharedBuf(Arc<Mutex<Vec<u8>>>);

    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn logger_writes_and_drops() {
        let backing = Arc::new(Mutex::new(Vec::<u8>::new()));
        {
            let mut logger = Logger::new(SharedBuf(backing.clone()));
            logger.log(LogFlag::Info, "Hello");
            logger.log_raw("World");
        } // drop executes

        let out = String::from_utf8(backing.lock().unwrap().clone()).unwrap();

        // Two log lines with timestamps and final END marker
        assert!(out.contains("info:"), "info flag missing: {out}");
        assert!(out.contains("Hello"), "message Hello missing: {out}");
        assert!(out.contains("World"), "raw message World missing: {out}");
        assert!(out.contains("END OF LOG"), "missing end marker: {out}");

        // At least two timestamps like [HH:MM:SS]
        let bracket_pairs = out.matches('[').count().min(out.matches(']').count());
        assert!(
            bracket_pairs >= 2,
            "expected at least two timestamp brackets, got {bracket_pairs} in {out}"
        );

        // Contains ANSI reset
        assert!(out.contains("\x1b[0m"), "expected ANSI reset sequence");
    }
}
