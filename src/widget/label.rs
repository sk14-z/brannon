use super::{attr::Attr, Widget};
use crate::{
    draw::cursor,
    style::{
        self,
        align::{AlignX, AlignY},
        set_style,
    },
    unit::{Point, Unit},
};

pub struct Label {
    pub attr: Attr,
    text: String,
}

impl Widget for Label {
    fn style(&self) -> &Attr {
        &(self.attr)
    }

    fn style_mut(&mut self) -> &mut Attr {
        &mut (self.attr)
    }

    fn render(&self, anchor: Point) {
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

        style::reset();

        self.render_border(anchor);
    }

    fn as_label(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

impl Label {
    pub fn new(text: String, attr: Option<Attr>) -> Box<Label> {
        Box::new(Label {
            attr: attr.unwrap_or_default(),
            text,
        })
    }

    pub fn text(&mut self, value: String) {
        self.text = value;
    }
}
