mod cache;
mod terminal;

use crate::{
    draw::cursor,
    key::Key,
    panel::{Panel, frame::Frame},
    scene::{DefaultScene, SceneHandler, SceneKey, SceneKeyT},
    style::{
        self,
        color::{Color, ColorBG},
        set_style,
    },
    theme::Theme,
    widget::{Widget, attr::Attr},
};
use cache::*;
use libc::{SIGINT, sighandler_t, signal};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    time::Instant,
};
use terminal::{Terminal, termsz};

extern "C" fn handle_sigint(_: i32) {}

pub struct App {
    scenes: SceneHandler,
    term: Terminal,
    has_changed: bool,
    // Properties
    pub theme: Theme,
    pub show_cursor: bool,
    pub no_interrupt: bool,
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
        let mut scenes = SceneHandler::new();
        scenes.create_scene(DefaultScene, Frame::new(None));

        Self {
            scenes,
            term: Terminal::initialize(),
            has_changed: true,
            theme: Theme::new(),
            show_cursor: false,
            no_interrupt: true,
            refresh_rate: 30,
            init: |_| {},
            run: |_, _| Some(0),
            end: |_| {},
            caches: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        if self.no_interrupt {
            unsafe {
                signal(SIGINT, handle_sigint as sighandler_t);
            }
        }

        Terminal::save();

        self.term.make_raw();

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
                // self.has_changed = true;
            }

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

            // if self.has_changed {
            set_style(self.frame().attr.fill);

            terminal::clear();
            cursor::home();

            self.frame().render();
            self.has_changed = false;
            // }

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

    pub fn create_scene<T: SceneKeyT>(&mut self, key: T, frame: Frame) {
        self.scenes.create_scene(key, frame);
    }

    pub fn remove_scene<T: SceneKeyT>(&mut self, key: T) -> Option<Frame> {
        if let Some(scene) = self.scenes.remove_scene(key) {
            Some(scene.frame)
        } else {
            None
        }
    }

    pub fn change_scene<T: SceneKeyT>(&mut self, key: &mut T) {
        self.scenes.change_scene(key);
        self.has_changed = true;
    }

    pub fn frame(&mut self) -> &mut Frame {
        &mut self.scenes.current().frame
    }

    pub fn current_scene_key<T: SceneKey>(&mut self) -> Option<&T> {
        self.scenes.current().key.as_any().downcast_ref::<T>()
    }

    pub fn get_widget<T>(&mut self, tag: &str) -> Option<&mut T>
    where
        T: Widget + 'static,
    {
        self.has_changed = true;

        if let Some(widget) = self.frame().get_child(tag) {
            if let Some(widget) = widget.as_any_mut().downcast_mut::<T>() {
                Some(widget)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn hide_widget(&mut self, tag: &str) {
        self.has_changed = true;

        let (_, children) = self.frame().split_mut();

        for child in children.iter_mut() {
            if child.style().tag == tag {
                child.style_mut().hide = true;
            } else if let Some(panel) = child.as_panel()
                && let Some(widget) = panel.get_child(tag)
            {
                widget.style_mut().hide = true;
                panel.flex();
            }
        }
    }

    pub fn show_widget(&mut self, tag: &str) {
        self.has_changed = true;

        let (_, children) = self.frame().split_mut();

        for child in children.iter_mut() {
            if child.style().tag == tag {
                child.style_mut().hide = false;
            } else if let Some(panel) = child.as_panel()
                && let Some(widget) = panel.get_child(tag)
            {
                widget.style_mut().hide = false;
                panel.flex();
            }
        }
    }

    pub fn toggle_visiblity_of(&mut self, tag: &str) {
        self.has_changed = true;

        let (_, children) = self.frame().split_mut();

        for child in children.iter_mut() {
            if child.style().tag == tag {
                child.style_mut().hide = !child.style().hide;
            } else if let Some(panel) = child.as_panel()
                && let Some(widget) = panel.get_child(tag)
            {
                widget.style_mut().hide = !widget.style().hide;
                panel.flex();
            }
        }
    }

    pub fn map_all(&mut self, map: fn(&mut Box<dyn Widget>)) {
        self.has_changed = true;

        self.frame().map_all(map);
    }

    pub fn style_all(&mut self, map: fn(&mut Attr)) {
        self.has_changed = true;

        self.frame().style_all(map);
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
}
