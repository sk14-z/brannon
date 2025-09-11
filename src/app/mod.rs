pub mod cache;
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
    theme::Theme,
    unit::Point,
    widget::Widget,
};

use cache::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    time::Instant,
};
use terminal::{termsz, Terminal};

pub struct App {
    pub frame: Frame,
    alert_status: bool,
    term: Terminal,
    pub theme: Theme,
    // Properties
    pub show_cursor: bool,
    pub refresh_rate: usize,
    // Events
    pub init: fn(&mut Self),
    pub run: fn(&mut Self, Option<Key>) -> Option<usize>,
    pub end: fn(&mut Self),
    // Caches
    caches: HashMap<TypeId, Box<dyn Any>>,
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
            theme: Theme::new(),
            show_cursor: false,
            refresh_rate: 60,
            init: |_| {},
            run: |_, _| Some(0),
            end: |_| {},
            caches: HashMap::new(),
        }
    }

    pub fn fg(&self) -> Color {
        self.theme.fg
    }

    pub fn fg_alt(&self) -> Color {
        self.theme.fg_alt
    }

    pub fn fg_focus(&self) -> Color {
        self.theme.fg_focus
    }

    pub fn bg(&self) -> ColorBG {
        self.theme.bg
    }

    pub fn bg_alt(&self) -> ColorBG {
        self.theme.bg_alt
    }

    pub fn bg_focus(&self) -> ColorBG {
        self.theme.bg_focus
    }

    pub fn accent(&self) -> Color {
        self.theme.accent
    }

    pub fn accent_alt(&self) -> Color {
        self.theme.accent_alt
    }

    pub fn border(&self) -> Color {
        self.theme.border
    }

    pub fn border_alt(&self) -> Color {
        self.theme.border_alt
    }

    pub fn red(&self) -> Color {
        self.theme.red
    }

    pub fn green(&self) -> Color {
        self.theme.green
    }

    pub fn yellow(&self) -> Color {
        self.theme.yellow
    }

    pub fn blue(&self) -> Color {
        self.theme.blue
    }

    pub fn magenta(&self) -> Color {
        self.theme.magenta
    }

    pub fn cyan(&self) -> Color {
        self.theme.cyan
    }

    pub fn white(&self) -> Color {
        self.theme.white
    }

    pub fn start(&mut self) {
        Terminal::save();

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
                // resize everything
                dim = termsz();
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

        Terminal::restore();
    }

    pub fn cache<T: 'static>(&mut self) -> &mut AppCache<T> {
        let id = TypeId::of::<T>();

        self.caches
            .entry(id)
            .or_insert(Box::new(AppCache::<T>::new()))
            .downcast_mut::<AppCache<T>>()
            .unwrap()
    }

    pub fn get_widget(&mut self, tag: &str) -> Option<&mut Box<dyn Widget>> {
        self.frame.get_widget(tag)
    }
}
