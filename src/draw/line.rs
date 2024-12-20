// Just wanted to test something out, this will probaly never be used
// Shitpost file. If you're here, leave.

use crate::{
    cursor,
    direction::Direction,
    style::{
        self,
        color::{Color, ColorFG},
        set_style,
    },
    unit::Point,
};

pub fn draw_char(c: char, color: &Color, direction: Direction, length: usize) {
    set_style(color);

    match direction {
        Direction::Up => printf!("{}", format!("{}\x1b[1A\x1b[1D", c).repeat(length)),
        Direction::Down => printf!("{}", format!("{}\x1b[1B\x1b[1D", c).repeat(length)),
        Direction::Right => printf!("{}", format!("{}", c).repeat(length)),
        Direction::Left => printf!("{}", format!("{}\x1b[2D", c).repeat(length)),
    }

    style::reset();
}

pub fn line(start: Point, end: Point, c: char, color: &Color) {
    set_style(color);
    cursor::go(start);

    if start.x.calc() != end.x.calc() {
        let slope: f32 = -(end.y.calc() as f32 - start.y.calc() as f32)
            / (end.x.calc() as f32 - start.x.calc() as f32);

        let mut y: f32 = 0.0;
        let mut x: f32 = 1.0;

        if start.x.calc() < end.x.calc() {
            printf!("{}", c);
        } else {
            printf!("\x1b[1D{}{}", c, c)
        }

        while x <= start.x.calc().abs_diff(end.x.calc()) as f32 {
            if y / x < slope.abs() {
                if start.y.calc() > end.y.calc() {
                    printf!("\x1b[1A\x1b[1D{}", c);
                    y += 1.0;
                } else {
                    printf!("\x1b[1B\x1b[1D{}", c);
                    y += 1.0;
                }
            } else if start.x.calc() < end.x.calc() {
                printf!("{}", c);
                x += 1.0;
            } else {
                printf!("\x1b[2D{}", c);
                x += 1.0;
            }
        }
    } else if start.y.calc() < end.y.calc() {
        draw_char(c, color, Direction::Down, end.y.calc() - start.y.calc());
    } else {
        draw_char(c, color, Direction::Up, start.y.calc() - end.y.calc());
    }

    style::reset();
}

pub fn multiline(vertices: Vec<Point>, c: char, color: &Color) {
    let mut prev: Point = vertices[0];
    for vertex in vertices {
        line(prev, vertex, c, color);
        prev = vertex;
    }
}

pub fn draw_house_that_takes_up_entire_screen_and_flickers_for_some_reason_i_dont_care_enough_to_fix(
) {
    multiline(
        vec![
            Point::pct(0, 50),
            Point::pct(50, 5),
            Point::pct(100, 50),
            Point::pct(0, 50),
            Point::pct(0, 100),
            Point::pct(100, 100),
            Point::pct(100, 48),
        ],
        '\u{2588}',
        &Color(Some(ColorFG::Blue), None),
    );
}
