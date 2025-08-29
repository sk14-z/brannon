use super::Panel;
use crate::{
    app::get_tsz,
    draw::{draw_binds, draw_box, draw_title_double},
    style::{
        align::{AlignX, AlignY},
        line::Line,
        orientation::Orientation,
    },
    unit::{Point, Unit},
    widget::{attr::Attr, Widget},
};

pub struct Frame {
    pub attr: Attr,
    pub children: Vec<Box<dyn Widget>>,
}

impl Panel for Frame {
    fn split(&self) -> (&Attr, &Vec<Box<dyn Widget>>) {
        (&self.attr, &self.children)
    }

    fn split_mut(&mut self) -> (&mut Attr, &mut Vec<Box<dyn Widget>>) {
        (&mut self.attr, &mut self.children)
    }

    fn as_frame(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

impl Frame {
    pub(crate) fn new() -> Frame {
        Frame {
            attr: Attr::new(),
            children: vec![],
        }
    }

    pub fn render(&self) {
        if !self.children.is_empty() {
            let (inner_x, inner_y) = self.bounds();

            let anchor = Point::new(
                match self.attr.alignx {
                    AlignX::Left => Unit::Cor(2),
                    AlignX::Center => Unit::Cor((get_tsz().0 / 2) - (inner_x.calc() / 2)),
                    AlignX::Right => Unit::Cor(get_tsz().0) - inner_x,
                },
                match self.attr.aligny {
                    AlignY::Top => Unit::Cor(2),
                    AlignY::Center => Unit::Cor((get_tsz().1 / 2) - (inner_y.calc() / 2)),
                    AlignY::Bottom => Unit::Cor(get_tsz().1) - inner_y,
                },
            );

            let mut pos = anchor;

            for child in &self.children {
                if !child.style().hide {
                    match self.attr.orientation {
                        Orientation::Horizontal => {
                            pos.y += match self.attr.aligny {
                                AlignY::Top => child.style().padding_top,
                                AlignY::Center => {
                                    Unit::Cor((inner_y - child.style().height).calc() / 2)
                                }
                                AlignY::Bottom => inner_y - child.style().total_height(),
                            };
                            pos.x += child.style().padding_left;

                            child.render(pos);

                            pos.y = anchor.y;
                            pos.x += child.style().width + child.style().padding_right
                        }
                        Orientation::Vertical => {
                            pos.x += match self.attr.alignx {
                                AlignX::Left => child.style().padding_left,
                                AlignX::Center => {
                                    Unit::Cor((inner_x - child.style().width).calc() / 2)
                                }
                                AlignX::Right => inner_x - child.style().total_width(),
                            };
                            pos.y += child.style().padding_top;

                            child.render(pos);

                            pos.x = anchor.x;
                            pos.y += child.style().height + child.style().padding_bottom
                        }
                    }
                }
            }
        }

        if !self.attr.hide_border {
            draw_box(
                Point::cor(1, 1),
                Line::Double,
                self.attr.border_color,
                Unit::PctH(100).calc(),
                Unit::PctV(100).calc(),
            );

            if !self.attr.hide_title {
                draw_title_double(
                    Point::cor(1, 1),
                    Unit::PctH(100).calc(),
                    self.attr.title.clone(),
                    self.attr.border_color,
                    self.attr.title_align,
                );
            }

            if !self.attr.hide_binds {
                draw_binds(
                    Point::new(Unit::Cor(1), Unit::PctHPO(100, -1)),
                    Unit::PctH(100).calc(),
                    self.attr
                        .binds
                        .iter()
                        .map(|bind| format!("<{}> {}", bind.0.to_char().unwrap_or(' '), bind.1))
                        .collect::<Vec<_>>()
                        .join(" "),
                    self.attr.border_color,
                    self.attr.binds_align,
                );
            }
        }
    }
}
