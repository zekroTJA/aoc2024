use crate::pos::Pos;
use core::fmt;
use std::ops::Mul;

/// Direction represents a vector with a length if 1 represented as "direction" in a
/// cartesian coordinate system.
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
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
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
            Direction::Up => Pos { x: 0, y: 1 },
            Direction::Down => Pos { x: 0, y: -1 },
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
