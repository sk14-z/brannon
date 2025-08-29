pub mod alert;
pub mod attr;
pub mod container;
pub mod label;
pub mod progress_bar;

use crate::{
    draw::{draw_arc_box, draw_binds, draw_box, draw_title},
    style::line::Line,
    unit::{Point, Unit},
};

pub trait Widget {
    fn style(&self) -> &attr::Attr;

    fn style_mut(&mut self) -> &mut attr::Attr;

    fn set_style(&mut self, attr: attr::Attr) {
        (*self.style_mut()) = attr;
    }

    fn hide(&mut self) {
        self.style_mut().hide = true;
    }
    fn show(&mut self) {
        self.style_mut().hide = false;
    }

    fn render(&self, anchor: Point);

    fn render_border(&self, anchor: Point) {
        if !self.style().hide_border {
            if self.style().arc {
                draw_arc_box(
                    anchor,
                    self.style().border_color,
                    self.style().width.calc(),
                    self.style().height.calc(),
                );
            } else {
                draw_box(
                    anchor,
                    if self.style().selected {
                        Line::Heavy
                    } else {
                        Line::Light
                    },
                    self.style().border_color,
                    self.style().width.calc(),
                    self.style().height.calc(),
                );
            }

            if !self.style().hide_title {
                draw_title(
                    anchor,
                    self.style().width.calc(),
                    self.style().title.clone(),
                    self.style().border_color,
                    self.style().title_align,
                );
            }

            if !self.style().hide_binds {
                draw_binds(
                    Point::from(
                        anchor,
                        Unit::Cor(0),
                        Unit::Cor(self.style().height.calc() - 1),
                    ),
                    self.style().width.calc(),
                    self.style()
                        .binds
                        .iter()
                        .map(|bind| format!("<{}> {}", bind.0.to_char().unwrap_or(' '), bind.1))
                        .collect::<Vec<_>>()
                        .join(" "),
                    self.style().border_color,
                    self.style().binds_align,
                );
            }
        }
    }

    fn as_alert(&mut self) -> Option<&mut alert::Alert> {
        None
    }

    fn as_label(&mut self) -> Option<&mut label::Label> {
        None
    }

    fn as_progress_bar(&mut self) -> Option<&mut progress_bar::ProgressBar> {
        None
    }

    fn as_container(&mut self) -> Option<&mut container::Container> {
        None
    }

    fn as_frame(&mut self) -> Option<&mut crate::panel::frame::Frame> {
        None
    }
}
