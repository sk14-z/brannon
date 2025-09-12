use super::{attr::Attr, Widget};
use crate::{
    draw::cursor,
    style::{self, color::ColorBG, set_style},
    unit::{Point, Unit},
    widget_shared,
};
use std::any::Any;

pub struct ProgressBar {
    pub attr: Attr,
    progress: usize,
    bar_color: ColorBG,
}

impl Widget for ProgressBar {
    widget_shared!();

    fn render(&self, anchor: Point) {
        set_style(self.bar_color);

        let mut pos = Point::from(anchor, Unit::Cor(1), Unit::Cor(1));
        cursor::go(pos);
        printf!(
            "{}",
            String::from(" ").repeat(
                (((self.progress as f32) / 100.0) * ((self.attr.width.calc() - 2) as f32)) as usize
            )
        );

        set_style(self.attr.text_color);
        set_style(self.attr.text_style);

        if self.progress < 53 {
            set_style(self.attr.fill);
        }

        pos.x += Unit::Cor(self.attr.width.calc() / 2);
        cursor::go(pos);
        printf!("{}%", self.progress);
    }
}

impl ProgressBar {
    pub fn new(bar_color: ColorBG, attr: Option<Attr>) -> Box<ProgressBar> {
        Box::new(ProgressBar {
            attr: attr.unwrap_or_default().height(3).to_owned(),
            bar_color,
            progress: 0,
        })
    }

    pub fn progress(&mut self, value: usize) {
        if value > 100 {
            self.progress = 100;
        } else {
            self.progress = value;
        }
    }

    pub fn increment(&mut self) {
        self.progress += 1;
        if self.progress > 100 {
            self.progress = 100;
        }
    }

    pub fn decrement(&mut self) {
        self.progress -= 1;
        if self.progress > 100 {
            self.progress = 100;
        }
    }

    pub fn inc_progress(&mut self, value: usize) {
        self.progress += value;
        if self.progress > 100 {
            self.progress = 100;
        }
    }

    pub fn dec_progress(&mut self, value: usize) {
        if value > self.progress {
            self.progress = 0;
        } else {
            self.progress -= value;
        }
    }

    pub fn reset(&mut self) {
        self.progress = 0;
    }

    pub fn bar_color(&mut self, value: ColorBG) {
        self.bar_color = value;
    }
}
