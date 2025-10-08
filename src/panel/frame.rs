use super::Panel;
use crate::{
    app::get_tsz,
    draw::draw_frame,
    panel_shared,
    style::align::{AlignX, AlignY},
    unit::{Point, Unit},
    widget::{Widget, WidgetList, attr::Attr},
};

#[derive(Clone, PartialEq)]
pub struct Frame {
    pub attr: Attr,
    children: WidgetList,
}

impl Frame {
    pub fn new(attr: Option<Attr>) -> Frame {
        Frame {
            attr: attr.unwrap_or_default(),
            children: [].into(),
        }
    }

    pub fn render(&mut self) {
        draw_frame(&self.attr);

        let (inner_x, inner_y) = self.bounds();

        let anchor = Point::new(
            match self.attr.alignx {
                AlignX::Left => Unit::CoR(2),
                AlignX::Center => Unit::CoR((get_tsz().0 / 2) - (inner_x.calc() / 2)),
                AlignX::Right => Unit::CoR(get_tsz().0) - inner_x,
            },
            match self.attr.aligny {
                AlignY::Top => Unit::CoR(2),
                AlignY::Center => Unit::CoR((get_tsz().1 / 2) - (inner_y.calc() / 2)),
                AlignY::Bottom => Unit::CoR(get_tsz().1) - inner_y,
            },
        );

        self.render_children(anchor);
    }
}

impl Panel for Frame {
    panel_shared!();
}
