mod terminal;

use crate::{
    draw::cursor,
    key::Key,
    panel::frame::Frame,
    panel::Panel,
    style::{
        self,
        color::{Color, ColorBG},
    },
    unit::Point,
    widget::Widget,
};
use std::{any::Any, time::Instant};
use terminal::{termsz, Terminal};

pub trait VarBufKey {
    fn index(&self) -> usize {
        0
    }
}

pub struct App {
    pub frame: Frame,
    alert_status: bool,
    term: Terminal,
    theme: crate::theme::Theme,
    // Properties
    pub show_cursor: bool,
    pub refresh_rate: usize,
    // Events
    pub init: fn(&mut Self),
    pub run: fn(&mut Self, Option<Key>) -> Option<usize>,
    pub end: fn(&mut Self),
    // App Variables (temporary solution for lack of globals in rust)
    var_buf: Vec<Box<dyn Any>>,
    var_key: Vec<String>,
}

pub(crate) fn get_tsz() -> (usize, usize) {
    termsz()
}

impl App {
    pub fn new() -> App {
        Self {
            frame: Frame::new(),
            alert_status: false,
            term: Terminal::initialize(),
            theme: crate::theme::Theme::new(),
            show_cursor: false,
            refresh_rate: 60,
            init: |_| {},
            run: |_, _| Some(0),
            end: |_| {},
            var_buf: Vec::new(),
            var_key: Vec::new(),
        }
    }

    pub fn fg(&self) -> &Color {
        &self.theme.fg
    }

    pub fn fg_alt(&self) -> &Color {
        &self.theme.fg_alt
    }

    pub fn fg_focus(&self) -> &Color {
        &self.theme.fg_focus
    }

    pub fn bg(&self) -> &ColorBG {
        &self.theme.bg
    }

    pub fn bg_alt(&self) -> &ColorBG {
        &self.theme.bg_alt
    }

    pub fn bg_focus(&self) -> &ColorBG {
        &self.theme.bg_focus
    }

    pub fn accent(&self) -> &Color {
        &self.theme.accent
    }

    pub fn accent_alt(&self) -> &Color {
        &self.theme.accent_alt
    }

    pub fn border(&self) -> &Color {
        &self.theme.border
    }

    pub fn border_alt(&self) -> &Color {
        &self.theme.border_alt
    }

    pub fn red(&self) -> &Color {
        &self.theme.red
    }

    pub fn green(&self) -> &Color {
        &self.theme.green
    }

    pub fn yellow(&self) -> &Color {
        &self.theme.yellow
    }

    pub fn blue(&self) -> &Color {
        &self.theme.blue
    }

    pub fn magenta(&self) -> &Color {
        &self.theme.magenta
    }

    pub fn cyan(&self) -> &Color {
        &self.theme.cyan
    }

    pub fn white(&self) -> &Color {
        &self.theme.white
    }

    pub fn start(&mut self) {
        printlnf!("\x1b[?47h");

        self.term.make_raw();
        terminal::clear();
        cursor::home();

        if !self.show_cursor {
            cursor::set_shape(cursor::CursorShape::None);
        }

        let mut dim: (usize, usize) = (0, 0);

        (self.init)(self);

        loop {
            let time = Instant::now();

            if dim != termsz() {
                dim = termsz();
                // resize everything
            }

            terminal::clear();
            cursor::home();

            if let Some(c) = terminal::getch() {
                if let Some(key) = Key::new(c) {
                    if let Some(n) = (self.run)(self, Some(key)) {
                        if n != 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else if let Some(n) = (self.run)(self, None) {
                if n != 0 {
                    break;
                }
            } else {
                break;
            }
            self.frame.render();

            style::reset();

            let target_time = std::time::Duration::from_millis(1000 / self.refresh_rate as u64);
            if time.elapsed() < target_time {
                std::thread::sleep(target_time - time.elapsed());
            }
        }

        (self.end)(self);

        printlnf!("\x1b[?47l\x1b[?25h");
    }

    pub fn new_var(&mut self, name: &'static str, value: impl Any) {
        self.var_key.push(String::from(name));
        self.var_buf.push(Box::new(value));
    }

    pub fn del_var(&mut self, name: &'static str) {
        if let Ok(i) = self.var_key.binary_search(&String::from(name)) {
            self.var_key.remove(i);
            self.var_buf.remove(i);
        }
    }

    pub fn get_var<T: 'static>(&mut self, name: &'static str) -> Option<&T> {
        match self.var_key.binary_search(&String::from(name)) {
            Ok(index) => self.var_buf.get(index).unwrap().downcast_ref::<T>(),
            Err(_) => None,
        }
    }

    pub fn get_var_mut<T: 'static>(&mut self, name: &'static str) -> Option<&mut T> {
        match self.var_key.binary_search(&String::from(name)) {
            Ok(index) => self.var_buf.get_mut(index).unwrap().downcast_mut::<T>(),
            Err(_) => None,
        }
    }

    pub fn get_widget(&mut self, tag: &str) -> Option<&mut Box<dyn Widget>> {
        self.frame.get_widget(tag)
    }
}
