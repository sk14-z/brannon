use super::{attr::Attr, Widget};
use crate::{
    draw::{cursor, draw_arc_box, draw_box, draw_title},
    style::{self, align::AlignX, line::Line, set_style},
    unit::{Point, Unit},
};

pub struct Label {
    pub attr: Attr,
    text: String,
}

impl Label {
    pub fn new(text: String, attr: Option<Attr>) -> Label {
        match attr {
            Some(attr) => Label { attr, text },
            None => Label {
                attr: Attr::new(),
                text,
            },
        }
    }

    pub fn text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for Label {
    fn style(&mut self) -> &mut Attr {
        &mut (self.attr)
    }

    fn render(&self) {
        let mut h = 0;

        set_style(self.attr.text_style);
        set_style(self.attr.text_color);

        if self.text.len() > (self.attr.width.calc() - 2) {
            let lines: Vec<String> = self
                .text
                .as_bytes()
                .chunks(self.attr.width.calc() - 2)
                .map(|s| String::from_utf8(s.to_vec()).unwrap())
                .collect();

            h = lines.len() + 2;

            let mut pos = Point::from(self.attr.anchor, Unit::Cor(1), Unit::Cor(1));

            for line in lines {
                if line.len() < (self.attr.width.calc() - 2) {
                    pos.x += Unit::Cor(match self.attr.alignx {
                        AlignX::Left => 0,
                        AlignX::Center => (self.attr.width.calc() / 2) - (line.len() / 2) - 1,
                        AlignX::Right => self.attr.width.calc() - line.len() - 2,
                    })
                }
                cursor::go(pos);
                printf!("{}", line);
                pos.y += Unit::Cor(1);
            }
        } else {
            h = 3;
            cursor::go(Point::from(
                self.attr.anchor,
                Unit::Cor(match self.attr.alignx {
                    AlignX::Left => 1,
                    AlignX::Center => (self.attr.width.calc() / 2) - (self.text.len() / 2),
                    AlignX::Right => (self.attr.width.calc() - self.text.len()) + 1,
                }),
                Unit::Cor(1),
            ));
            printf!("{}", self.text);
        }

        style::reset();

        if !self.attr.hide_border {
            if self.attr.arc {
                draw_arc_box(
                    self.attr.anchor,
                    self.attr.border_color,
                    self.attr.width.calc(),
                    h,
                );
            } else {
                draw_box(
                    self.attr.anchor,
                    Line::Light,
                    self.attr.border_color,
                    self.attr.width.calc(),
                    h,
                );
            }

            if !self.attr.hide_title {
                draw_title(
                    self.attr.anchor,
                    self.attr.width.calc(),
                    &self.attr.title,
                    self.attr.border_color,
                    self.attr.title_align,
                );
            }
        }
    }
}
