#![allow(dead_code)]
pub struct Solution {}

// Rotate algorithm: https://stackoverflow.com/a/8664879
// Transpose algorithm: https://en.wikipedia.org/wiki/In-place_matrix_transposition#Square_matrices
impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        Solution::transpose(matrix);
        Solution::reverse_row(matrix);
    }

    fn transpose(matrix: &mut [Vec<i32>]) {
        for x in 0..matrix.len() {
            for y in x + 1..matrix.len() {
                // unstable feature
                // (matrix[x][y], matrix[y][x]) = (matrix[y][x], matrix[x][y]);
                let temp = matrix[x][y];
                matrix[x][y] = matrix[y][x];
                matrix[y][x] = temp;
            }
        }
    }

    fn reverse_row(matrix: &mut [Vec<i32>]) {
        for row in matrix.iter_mut() {
            row.reverse()
        }
    }
}

#[cfg(test)]
mod tests_48 {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
