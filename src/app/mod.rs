mod cache;
pub mod option;
pub(crate) mod terminal;

use crate::{
    draw::cursor,
    input::Input,
    panel::{Panel, frame::Frame},
    scene::{DefaultScene, SceneHandler, SceneKey, SceneKeyT},
    style::{self, PrintableStyle, set_style},
    widget::{Widget, attr::Attr},
};
use cache::*;
use libc::{SIGINT, sighandler_t, signal};
use option::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    time::Instant,
};
use terminal::{Terminal, termsz};

pub(crate) fn get_tsz() -> (usize, usize) {
    termsz()
}

extern "C" fn handle_sigint(_: i32) {}

pub struct App {
    term: Terminal,
    // Options
    pub opts: AppOptions,
    // Scenes
    pub scenes: SceneHandler,
    // App lifecycle delegates
    pub init: fn(&mut Self),
    pub run: fn(&mut Self, Option<Input>) -> Option<usize>,
    pub end: fn(&mut Self),
    // Caches
    caches: HashMap<TypeId, Box<dyn Any>>,
}

impl App {
    pub fn new() -> App {
        let mut scenes = SceneHandler::new();
        scenes.add(DefaultScene, Frame::new(None));

        Self {
            term: Terminal::initialize(),
            scenes,
            opts: AppOptions::new(),
            init: |_| {},
            run: |_, _| Some(0),
            end: |_| {},
            caches: HashMap::new(),
        }
    }

    pub fn start(mut self) {
        (self.init)(&mut self);

        if self.opts.no_interrupt {
            unsafe {
                signal(SIGINT, handle_sigint as sighandler_t);
            }
        }

        self.opts.key_protocol.activate();
        // crate::printf!("\x1b[={};1u", 0b1111);

        // temp
        // send mouse events: press, release, move with button, scroll
        crate::printf!("\x1b[?1002h");
        // mouse reporting format
        crate::printf!("\x1b[?1006h");

        cursor::hide();

        let mut dim: (usize, usize) = (0, 0);

        loop {
            let time = Instant::now();

            if dim != termsz() {
                dim = termsz();
            }

            if !self.run_until_i_can_code() {
                break;
            }

            // if let Some(n) = (self.run)(&mut self, terminal::poll_input()) {
            //     if n != 0 {
            //         break;
            //     }
            // } else {
            //     (self.end)(&mut self);
            //
            //     drop(self.term);
            //
            //     crate::printf!(
            //         "\n{}App terminated without exit code\n",
            //         crate::style::color::Color::Red.print()
            //     );
            //
            //     style::reset();
            //
            //     return;
            // }

            set_style(self.current_frame().attr.fill);

            terminal::clear();
            cursor::home();

            self.current_frame().render();

            style::reset();

            let target_time =
                std::time::Duration::from_millis(1000 / self.opts.refresh_rate as u64);

            if time.elapsed() < target_time {
                std::thread::sleep(target_time - time.elapsed());
            }
        }

        (self.end)(&mut self);
    }

    pub fn run_until_i_can_code(&mut self) -> bool {
        let inputs = terminal::poll_until_i_can_code();

        for input in inputs {
            if let Some(n) = (self.run)(self, Some(input)) {
                if n != 0 {
                    return false;
                }
            } else {
                // (self.end)(&mut self);

                // drop(self.term);

                // crate::printf!(
                //     "\n{}App terminated without exit code\n",
                //     crate::style::color::Color::Red.print()
                // );

                // style::reset();

                return false;
            }
        }

        if let Some(n) = (self.run)(self, None) {
            return n == 0;
        }
        // else {
        // (self.end)(&mut self);

        // drop(self.term);

        // crate::printf!(
        //     "\n{}App terminated without exit code\n",
        //     crate::style::color::Color::Red.print()
        // );

        // style::reset();
        // }

        true
    }

    pub fn cache<T: 'static>(&mut self) -> &mut Cache<T> {
        let id = TypeId::of::<T>();

        self.caches
            .entry(id)
            .or_insert(Box::new(Cache::<T>::new()))
            .downcast_mut::<Cache<T>>()
            .unwrap()
    }

    pub fn current_frame(&mut self) -> &mut Frame {
        &mut self.scenes.current().frame
    }

    pub fn current_scene_key<T: SceneKey>(&mut self) -> Option<&T> {
        self.scenes.current().key.as_any().downcast_ref::<T>()
    }

    pub fn get_widget<T: Widget>(&mut self, tag: &str) -> Option<&mut T> {
        // self.has_changed = true;

        if let Some(widget) = self.current_frame().get_child(tag)
            && let Some(widget_as) = widget.as_any_mut().downcast_mut::<T>()
        {
            Some(widget_as)
        } else {
            None
        }
    }

    pub fn hide_widget(&mut self, tag: &str) {
        // self.has_changed = true;

        let (_, children) = self.current_frame().split_mut();

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
        // self.has_changed = true;

        let (_, children) = self.current_frame().split_mut();

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
        // self.has_changed = true;

        let (_, children) = self.current_frame().split_mut();

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
        // self.has_changed = true;

        self.current_frame().map_all(map);
    }

    pub fn style_all(&mut self, map: fn(&mut Attr)) {
        // self.has_changed = true;

        self.current_frame().style_all(map);
    }
}
