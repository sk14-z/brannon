pub mod attr;
pub mod container;
pub mod label;
pub mod progress_bar;

use crate::{
    draw::{cursor, draw_binds, draw_box, draw_title},
    panel::Panel,
    printf,
    style::{line::Line, set_style, text::TextStyle},
    unit::Point,
};
use attr::Attr;
use std::any::Any;

pub trait WidgetBase: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn style(&self) -> &Attr;

    fn style_mut(&mut self) -> &mut Attr;

    fn wclone(&self) -> Box<dyn Widget>;

    fn weq(&self, other: &dyn Widget) -> bool;
}

pub trait Widget: WidgetBase {
    fn set_style(&mut self, attr: Attr) {
        (*self.style_mut()) = attr;
    }

    fn as_panel(&mut self) -> Option<&mut dyn Panel> {
        None
    }

    fn render(&mut self, anchor: Point);

    fn outline(&self, anchor: Point) {
        self.fill(anchor);
        self.border(anchor);
    }

    fn fill(&self, anchor: Point) {
        if self.style().should_fill {
            let h = self.style().height.calc();
            let s = " ".repeat(self.style().width.calc());
            let mut pos = anchor;

            set_style(self.style().fill);

            for _ in 0..h {
                cursor::go(pos);
                printf!("{}", s);
                pos.y += 1.into();
            }
        }
    }

    fn border(&self, anchor: Point) {
        set_style(self.style().border_color);
        set_style(self.style().border_fill);

        draw_box(
            anchor,
            self.style(),
            if self.style().selected {
                Line::Heavy
            } else {
                Line::Light
            },
        );

        set_style(TextStyle::Bold);

        draw_title(anchor, self.style());
        draw_binds(anchor, self.style());
    }
}

impl Clone for Box<dyn Widget> {
    fn clone(&self) -> Box<dyn Widget> {
        self.wclone()
    }
}

impl PartialEq for Box<dyn Widget> {
    fn eq(&self, other: &Box<dyn Widget>) -> bool {
        self.weq(other.as_ref())
    }
}

#[derive(PartialEq, Default)]
pub struct WidgetList(pub(crate) Vec<Box<dyn Widget>>);

impl Clone for WidgetList {
    fn clone(&self) -> Self {
        Self(self.0.iter().map(|w| w.wclone()).collect())
    }
}

impl From<Box<dyn Widget>> for WidgetList {
    fn from(widget: Box<dyn Widget>) -> Self {
        Self(vec![widget])
    }
}

impl From<Vec<Box<dyn Widget>>> for WidgetList {
    fn from(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self(widgets)
    }
}

impl<const N: usize> From<[Box<dyn Widget>; N]> for WidgetList {
    fn from(widgets: [Box<dyn Widget>; N]) -> Self {
        Self(widgets.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::{WidgetBase, WidgetList, container::Container, label::Label};

    #[test]
    fn from() {
        let l1 = Label::new("", None);
        let l2 = Label::new("", None);
        let c1 = Container::new(None);

        let wl_with_1 = WidgetList(vec![c1.wclone()]);

        assert!(wl_with_1 == c1.wclone().into());

        let wl = WidgetList(vec![l1.wclone(), l2.wclone(), c1.wclone()]);

        // assert!(wl == vec![l1.clone(), l2.clone(), c1.wclone()].into());
        assert!(wl == [l1.wclone(), l2.wclone(), c1.wclone(),].into());
    }
}
