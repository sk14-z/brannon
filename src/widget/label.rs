use crate::{
    draw::cursor,
    impl_widget_base, printf,
    style::{
        align::{AlignX, AlignY},
        set_style,
    },
    unit::{Point, Unit},
    widget::{Widget, attr::Attr},
};
use std::any::Any;

#[derive(Clone, PartialEq)]
pub struct Label {
    pub attr: Attr,
    pub text: String,
}

impl Label {
    pub fn new(text: impl Into<String>, attr: Option<Attr>) -> Box<Label> {
        Box::new(Label {
            attr: attr.unwrap_or_default(),
            text: text.into(),
        })
    }
}

impl_widget_base!(Label);

impl Widget for Label {
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

        let mut pos: Point = (
            anchor,
            Unit::CoR(1),
            match self.attr.aligny {
                AlignY::Top => Unit::CoR(1),
                AlignY::Center => Unit::CoR((self.attr.height.calc() / 2) - (lines.len() / 2)),
                AlignY::Bottom => Unit::CoR(self.attr.height.calc() - lines.len() - 1),
            },
        )
            .into();

        for line in lines {
            if line.len() < (self.attr.width.calc() - 2) {
                pos.x += Unit::CoR(match self.attr.alignx {
                    AlignX::Left => 0,
                    AlignX::Center => (self.attr.width.calc() / 2) - (line.len() / 2) - 1,
                    AlignX::Right => self.attr.width.calc() - line.len() - 2,
                })
            }
            cursor::go(pos);
            printf!("{}", line);
            pos.y += Unit::CoR(1);
        }
    }
}
