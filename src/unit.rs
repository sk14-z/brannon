use crate::app::App;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: Unit,
    pub y: Unit,
}

impl Point {
    pub fn new(x: Unit, y: Unit) -> Point {
        Point { x, y }
    }

    pub fn from(p: Point, x_offset: Unit, y_offset: Unit) -> Point {
        Point::new(p.x + x_offset, p.y + y_offset)
    }

    pub fn cor(x: usize, y: usize) -> Point {
        Point {
            x: Unit::Cor(x),
            y: Unit::Cor(y),
        }
    }

    pub fn pct(x: usize, y: usize) -> Point {
        Point {
            x: Unit::PctH(x),
            y: Unit::PctV(y),
        }
    }

    pub fn pair(&self) -> (usize, usize) {
        (self.x.calc(), self.y.calc())
    }
}

#[derive(Copy, Clone)]
pub enum Unit {
    Cor(usize),           // Columns OR Rows (start at 1)
    PctH(usize),          // Horizontal Percent
    PctV(usize),          // Vertical Percent
    PctHPO(usize, usize), // Horizontal Percent + Offset
    PctVPO(usize, usize), // Vertical Percent + Offset
}

impl Unit {
    pub fn calc(&self) -> usize {
        match self {
            Unit::Cor(n) => *n,
            Unit::PctH(n) => {
                if *n == 100 {
                    App::get_tsz().0 - 1
                } else {
                    (((*n as f32) / 100.0) * (crate::app::App::get_tsz().0 as f32)) as usize
                }
            }
            Unit::PctV(n) => {
                (((*n as f32) / 100.0) * (crate::app::App::get_tsz().1 as f32)) as usize
            }
            Unit::PctHPO(n, o) => {
                ((((*n as f32) / 100.0) * (crate::app::App::get_tsz().0 as f32)) as usize) + o
            }
            Unit::PctVPO(n, o) => {
                ((((*n as f32) / 100.0) * (crate::app::App::get_tsz().1 as f32)) as usize) + o
            }
        }
    }
}

// Adding incompatiable percents automatically converts the rhs
impl std::ops::Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Unit::Cor(n) => match rhs {
                Unit::Cor(n2) => Unit::Cor(n + n2),
                Unit::PctH(n2) => Unit::PctHPO(n2, n),
                Unit::PctV(n2) => Unit::PctVPO(n2, n),
                Unit::PctHPO(n2, o2) => Unit::PctHPO(n2, n + o2),
                Unit::PctVPO(n2, o2) => Unit::PctVPO(n2, n + o2),
            },
            Unit::PctH(n) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, n2),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctH(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o2),
            },
            Unit::PctV(n) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, n2),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctV(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n + n2, o2),
            },
            Unit::PctHPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, o + n2),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctHPO(n + n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o + o2),
            },
            Unit::PctVPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctVPO(n, o + n2),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctVPO(n + n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n + n2, o + o2),
            },
        }
    }
}

impl std::ops::AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
