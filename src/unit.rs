use crate::app::get_tsz;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: Unit,
    pub y: Unit,
}

impl Point {
    pub fn new<T: Into<Unit>>(x: T, y: T) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn from<T: Into<Unit>>(p: Point, x_offset: T, y_offset: T) -> Point {
        Point::new(p.x + x_offset.into(), p.y + y_offset.into())
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
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Unit {
    Cor(usize),         // Columns OR Rows (start at 1)
    PctH(usize),        // Horizontal Percent
    PctV(usize),        // Vertical Percent
    PctHPO(usize, i16), // Horizontal Percent + Offset
    PctVPO(usize, i16), // Vertical Percent + Offsett
}

impl Unit {
    pub fn calc(&self) -> usize {
        match self {
            Unit::Cor(n) => *n,
            Unit::PctH(n) => (((*n as f32) / 100.0) * (get_tsz().0 as f32)) as usize,
            Unit::PctV(n) => (((*n as f32) / 100.0) * (get_tsz().1 as f32)) as usize,
            Unit::PctHPO(n, o) => {
                (((((*n as f32) / 100.0) * (get_tsz().0 as f32)) as i16) + o) as usize
            }
            Unit::PctVPO(n, o) => {
                (((((*n as f32) / 100.0) * (get_tsz().1 as f32)) as i16) + o) as usize
            }
        }
    }
}

impl From<usize> for Unit {
    fn from(value: usize) -> Self {
        Unit::Cor(value)
    }
}

// Adding unlike orientations is not reccomended. Orientation of lhs takes precedence: H + V = H
impl std::ops::Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Unit::Cor(n) => match rhs {
                Unit::Cor(n2) => Unit::Cor(n + n2),
                Unit::PctH(n2) => Unit::PctHPO(n2, n as i16),
                Unit::PctV(n2) => Unit::PctVPO(n2, n as i16),
                Unit::PctHPO(n2, o2) => Unit::PctHPO(n2, n as i16 + o2),
                Unit::PctVPO(n2, o2) => Unit::PctVPO(n2, n as i16 + o2),
            },
            Unit::PctH(n) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctH(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o2),
            },
            Unit::PctV(n) => match rhs {
                Unit::Cor(n2) => Unit::PctVPO(n, n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctV(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n + n2, o2),
            },
            Unit::PctHPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, o + n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctHPO(n + n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o + o2),
            },
            Unit::PctVPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctVPO(n, o + n2 as i16),
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

impl std::ops::Sub for Unit {
    type Output = Unit;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Unit::Cor(n) => match rhs {
                Unit::Cor(n2) => Unit::Cor(n - n2),
                Unit::PctH(n2) => Unit::PctHPO(n2, -(n as i16)),
                Unit::PctV(n2) => Unit::PctVPO(n2, -(n as i16)),
                Unit::PctHPO(n2, o2) => Unit::PctHPO(n2, n as i16 - o2),
                Unit::PctVPO(n2, o2) => Unit::PctVPO(n2, n as i16 - o2),
            },
            Unit::PctH(n) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, -(n2 as i16)),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctH(n - n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n - n2, o2),
            },
            Unit::PctV(n) => match rhs {
                Unit::Cor(n2) => Unit::PctVPO(n, -(n2 as i16)),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctV(n - n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n - n2, o2),
            },
            Unit::PctHPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctHPO(n, o - n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctHPO(n - n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n - n2, o - o2),
            },
            Unit::PctVPO(n, o) => match rhs {
                Unit::Cor(n2) => Unit::PctVPO(n, o - n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctVPO(n - n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n - n2, o - o2),
            },
        }
    }
}

impl std::ops::SubAssign for Unit {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
