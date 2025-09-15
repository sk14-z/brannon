use super::{Widget, attr::Attr};
use crate::{
    draw::cursor,
    style::{
        align::{AlignX, AlignY},
        set_style,
    },
    unit::{Point, Unit},
    widget_shared,
};
use std::any::Any;

pub struct Label {
    pub attr: Attr,
    pub text: String,
}

impl Widget for Label {
    widget_shared!();

    fn render(&mut self, anchor: Point) {
        set_style(self.attr.text_style);
        set_style(self.attr.text_color);

        let mut lines: Vec<String> = self
            .text
            .chars()
            .collect::<Vec<_>>()
            .chunks(self.attr.width.calc() - 2)
            .map(|chunk| chunk.iter().collect())
            .collect();
        lines.truncate(self.attr.height.calc() - 2);

        let mut pos = Point::from(
            anchor,
            Unit::Cor(1),
            match self.attr.aligny {
                AlignY::Top => Unit::Cor(1),
                AlignY::Center => Unit::Cor((self.attr.height.calc() / 2) - (lines.len() / 2)),
                AlignY::Bottom => Unit::Cor(self.attr.height.calc() - lines.len() - 1),
            },
        );

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
    }
}

impl Label {
    pub fn new(text: String, attr: Option<Attr>) -> Box<Label> {
        Box::new(Label {
            attr: attr.unwrap_or_default(),
            text,
        })
    }
}
