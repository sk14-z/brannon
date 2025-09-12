pub mod box_char;
pub mod cursor;
pub mod direction;

use crate::{
    app::get_tsz,
    style::{self, align::AlignX, line::Line, set_style, text::TextStyle},
    unit::{Point, Unit},
    widget::attr::Attr,
};
use box_char::BoxChar;

pub fn clear() {
    printf!("\x1b[2J");
}

pub(crate) fn draw_frame(attr: &Attr) {
    if !attr.hide_border {
        let anchor = Point::new(1, 1);
        let (w, h) = get_tsz();

        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(anchor);

        printf!(
            "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
            BoxChar::DoubleTL,
            format!("{}", BoxChar::DoubleH).repeat(w - 2),
            BoxChar::DoubleTR,
            format!("{}\x1b[1B", BoxChar::DoubleV).repeat(h - 2),
            BoxChar::DoubleBR,
            format!("{}\x1b[2D", BoxChar::DoubleH).repeat(w - 2),
            BoxChar::DoubleBL,
            format!("{}\x1b[1A\x1b[1D", BoxChar::DoubleV).repeat(h - 2),
        );

        if !attr.hide_title {
            let title = &attr.title;

            let x_offset = match attr.title_align {
                AlignX::Left => 1,
                AlignX::Center => (w / 2) - ((title.len() + 4) / 2),
                AlignX::Right => w - (title.len() + 5),
            };

            set_style(TextStyle::Bold);

            cursor::go(Point::from(anchor, Unit::Cor(x_offset), Unit::Cor(0)));
            printf!("╣ {} ╠", title);
        }

        if !attr.hide_binds {
            let binds = attr
                .binds
                .iter()
                .map(|bind| format!("<{}> {}", bind.0.to_char().unwrap_or(' '), bind.1))
                .collect::<Vec<_>>()
                .join(" ");

            let x_offset = match attr.binds_align {
                AlignX::Left => 1,
                AlignX::Center => (w / 2) - (binds.len() / 2),
                AlignX::Right => w - (binds.len() + 1),
            };

            cursor::go(Point::from(anchor, x_offset, h - 1));
            printf!("{}", binds);
        }

        style::reset();
    }
}

pub fn draw_box(anchor: Point, attr: &Attr, line: Line) {
    if !attr.hide_border {
        let w = attr.width.calc();
        let h = attr.height.calc();

        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(anchor);

        match line {
            Line::Light => printf!(
                "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
                BoxChar::LightTL,
                format!("{}", BoxChar::LightH).repeat(w - 2),
                BoxChar::LightTR,
                format!("{}\x1b[1B\x1b[1D", BoxChar::LightV).repeat(h - 2),
                BoxChar::LightBR,
                format!("{}\x1b[2D", BoxChar::LightH).repeat(w - 2),
                BoxChar::LightBL,
                format!("{}\x1b[1A\x1b[1D", BoxChar::LightV).repeat(h - 2),
            ),
            Line::Heavy => printf!(
                "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
                BoxChar::HeavyTL,
                format!("{}", BoxChar::HeavyH).repeat(w - 2),
                BoxChar::HeavyTR,
                format!("{}\x1b[1B\x1b[1D", BoxChar::HeavyV).repeat(h - 2),
                BoxChar::HeavyBR,
                format!("{}\x1b[2D", BoxChar::HeavyH).repeat(w - 2),
                BoxChar::HeavyBL,
                format!("{}\x1b[1A\x1b[1D", BoxChar::HeavyV).repeat(h - 2),
            ),
            Line::Double => printf!(
                "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
                BoxChar::DoubleTL,
                format!("{}", BoxChar::DoubleH).repeat(w - 2),
                BoxChar::DoubleTR,
                format!("{}\x1b[1B\x1b[1D", BoxChar::DoubleV).repeat(h - 2),
                BoxChar::DoubleBR,
                format!("{}\x1b[2D", BoxChar::DoubleH).repeat(w - 2),
                BoxChar::DoubleBL,
                format!("{}\x1b[1A\x1b[1D", BoxChar::DoubleV).repeat(h - 2),
            ),
        }

        style::reset();
    }
}

pub fn draw_arc_box(anchor: Point, attr: &Attr) {
    if !attr.hide_border {
        let w = attr.width.calc();
        let h = attr.height.calc();

        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(anchor);

        printf!(
            "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
            BoxChar::ArcTL,
            format!("{}", BoxChar::LightH).repeat(w - 2),
            BoxChar::ArcTR,
            format!("{}\x1b[1B\x1b[1D", BoxChar::LightV).repeat(h - 2),
            BoxChar::ArcBR,
            format!("{}\x1b[2D", BoxChar::LightH).repeat(w - 2),
            BoxChar::ArcBL,
            format!("{}\x1b[1A\x1b[1D", BoxChar::LightV).repeat(h - 2),
        );

        style::reset();
    }
}

pub fn draw_title(anchor: Point, attr: &Attr) {
    if !attr.hide_border && !attr.hide_title {
        let width = attr.width.calc();
        let title = &attr.title;

        let x_offset = match attr.alignx {
            AlignX::Left => 1,
            AlignX::Center => (width / 2) - ((title.len() + 4) / 2),
            AlignX::Right => width - (title.len() + 5),
        };

        set_style(TextStyle::Bold);
        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(Point::from(anchor, x_offset, 0));
        printf!("┤ {} ├", title);

        style::reset();
    }
}

pub fn draw_binds(anchor: Point, attr: &Attr) {
    if !attr.hide_border && !attr.hide_binds {
        let width = attr.width.calc();
        let binds = attr
            .binds
            .iter()
            .map(|bind| format!("<{}> {}", bind.0.to_char().unwrap_or(' '), bind.1))
            .collect::<Vec<_>>()
            .join(" ");

        let x_offset = match attr.binds_align {
            AlignX::Left => 1,
            AlignX::Center => (width / 2) - (binds.len() / 2),
            AlignX::Right => width - (binds.len() + 1),
        };

        set_style(TextStyle::Bold);
        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(Point::from(anchor, x_offset, attr.height.calc() - 1));
        printf!("{}", binds);

        style::reset();
    }
}
