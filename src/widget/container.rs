use crate::{
    panel::Panel,
    style::{
        align::{AlignX, AlignY},
        orientation::Orientation,
    },
    unit::{Point, Unit},
    widget::{attr::Attr, Widget},
};

pub struct Container {
    pub attr: Attr,
    children: Vec<Box<dyn Widget>>,
}

impl Panel for Container {
    fn split(&self) -> (&Attr, &Vec<Box<dyn Widget>>) {
        (&self.attr, &self.children)
    }

    fn split_mut(&mut self) -> (&mut Attr, &mut Vec<Box<dyn Widget>>) {
        (&mut self.attr, &mut self.children)
    }

    fn as_container(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

impl Widget for Container {
    fn style(&self) -> &Attr {
        &(self.attr)
    }

    fn style_mut(&mut self) -> &mut Attr {
        &mut (self.attr)
    }

    fn render(&self, anchor: Point) {
        if !self.children.is_empty() {
            let (inner_x, inner_y) = self.bounds();

            let anchorw = Point::from(anchor, Unit::Cor(1), Unit::Cor(1));
            let mut pos = anchorw;

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

                            pos.y = anchorw.y;
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

                            pos.x = anchorw.x;
                            pos.y += child.style().height + child.style().padding_bottom
                        }
                    }
                }
            }
        }

        self.render_border(anchor);
    }

    fn as_container(&mut self) -> Option<&mut Self> {
        Some(self)
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
