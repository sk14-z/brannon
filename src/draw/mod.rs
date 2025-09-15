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
            let l = attr.binds.len();

            let x_offset = match attr.binds_align {
                AlignX::Left => 1,
                AlignX::Center => (w / 2) - (l / 2),
                AlignX::Right => w - (l + 1),
            };

            cursor::go(Point::from(anchor, x_offset, h - 1));
            printf!("{}", attr.binds);
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

        let (hc, vc, mut tl, mut tr, mut br, mut bl) = match line {
            Line::Light => (
                BoxChar::LightH,
                BoxChar::LightV,
                BoxChar::LightTL,
                BoxChar::LightTR,
                BoxChar::LightBR,
                BoxChar::LightBL,
            ),
            Line::Heavy => (
                BoxChar::HeavyH,
                BoxChar::HeavyV,
                BoxChar::HeavyTL,
                BoxChar::HeavyTR,
                BoxChar::HeavyBR,
                BoxChar::HeavyBL,
            ),
            Line::Double => (
                BoxChar::DoubleH,
                BoxChar::DoubleV,
                BoxChar::DoubleTL,
                BoxChar::DoubleTR,
                BoxChar::DoubleBR,
                BoxChar::DoubleBL,
            ),
        };

        if attr.arc {
            tl = BoxChar::ArcTL;
            tr = BoxChar::ArcTR;
            br = BoxChar::ArcBR;
            bl = BoxChar::ArcBL;
        }

        cursor::go(anchor);

        printf!(
            "{}{}{}\x1b[1D\x1b[1B{}{}\x1b[2D{}{}\x1b[1D\x1b[1A{}",
            tl,
            format!("{}", hc).repeat(w - 2),
            tr,
            format!("{}\x1b[1B\x1b[1D", vc).repeat(h - 2),
            br,
            format!("{}\x1b[2D", hc).repeat(w - 2),
            bl,
            format!("{}\x1b[1A\x1b[1D", vc).repeat(h - 2),
        );

        // printf!("{}{}{}", tl, format!("{}", hc).repeat(w - 2), tr);
        //
        // printf!(
        //     "\x1b[1D\x1b[1B{}{}",
        //     format!("{}\x1b[1B\x1b[1D", vc).repeat(h - 2),
        //     br,
        // );
        //
        // printf!("\x1b[2D{}{}", format!("{}\x1b[2D", hc).repeat(w - 2), bl,);
        //
        // printf!(
        //     "\x1b[1D\x1b[1A{}",
        //     format!("{}\x1b[1A\x1b[1D", vc).repeat(h - 2),
        // );

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
        let l = attr.binds.len();

        let x_offset = match attr.binds_align {
            AlignX::Left => 1,
            AlignX::Center => (width / 2) - (l / 2),
            AlignX::Right => width - (l + 1),
        };

        set_style(TextStyle::Bold);
        set_style(attr.border_color);
        set_style(attr.border_fill);

        cursor::go(Point::from(anchor, x_offset, attr.height.calc() - 1));
        printf!("{}", attr.binds);

        style::reset();
    }
}
