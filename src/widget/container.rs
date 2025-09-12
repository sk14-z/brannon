use crate::{
    panel::Panel,
    panel_shared,
    style::{
        align::{AlignX, AlignY},
        orientation::Orientation,
    },
    unit::{Point, Unit},
    widget::{attr::Attr, Widget},
    widget_shared,
};
use std::any::Any;

pub struct Container {
    pub attr: Attr,
    children: Vec<Box<dyn Widget>>,
}

impl Panel for Container {
    panel_shared!();
}

impl Widget for Container {
    widget_shared!();

    fn as_panel(&mut self) -> Option<&mut dyn Panel> {
        Some(self)
    }

    fn render(&self, anchor: Point) {
        self.render_children(Point::from(anchor, 1, 1));
    }
}

impl Container {
    pub fn new(attr: Option<Attr>) -> Box<Container> {
        Box::new(Container {
            attr: attr.unwrap_or_default(),
            children: vec![],
        })
    }
}
