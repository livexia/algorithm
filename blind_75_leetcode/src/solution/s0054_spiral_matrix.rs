#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut left) = (0, 0);
        let (mut bottom, mut right) = (matrix.len(), matrix[0].len());
        let mut result = vec![];

        // counter clockwise direction
        // 0 -> right, 1 -> down, 2 -> left, 3 -> up
        let mut dir = 0;
        let mut cur = (0, 0);

        while top < bottom && left < right {
            let (x, y) = cur;
            match dir {
                0 => {
                    result.extend((left..right).map(|y| matrix[x][y]));
                    top += 1;
                    cur = (x, right - 1);
                }
                1 => {
                    result.extend((top..bottom).map(|x| matrix[x][y]));
                    right -= 1;
                    cur = (bottom - 1, y);
                }
                2 => {
                    result.extend((left..right).rev().map(|y| matrix[x][y]));
                    bottom -= 1;
                    cur = (x, left);
                }
                3 => {
                    result.extend((top..bottom).rev().map(|x| matrix[x][y]));
                    left += 1;
                    cur = (top, y);
                }
                _ => unreachable!(),
            }

            dir = (dir + 1) % 4;
        }
        result
    }
}

#[cfg(test)]
mod tests_54 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
