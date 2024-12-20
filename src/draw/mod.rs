pub mod box_char;
pub mod cursor;
pub mod direction;

use crate::{
    style::{self, align::AlignX, color::Color, line::Line, set_style, text::Text},
    unit::{Point, Unit},
};
use box_char::BoxChar;
use direction::Direction;

pub fn clear() {
    printf!("\x1b[2J");
}

// I might have created these for no reason

pub fn draw(c: char, color: Color, direction: Direction, length: usize) {
    set_style(color);

    match direction {
        Direction::Up => printf!("{}", format!("{}\x1b[1A\x1b[1D", c).repeat(length)),
        Direction::Down => printf!("{}", format!("{}\x1b[1B\x1b[1D", c).repeat(length)),
        Direction::Right => printf!("{}", format!("{}", c).repeat(length)),
        Direction::Left => printf!("{}", format!("{}\x1b[2D", c).repeat(length)),
    }

    style::reset();
}

pub fn draw_at(p: Point, c: char, color: Color, direction: Direction, length: usize) {
    cursor::go(p);
    draw(c, color, direction, length);
}

pub fn draw_box(anchor: Point, line: Line, color: Color, w: usize, h: usize) {
    cursor::go(anchor);
    set_style(color);

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

pub fn draw_arc_box(anchor: Point, color: Color, w: usize, h: usize) {
    cursor::go(anchor);
    set_style(color);

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

pub fn draw_title(anchor: Point, width: usize, title: &String, color: Color, align: AlignX) {
    let x_offset = match align {
        AlignX::Left => 1,
        AlignX::Center => (width / 2) - (title.len() / 2),
        AlignX::Right => width - (title.len() + 1),
    };

    set_style(Text::Bold);
    set_style(color);

    cursor::go(Point::from(anchor, Unit::Cor(x_offset), Unit::Cor(0)));
    printf!("{}", title);

    style::reset();
}
