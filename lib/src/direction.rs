use crate::pos::Pos;
use core::fmt;
use std::ops::Mul;

/// Direction represents a vector with a length if 1 represented as "direction" in a
/// cartesian coordinate system.
///
/// Because this primarily used for Advent of Code puzzle solutions, grids mostly
/// are represented as a coordinate system from top to bottom, so visually going
/// "up" means actually going negative in the y-axis. In example, going from
/// A(2,2) "down" to B(2,5) increases the y-value by 3.
/// ```plain
/// 0 1 2 3 ->
/// 1
/// 2   A
/// 3   |
/// 4   v
/// 5   B
/// 6
/// |
/// v
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns an array of all available directions.
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }

    /// Reverses a direction.
    ///
    /// # Example
    /// ```
    /// # use lib::Direction;
    /// assert_eq!(Direction::Up.reverse(), Direction::Down);
    /// assert_eq!(Direction::Down.reverse(), Direction::Up);
    /// assert_eq!(Direction::Left.reverse(), Direction::Right);
    /// assert_eq!(Direction::Right.reverse(), Direction::Left);
    /// ```
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Turns the direction clockwise.
    ///
    /// # Example
    /// ```
    /// # use lib::Direction;
    /// assert_eq!(Direction::Up.turn_cw(), Direction::Right);
    /// assert_eq!(Direction::Down.turn_cw(), Direction::Left);
    /// assert_eq!(Direction::Left.turn_cw(), Direction::Up);
    /// assert_eq!(Direction::Right.turn_cw(), Direction::Down);
    /// ```
    pub fn turn_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    /// Tuns the direction counter-clockwise.
    ///
    /// # Example
    /// ```
    /// # use lib::Direction;
    /// assert_eq!(Direction::Up.turn_ccw(), Direction::Left);
    /// assert_eq!(Direction::Down.turn_ccw(), Direction::Right);
    /// assert_eq!(Direction::Left.turn_ccw(), Direction::Down);
    /// assert_eq!(Direction::Right.turn_ccw(), Direction::Up);
    pub fn turn_ccw(&self) -> Direction {
        self.turn_cw().reverse()
    }
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

impl From<Direction> for Pos {
    fn from(v: Direction) -> Self {
        match v {
            Direction::Up => Pos { x: 0, y: -1 },
            Direction::Down => Pos { x: 0, y: 1 },
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
        }
    }
}

impl Mul<isize> for Direction {
    type Output = Pos;

    fn mul(self, rhs: isize) -> Self::Output {
        Pos::from(self) * Pos { x: rhs, y: rhs }
    }
}

impl Mul<usize> for Direction {
    type Output = Pos;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as isize
    }
}
