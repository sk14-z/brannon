pub mod frame;

use crate::{
    style::{
        self,
        align::{AlignX, AlignY},
        orientation::Orientation,
        set_style,
    },
    unit::{Point, Unit},
    widget::{Widget, attr::Attr},
};

pub trait Panel {
    fn split(&self) -> (&Attr, &Vec<Box<dyn Widget>>);
    fn split_mut(&mut self) -> (&mut Attr, &mut Vec<Box<dyn Widget>>);

    fn map_all(&mut self, map: fn(&mut Box<dyn Widget>)) {
        let (_, children) = self.split_mut();

        for child in children.iter_mut() {
            map(child);

            if let Some(panel) = child.as_panel() {
                panel.map_all(map);
            }
        }
    }

    fn style_all(&mut self, map: fn(&mut Attr)) {
        let (_, children) = self.split_mut();

        for child in children.iter_mut() {
            map(child.style_mut());

            if let Some(panel) = child.as_panel() {
                panel.style_all(map);
            }
        }
    }

    fn get_child(&mut self, tag: &str) -> Option<&mut Box<dyn Widget>> {
        let (_, children) = self.split_mut();

        for child in children.iter_mut() {
            if child.style().tag == tag {
                return Some(child);
            }

            if let Some(panel) = child.as_panel()
                && let Some(widget) = panel.get_child(tag)
            {
                return Some(widget);
            }
        }

        None
    }

    fn child_at(&mut self, index: usize) -> Option<&mut Box<dyn Widget>> {
        let (_, children) = self.split_mut();

        children.get_mut(index)
    }

    fn add(&mut self, widget: Box<dyn Widget>) {
        let (_, children) = self.split_mut();

        children.push(widget);
        self.flex();
    }

    fn addm(&mut self, widgets: Vec<Box<dyn Widget>>) {
        let (_, children) = self.split_mut();

        for widget in widgets {
            children.push(widget);
        }

        self.flex();
    }

    fn remove(&mut self, tag: &str) -> Option<Box<dyn Widget>> {
        let (_, children) = self.split_mut();

        let mut removed = None;

        if let Some(i) = children.iter().position(|w| w.style().tag == tag) {
            removed = Some(children.remove(i));
        }

        self.flex();

        removed
    }

    fn removem(&mut self, tags: Vec<&str>) -> Vec<Option<Box<dyn Widget>>> {
        let (_, children) = self.split_mut();

        let mut removed = Vec::new();

        for tag in tags {
            if let Some(i) = children.iter().position(|w| w.style().tag == tag) {
                removed.push(Some(children.remove(i)))
            } else {
                removed.push(None)
            }
        }

        self.flex();

        removed
    }

    fn bounds(&self) -> (Unit, Unit) {
        let (attr, children) = self.split();

        if children.is_empty() {
            (Unit::CoR(0), Unit::CoR(0))
        } else {
            match attr.orientation {
                Orientation::Horizontal => {
                    let inner_x = Unit::CoR(
                        children
                            .iter()
                            .map(|c| {
                                if !c.style().hide {
                                    c.style().total_width().calc()
                                } else {
                                    0
                                }
                            })
                            .sum(),
                    );

                    let inner_y = Unit::CoR(
                        children
                            .iter()
                            .map(|c| {
                                if !c.style().hide {
                                    c.style().total_height().calc()
                                } else {
                                    0
                                }
                            })
                            .max()
                            .unwrap(),
                    );

                    (inner_x, inner_y)
                }
                Orientation::Vertical => {
                    let inner_x = Unit::CoR(
                        children
                            .iter()
                            .map(|c| {
                                if !c.style().hide {
                                    c.style().total_width().calc()
                                } else {
                                    0
                                }
                            })
                            .max()
                            .unwrap(),
                    );

                    let inner_y = Unit::CoR(
                        children
                            .iter()
                            .map(|c| {
                                if !c.style().hide {
                                    c.style().total_height().calc()
                                } else {
                                    0
                                }
                            })
                            .sum(),
                    );

                    (inner_x, inner_y)
                }
            }
        }
    }

    fn render_children(&mut self, anchor: Point) {
        let (inner_x, inner_y) = self.bounds();
        let (attr, children) = self.split_mut();
        let mut pos = anchor;

        for child in children {
            if !child.style().hide {
                match attr.orientation {
                    Orientation::Horizontal => {
                        pos.y += match attr.aligny {
                            AlignY::Top => child.style().padding_top,
                            AlignY::Center => {
                                Unit::CoR((inner_y.calc() / 2) - (child.style().height.calc() / 2))
                            }
                            AlignY::Bottom => inner_y - child.style().total_height(),
                        };
                        pos.x += child.style().padding_left;

                        let fill = child.style().fill;
                        if fill != attr.fill {
                            child.style_mut().should_fill = true;
                        }

                        child.outline(pos);

                        set_style(child.style().fill);
                        child.render(pos);

                        pos.y = anchor.y;
                        pos.x += child.style().width + child.style().padding_right
                    }
                    Orientation::Vertical => {
                        pos.x += match attr.alignx {
                            AlignX::Left => child.style().padding_left,
                            AlignX::Center => {
                                Unit::CoR((inner_x.calc() / 2) - (child.style().width.calc() / 2))
                            }
                            AlignX::Right => inner_x - child.style().total_width(),
                        };
                        pos.y += child.style().padding_top;

                        let fill = child.style().fill;
                        if fill != attr.fill {
                            child.style_mut().should_fill = true;
                        }

                        child.outline(pos);

                        set_style(child.style().fill);
                        child.render(pos);

                        pos.x = anchor.x;
                        pos.y += child.style().height + child.style().padding_bottom
                    }
                }

                style::reset();
            }
        }
    }

    fn flex(&mut self) {
        let (inner_x, inner_y) = self.bounds();
        let (attr, _) = self.split_mut();

        if inner_x.calc() >= attr.width.calc() {
            attr.width = inner_x + Unit::CoR(2);
        }

        if inner_y.calc() >= attr.height.calc() {
            attr.height = inner_y + Unit::CoR(2);
        }

        if attr.flex {
            if inner_x.calc() <= attr.width.calc() {
                attr.width = inner_x + Unit::CoR(2);
            }

            if inner_y.calc() <= attr.height.calc() {
                attr.height = inner_y + Unit::CoR(2);
            }
        }
    }
}
