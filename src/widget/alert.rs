use super::attr::Attr;
use crate::{
    draw::{cursor, draw_arc_box, draw_binds, draw_box, draw_title},
    style::{
        self,
        align::AlignX,
        color::{Color, ColorBG},
        line::Line,
        set_style,
        text::TextStyle,
    },
    unit::{Point, Unit},
};

pub enum AlertType {
    Info,
    Warning,
    Error,
}

pub struct Alert {
    pub attr: Attr,
    text: String,
    alert_type: AlertType,
}

impl Alert {
    pub fn new(text: String, alert_type: AlertType) -> Alert {
        let (color, title) = match alert_type {
            AlertType::Info => (Color::Green, String::from("Info")),
            AlertType::Warning => (Color::Yellow, String::from("Warning")),
            AlertType::Error => (Color::Red, String::from("Error")),
        };

        Alert {
            attr: Attr::new(),
            text,
            alert_type,
        }
    }

    fn style(&self) -> &Attr {
        &(self.attr)
    }

    pub fn render(&self, anchor: Point) {
        set_style(self.attr.text_color);
        set_style(self.attr.text_style);

        let mut lines: Vec<String> = self
            .text
            .chars()
            .collect::<Vec<_>>()
            .chunks(self.attr.width.calc() - 2)
            .map(|chunk| chunk.iter().collect())
            .collect();
        lines.truncate(self.attr.height.calc() - 2);

        let anchor = Point::new(Unit::PctH(35), Unit::PctV(35));
        let mut pos = Point::from(
            anchor,
            Unit::Cor(1),
            Unit::Cor((self.attr.height.calc() / 2) - (lines.len() / 2)),
        );

        for line in lines {
            if line.len() < (self.attr.width.calc() - 2) {
                pos.x += Unit::Cor((self.attr.width.calc() / 2) - (line.len() / 2) - 1)
            }

            cursor::go(pos);
            printf!("{}", line);
            pos.y += Unit::Cor(1);
        }

        style::reset();

        // if !self.attr.hide_border {
        //     if self.attr.arc {
        //         draw_arc_box(
        //             anchor,
        //             self.attr.border_color,
        //             self.attr.width.calc(),
        //             self.attr.height.calc(),
        //         );
        //     } else {
        //         draw_box(
        //             anchor,
        //             if self.attr.selected {
        //                 Line::Heavy
        //             } else {
        //                 Line::Light
        //             },
        //             self.attr.border_color,
        //             self.attr.width.calc(),
        //             self.attr.height.calc(),
        //         );
        //     }
        //
        //     if !self.attr.hide_title {
        //         draw_title(
        //             anchor,
        //             self.attr.width.calc(),
        //             self.attr.title.clone(),
        //             self.attr.border_color,
        //             self.attr.title_align,
        //         );
        //     }
        //
        //     if !self.attr.hide_binds {
        //         draw_binds(
        //             Point::from(anchor, Unit::Cor(0), Unit::Cor(self.attr.height.calc() - 1)),
        //             self.attr.width.calc(),
        //             self.attr
        //                 .binds
        //                 .iter()
        //                 .map(|bind| format!("<{}> {}", bind.0.to_char().unwrap_or(' '), bind.1))
        //                 .collect::<Vec<_>>()
        //                 .join(" "),
        //             self.attr.border_color,
        //             self.attr.binds_align,
        //         );
        //     }
        // }
    }

    fn as_alert(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}
