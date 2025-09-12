pub mod alert;
pub mod attr;
pub mod container;
pub mod label;
pub mod progress_bar;

use crate::{
    draw::{cursor, draw_arc_box, draw_binds, draw_box, draw_title},
    panel::Panel,
    style::{line::Line, set_style},
    unit::Point,
};
use attr::Attr;
use std::any::Any;

#[macro_export]
macro_rules! widget_shared {
    () => {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn style(&self) -> &Attr {
            &(self.attr)
        }

        fn style_mut(&mut self) -> &mut Attr {
            &mut (self.attr)
        }
    };
}

pub trait Widget: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn style(&self) -> &Attr;

    fn style_mut(&mut self) -> &mut Attr;

    fn set_style(&mut self, attr: Attr) {
        (*self.style_mut()) = attr;
    }

    fn as_panel(&mut self) -> Option<&mut dyn Panel> {
        None
    }

    fn render(&self, anchor: Point);

    fn outline(&self, anchor: Point) {
        self.fill(anchor);
        self.border(anchor);
    }

    fn fill(&self, anchor: Point) {
        // if self.style().fill != ColorBG::None {
        set_style(self.style().fill);
        let h = self.style().height.calc();
        let s = " ".repeat(self.style().width.calc());
        let mut pos = anchor;

        for _ in 0..h {
            cursor::go(pos);
            printf!("{}", s);
            pos.y += 1.into();
        }
        // }
    }

    fn border(&self, anchor: Point) {
        if self.style().arc {
            draw_arc_box(anchor, self.style());
        } else {
            draw_box(
                anchor,
                self.style(),
                if self.style().selected {
                    Line::Heavy
                } else {
                    Line::Light
                },
            );
        }

        draw_title(anchor, self.style());
        draw_binds(anchor, self.style());
    }
}
