use crate::{
    impl_widget_base,
    panel::Panel,
    panel_shared,
    unit::Point,
    widget::{Widget, WidgetList, attr::Attr},
};
use std::any::Any;

#[derive(Clone, PartialEq)]
pub struct Container {
    pub attr: Attr,
    pub children: WidgetList,
}

impl Container {
    pub fn new(attr: Option<Attr>) -> Box<Container> {
        Box::new(Container {
            attr: attr.unwrap_or_default(),
            children: [].into(),
        })
    }
}

impl_widget_base!(Container);

impl Panel for Container {
    panel_shared!();
}

impl Widget for Container {
    fn as_panel(&mut self) -> Option<&mut dyn Panel> {
        Some(self)
    }

    fn render(&mut self, anchor: Point) {
        self.render_children(Point::from(anchor, 1, 1));
    }
}
