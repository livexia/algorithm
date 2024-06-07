#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut flag_row = false;
        let mut flag_col = false;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                    if i == 0 {
                        flag_row = true
                    }
                    if j == 0 {
                        flag_col = true
                    }
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0
                }
            }
        }
        if flag_row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        if flag_col {
            for row in matrix.iter_mut() {
                row[0] = 0;
            }
        }
    }

    pub fn set_zeroes_with_one_var(matrix: &mut [Vec<i32>]) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut flag_col = false;
        for i in 0..m {
            if matrix[i][0] == 0 {
                flag_col = true;
            }

            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in (0..m).rev() {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if flag_col {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests_73 {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );

        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes_with_one_var(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
