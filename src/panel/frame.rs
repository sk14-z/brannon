use super::Panel;
use crate::{
    app::get_tsz,
    draw::draw_frame,
    panel_shared,
    style::align::{AlignX, AlignY},
    unit::{Point, Unit},
    widget::{Widget, attr::Attr},
};

pub struct Frame {
    pub attr: Attr,
    children: Vec<Box<dyn Widget>>,
}

impl Panel for Frame {
    panel_shared!();
}

impl Frame {
    pub(crate) fn new() -> Frame {
        Frame {
            attr: Attr::new(),
            children: vec![],
        }
    }

    pub fn render(&mut self) {
        draw_frame(&self.attr);

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

        self.render_children(anchor);
    }
}
