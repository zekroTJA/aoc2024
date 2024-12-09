use crate::{vector::Vector, Direction};
use core::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Position in a two dimensional cartesian coordinate system.
#[derive(Clone, Copy, Hash, Eq, PartialEq, Default, Debug, PartialOrd, Ord)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    /// Returns true if either the x or y component of the point is negative.
    ///
    /// # Example
    /// ```
    /// # use lib::Pos;
    /// let pos1 = Pos { x: 1, y: 1 };
    /// assert!(!pos1.is_any_negative());
    ///
    /// let pos2 = Pos { x: -1, y: 1 };
    /// assert!(pos2.is_any_negative());
    /// ```
    pub fn is_any_negative(&self) -> bool {
        self.x.is_negative() || self.y.is_negative()
    }

    /// Returns true if either the x or y component of the point is zero.
    ///
    /// # Example
    /// ```
    /// # use lib::Pos;
    /// let pos1 = Pos { x: 0, y: 1 };
    /// assert!(!pos1.is_zero());
    ///
    /// let pos2 = Pos { x: 0, y: 0 };
    /// assert!(pos2.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Returns the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) from the
    /// current point to Point `to`.
    ///
    /// # Example
    /// ```
    /// # use lib::Pos;
    /// let pos1 = Pos { x: 1, y: 2};
    /// let pos2 = Pos { x: 4, y: 3 };
    ///
    /// assert_eq!(pos1.manhattan_distance(pos2), 4);
    /// ```
    pub fn manhattan_distance(&self, to: Pos) -> usize {
        (to.y - self.y).unsigned_abs() + (to.x - self.x).unsigned_abs()
    }

    /// "Walks" the point into the given direction by one unit.
    ///
    /// # Example
    /// ```
    /// # use lib::{Pos, Direction};
    /// let pos = Pos { x: 1, y: 2};
    ///
    /// assert_eq!(pos.mv(Direction::Down), Pos { x: 1, y: 3 });
    /// ```
    pub fn mv(self, dir: Direction) -> Self {
        self + dir.into()
    }

    /// Seen the point as a vector from (0, 0), this function returns
    /// the vector turned 90° counter-clockwise.
    ///
    /// # Example
    /// ```
    /// # use lib::{Pos, Direction};
    /// let pos = Pos { x: 1, y: 2};
    ///
    /// assert_eq!(pos.turn_cw(), Pos { x: 2, y: -1 });
    /// ```
    pub fn turn_cw(&self) -> Pos {
        Pos {
            x: self.y,
            y: -self.x,
        }
    }

    /// Seen the point as a vector from (0, 0), this function returns
    /// the vector turned 90° counter-clockwise.
    ///
    /// # Example
    /// ```
    /// # use lib::{Pos, Direction};
    /// let pos = Pos { x: 1, y: 2};
    ///
    /// assert_eq!(pos.turn_ccw(), Pos { x: -2, y: 1 });
    /// ```
    pub fn turn_ccw(&self) -> Pos {
        Pos {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Vector for Pos {
    type Output = Pos;

    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt().abs()
    }

    fn flatten(&self) -> Self::Output {
        let (mut x, mut y) = (self.x, self.y);
        if x != 0 {
            x /= x.abs();
        }
        if y != 0 {
            y /= y.abs();
        }
        (x, y).into()
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Pos {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x, self.y * rhs.y).into()
    }
}

impl MulAssign for Pos {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<isize>,
{
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
