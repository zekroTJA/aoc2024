use crate::Pos;

#[macro_export]
macro_rules! grid {
    ( $( $( $v:expr),+);+ $(;)? ) => {
        vec![
            $( vec![ $($v),+ ] ),+
        ]
    };
}

/// Takes a string input and transforms it into a grid by using each
/// line as row and character in line as column.
///
/// # Example
/// ```
/// # use lib::{to_grid, grid};
/// let input = "\
/// 1234
/// 5678
/// 9012";
/// let grid = to_grid(input);
/// assert_eq!(grid, grid!{
///     '1', '2', '3', '4';
///     '5', '6', '7', '8';
///     '9', '0', '1', '2';
/// });
/// ```
pub fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Tries to find the first occurence of `v` in the `grid`.
///
/// # Example
/// ```
/// # use lib::{Pos, find, grid};
/// let grid = grid! {
///     1, 2, 3;
///     4, 5, 6;
/// };
/// assert_eq!(find(&grid, &3), Some(Pos { x: 2, y: 0 }));
/// assert_eq!(find(&grid, &7), None);
/// ```
pub fn find<T: PartialEq>(grid: &[Vec<T>], v: &T) -> Option<Pos> {
    for (y, row) in grid.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if e == v {
                return Some((x as isize, y as isize).into());
            }
        }
    }
    None
}

/// Tries to find all occurences of `v` in the `grid` and returns the
/// found positions as a list.
///
/// # Example
/// ```
/// # use lib::{Pos, find_all, grid};
/// let grid = grid! {
///     1, 2, 3;
///     2, 4, 2;
/// };
/// assert_eq!(find_all(&grid, &2), vec![
///     Pos { x: 1, y: 0 },
///     Pos { x: 0, y: 1 },
///     Pos { x: 2, y: 1 },
/// ]);
/// assert!(find_all(&grid, &7).is_empty());
/// ```
pub fn find_all<T: PartialEq>(grid: &[Vec<T>], v: &T) -> Vec<Pos> {
    let mut res = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if e == v {
                res.push((x as isize, y as isize).into())
            }
        }
    }
    res
}

/// Returns true if the given position is inside the coordinate bounds of
/// the given grid.
///
/// # Example
/// ```
/// # use lib::{Pos, is_in_bounds, grid};
/// let grid = grid! {
///     1, 2, 3;
///     2, 4, 2;
/// };
/// assert!(is_in_bounds(&grid, Pos { x: 0, y: 0 }));
/// assert!(is_in_bounds(&grid, Pos { x: 1, y: 0 }));
/// assert!(is_in_bounds(&grid, Pos { x: 2, y: 1 }));
///
/// assert!(!is_in_bounds(&grid, Pos { x: 3, y: 1 }));
/// assert!(!is_in_bounds(&grid, Pos { x: 1, y: 2 }));
/// assert!(!is_in_bounds(&grid, Pos { x: -1, y: 0 }));
/// assert!(!is_in_bounds(&grid, Pos { x: 0, y: -1 }));
/// ```
pub fn is_in_bounds<T>(grid: &[Vec<T>], pos: Pos) -> bool {
    !pos.is_any_negative() && pos.y < grid.len() as isize && pos.x < grid[0].len() as isize
}

/// Returns the value at the given position in the grid, if the
/// position is in the coordinate bounds of the grid.
///
/// # Example
/// ```
/// # use lib::{Pos, get_at, grid};
/// let grid = grid! {
///     1, 2, 3;
///     2, 4, 2;
/// };
/// assert_eq!(get_at(&grid, Pos { x: 0, y: 0 }), Some(&1));
/// assert_eq!(get_at(&grid, Pos { x: 1, y: 1 }), Some(&4));
/// assert_eq!(get_at(&grid, Pos { x: 1, y: 2 }), None);
/// assert_eq!(get_at(&grid, Pos { x: -1, y: 0 }), None);
/// ```
pub fn get_at<T>(grid: &[Vec<T>], pos: Pos) -> Option<&T> {
    if !is_in_bounds(grid, pos) {
        None
    } else {
        Some(&grid[pos.y as usize][pos.x as usize])
    }
}

/// Takes a matrix flips rows to colums and colums to rows.
///
/// # Example
/// ```
/// # use lib::flip_grid;
/// let res = flip_grid(&[vec![1, 2], vec![3, 4]]);
/// assert_eq!(res, vec![vec![1, 3], vec![2, 4]]);
/// ```
pub fn flip_grid<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!grid.is_empty(), "Matrix has no rows");
    for line in grid {
        assert_eq!(
            line.len(),
            grid[0].len(),
            "Matrix contains rows with different lengths"
        );
    }

    let mut new = vec![];

    for x in 0..grid[0].len() {
        let mut row = vec![];
        for line in grid {
            row.push(line[x])
        }
        new.push(row);
    }

    new
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filp_grid() {
        assert_eq!(flip_grid(&[vec![1, 2]]), vec![vec![1], vec![2]]);
        assert_eq!(
            flip_grid(&[vec![1, 2], vec![3, 4]]),
            vec![vec![1, 3], vec![2, 4]]
        );
    }

    #[test]
    #[should_panic]
    fn test_flip_grid_panic_empty() {
        flip_grid::<i32>(&[]);
    }

    #[test]
    #[should_panic]
    fn test_flip_grid_panic_row_length_missmatch() {
        flip_grid(&[vec![1, 2], vec![3]]);
    }
}
