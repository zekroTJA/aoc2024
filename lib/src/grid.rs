use crate::Pos;

/// Takes a string input and transforms it into a grid by using each
/// line as row and character in line as column.
pub fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

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

pub fn get_at<T>(grid: &[Vec<T>], pos: Pos) -> Option<&T> {
    if pos.is_any_negative() || pos.y >= grid.len() as isize || pos.x >= grid[0].len() as isize {
        None
    } else {
        Some(&grid[pos.y as usize][pos.x as usize])
    }
}

/// Takes a matrix flips rows to colums and colums to rows.
///
/// Example:
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
