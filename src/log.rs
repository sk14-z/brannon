use crate::style::{color::Color, text::TextStyle, PrintableStyle};
use std::{fmt::Display, io::Write};
use time::{format_description, OffsetDateTime};

// You can use less or cat to view a log file with color

pub enum LogFlag {
    None,
    Info,
    Debug,
    Warning,
    Error,
}

impl LogFlag {
    fn format(&self) -> String {
        match self {
            LogFlag::None => String::from("-"),
            LogFlag::Info => format!("{}info:", Color::Green.print()),
            LogFlag::Debug => format!("{}debug:", Color::Blue.print()),
            LogFlag::Warning => format!("{}warning:", Color::Yellow.print(),),
            LogFlag::Error => format!("{}error:", Color::Red.print()),
        }
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

    pub fn file(path: &str) -> Option<Logger> {
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
            flag.format(),
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
