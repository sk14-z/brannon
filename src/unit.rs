use crate::app::get_tsz;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

    pub fn cor(x: usize, y: usize) -> Point {
        Point {
            x: Unit::CoR(x),
            y: Unit::CoR(y),
        }
    }

    pub fn pct(x: usize, y: usize) -> Point {
        Point {
            x: Unit::PctH(x),
            y: Unit::PctV(y),
        }
    }
}

impl<T: Into<Unit>> From<T> for Point {
    fn from(value: T) -> Self {
        let v = value.into();
        Point::new(v, v)
    }
}

impl<T: Into<Unit>> From<(T, T)> for Point {
    fn from(value: (T, T)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl<T: Into<Unit>> From<(Point, T, T)> for Point {
    fn from(value: (Point, T, T)) -> Self {
        let mut p = value.0;
        p.x += value.1.into();
        p.y += value.2.into();

        p
    }
}

impl From<()> for Point {
    fn from(_: ()) -> Self {
        Point::new(0, 0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Unit {
    CoR(usize),         // Columns OR Rows (start at 1)
    PctH(usize),        // Horizontal Percent
    PctV(usize),        // Vertical Percent
    PctHPO(usize, i16), // Horizontal Percent + Offset
    PctVPO(usize, i16), // Vertical Percent + Offsett
}

impl Unit {
    pub fn calc(&self) -> usize {
        match self {
            Unit::CoR(n) => *n,
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
        Unit::CoR(value)
    }
}

// Adding unlike orientations is not reccomended. Orientation of lhs takes precedence: H + V = H
impl std::ops::Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Unit::CoR(n) => match rhs {
                Unit::CoR(n2) => Unit::CoR(n + n2),
                Unit::PctH(n2) => Unit::PctHPO(n2, n as i16),
                Unit::PctV(n2) => Unit::PctVPO(n2, n as i16),
                Unit::PctHPO(n2, o2) => Unit::PctHPO(n2, n as i16 + o2),
                Unit::PctVPO(n2, o2) => Unit::PctVPO(n2, n as i16 + o2),
            },
            Unit::PctH(n) => match rhs {
                Unit::CoR(n2) => Unit::PctHPO(n, n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctH(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o2),
            },
            Unit::PctV(n) => match rhs {
                Unit::CoR(n2) => Unit::PctVPO(n, n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctV(n + n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n + n2, o2),
            },
            Unit::PctHPO(n, o) => match rhs {
                Unit::CoR(n2) => Unit::PctHPO(n, o + n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctHPO(n + n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n + n2, o + o2),
            },
            Unit::PctVPO(n, o) => match rhs {
                Unit::CoR(n2) => Unit::PctVPO(n, o + n2 as i16),
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
            Unit::CoR(n) => match rhs {
                Unit::CoR(n2) => Unit::CoR(n - n2),
                Unit::PctH(n2) => Unit::PctHPO(n2, -(n as i16)),
                Unit::PctV(n2) => Unit::PctVPO(n2, -(n as i16)),
                Unit::PctHPO(n2, o2) => Unit::PctHPO(n2, n as i16 - o2),
                Unit::PctVPO(n2, o2) => Unit::PctVPO(n2, n as i16 - o2),
            },
            Unit::PctH(n) => match rhs {
                Unit::CoR(n2) => Unit::PctHPO(n, -(n2 as i16)),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctH(n - n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n - n2, o2),
            },
            Unit::PctV(n) => match rhs {
                Unit::CoR(n2) => Unit::PctVPO(n, -(n2 as i16)),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctV(n - n2),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctVPO(n - n2, o2),
            },
            Unit::PctHPO(n, o) => match rhs {
                Unit::CoR(n2) => Unit::PctHPO(n, o - n2 as i16),
                Unit::PctH(n2) | Unit::PctV(n2) => Unit::PctHPO(n - n2, o),
                Unit::PctHPO(n2, o2) | Unit::PctVPO(n2, o2) => Unit::PctHPO(n - n2, o - o2),
            },
            Unit::PctVPO(n, o) => match rhs {
                Unit::CoR(n2) => Unit::PctVPO(n, o - n2 as i16),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_constructors() {
        let p = Point::new(3usize, 4usize);
        assert_eq!(p.x, Unit::CoR(3));
        assert_eq!(p.y, Unit::CoR(4));

        let p2: Point = (p, 2usize, 3usize).into();
        assert_eq!(p2.x, Unit::CoR(5));
        assert_eq!(p2.y, Unit::CoR(7));

        let pc = Point::cor(1, 2);
        assert_eq!(pc.x, Unit::CoR(1));
        assert_eq!(pc.y, Unit::CoR(2));

        let pp = Point::pct(10, 20);
        assert_eq!(pp.x, Unit::PctH(10));
        assert_eq!(pp.y, Unit::PctV(20));
    }

    #[test]
    fn unit_from_usize() {
        let u: Unit = 7usize.into();
        assert_eq!(u, Unit::CoR(7));
    }

    #[test]
    fn unit_addition_basic() {
        assert_eq!(Unit::CoR(5) + Unit::CoR(3), Unit::CoR(8));
        assert_eq!(Unit::CoR(5) + Unit::PctH(10), Unit::PctHPO(10, 5));
        assert_eq!(Unit::PctH(10) + Unit::CoR(5), Unit::PctHPO(10, 5));
        assert_eq!(Unit::PctH(10) + Unit::PctH(5), Unit::PctH(15));
        assert_eq!(Unit::PctH(10) + Unit::PctV(5), Unit::PctH(15));
        assert_eq!(Unit::PctV(20) + Unit::CoR(4), Unit::PctVPO(20, 4));
        assert_eq!(Unit::PctV(20) + Unit::PctV(5), Unit::PctV(25));
    }

    #[test]
    fn unit_addition_with_offsets() {
        assert_eq!(Unit::PctHPO(10, 2) + Unit::CoR(3), Unit::PctHPO(10, 5));
        assert_eq!(Unit::PctHPO(10, 2) + Unit::PctH(5), Unit::PctHPO(15, 2));
        assert_eq!(
            Unit::PctHPO(10, 2) + Unit::PctHPO(5, 3),
            Unit::PctHPO(15, 5)
        );
        assert_eq!(
            Unit::PctVPO(7, -2) + Unit::PctVPO(3, 5),
            Unit::PctVPO(10, 3)
        );
    }

    #[test]
    fn unit_subtraction_basic() {
        assert_eq!(Unit::CoR(8) - Unit::CoR(3), Unit::CoR(5));
        assert_eq!(Unit::CoR(10) - Unit::PctH(5), Unit::PctHPO(5, -10));
        assert_eq!(Unit::PctH(12) - Unit::CoR(2), Unit::PctHPO(12, -2));
        assert_eq!(Unit::PctH(15) - Unit::PctH(5), Unit::PctH(10));
        assert_eq!(Unit::PctV(20) - Unit::PctV(5), Unit::PctV(15));
    }

    #[test]
    fn unit_subtraction_with_offsets() {
        assert_eq!(Unit::PctHPO(10, 5) - Unit::CoR(3), Unit::PctHPO(10, 2));
        assert_eq!(Unit::PctHPO(10, 5) - Unit::PctH(4), Unit::PctHPO(6, 5));
        assert_eq!(Unit::PctHPO(10, 5) - Unit::PctHPO(3, 2), Unit::PctHPO(7, 3));
        assert_eq!(
            Unit::PctVPO(9, -3) - Unit::PctVPO(4, 5),
            Unit::PctVPO(5, -8)
        );
    }

    #[test]
    fn unit_add_assign_and_sub_assign() {
        let mut u = Unit::CoR(5);
        u += Unit::CoR(7);
        assert_eq!(u, Unit::CoR(12));
        u -= Unit::CoR(2);
        assert_eq!(u, Unit::CoR(10));
        u += Unit::PctH(10); // becomes offset pct
        assert_eq!(u, Unit::PctHPO(10, 10));
    }

    #[test]
    fn calc_cor_does_not_depend_on_term() {
        // Only test CoR variant to avoid relying on get_tsz()
        let u = Unit::CoR(42);
        assert_eq!(u.calc(), 42);
    }
}
