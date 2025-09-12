use brannon::{
    draw::cursor,
    printf,
    unit::Point,
    widget::{attr::Attr, Widget},
    widget_shared,
};
use std::any::Any;

pub struct MyWidget {
    pub attr: Attr,
}

impl Widget for MyWidget {
    widget_shared!();

    fn render(&self, anchor: Point) {
        cursor::go(Point::from(anchor, 1, 1));
        printf!("I made a custom widget :)");
        self.border(anchor);
    }
}

impl MyWidget {
    pub fn new(attr: Option<Attr>) -> Box<Self> {
        Box::new(Self {
            attr: attr.unwrap_or_default(),
        })
    }
}
