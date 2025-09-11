pub(crate) mod frame;

use crate::{
    style::orientation::Orientation,
    unit::Unit,
    widget::{attr::Attr, Widget},
};

pub trait Panel {
    fn split(&self) -> (&Attr, &Vec<Box<dyn Widget>>);
    fn split_mut(&mut self) -> (&mut Attr, &mut Vec<Box<dyn Widget>>);

    fn style_all(&mut self, map: fn(&mut Box<dyn Widget>)) {
        let (_, children) = self.split_mut();

        for child in children.iter_mut() {
            map(child);

            if let Some(container) = child.as_container() {
                container.style_all(map);
            }
        }
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

        self.shrink();

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

        self.shrink();

        removed
    }

    fn get_widget(&mut self, tag: &str) -> Option<&mut Box<dyn Widget>> {
        let (_, children) = self.split_mut();

        for child in children.iter_mut() {
            if child.style().tag == tag {
                return Some(child);
            }

            if let Some(container) = child.as_container() {
                if let Some(widget) = container.get_widget(tag) {
                    return Some(widget);
                }
            }
        }

        None
    }

    fn bounds(&self) -> (Unit, Unit) {
        let (attr, children) = self.split();
        match attr.orientation {
            Orientation::Horizontal => {
                let inner_x = Unit::Cor(
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

                let inner_y = Unit::Cor(
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
                let inner_x = Unit::Cor(
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

                let inner_y = Unit::Cor(
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

    // If too small, adjust to fit children
    fn flex(&mut self) {
        let (inner_x, inner_y) = self.bounds();
        let (attr, _) = self.split_mut();

        if inner_x.calc() >= attr.width.calc() {
            attr.width = inner_x + Unit::Cor(2);
        }

        if inner_y.calc() >= attr.height.calc() {
            attr.height = inner_y + Unit::Cor(2);
        }
    }

    // If too big, adjust to fit children
    fn shrink(&mut self) {
        let (inner_x, inner_y) = self.bounds();
        let (attr, _) = self.split_mut();

        if inner_x.calc() <= attr.width.calc() {
            attr.width = inner_x + Unit::Cor(2);
        }

        if inner_y.calc() <= attr.height.calc() {
            attr.height = inner_y + Unit::Cor(2);
        }
    }

    fn as_frame(&mut self) -> Option<&mut frame::Frame> {
        None
    }

    fn as_container(&mut self) -> Option<&mut crate::widget::container::Container> {
        None
    }
}
