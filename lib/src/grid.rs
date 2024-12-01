/// Takes a matrix flips rows to colums and colums to rows.
///
/// Example:
/// ```
/// # use lib::flip_matrix;
/// let res = flip_matrix(&[vec![1, 2], vec![3, 4]]);
/// assert_eq!(res, vec![vec![1, 3], vec![2, 4]]);
/// ```
pub fn flip_matrix<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!matrix.is_empty(), "Matrix has no rows");
    for line in matrix {
        assert_eq!(
            line.len(),
            matrix[0].len(),
            "Matrix contains rows with different lengths"
        );
    }

    let mut new = vec![];

    for x in 0..matrix[0].len() {
        let mut row = vec![];
        for line in matrix {
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
    fn test_filp_matrix() {
        assert_eq!(flip_matrix(&[vec![1, 2]]), vec![vec![1], vec![2]]);
        assert_eq!(
            flip_matrix(&[vec![1, 2], vec![3, 4]]),
            vec![vec![1, 3], vec![2, 4]]
        );
    }

    #[test]
    #[should_panic]
    fn test_flip_matrix_panic_empty() {
        flip_matrix::<i32>(&[]);
    }

    #[test]
    #[should_panic]
    fn test_flip_matrix_panic_row_length_missmatch() {
        flip_matrix(&[vec![1, 2], vec![3]]);
    }
}
