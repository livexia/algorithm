#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut visited = vec![vec![0; n]; m];
        visited[0] = vec![1; n];
        (0..m).for_each(|i| visited[i][0] = 1);

        visited[m - 1] = visited[m - 1].iter().map(|&v| v | 2).collect();
        (0..m).for_each(|i| visited[i][n - 1] |= 2);
        for i in 1..m {
            for j in 1..n {
                if heights[i][j] >= heights[i - 1][j] {
                    visited[i][j] |= visited[i - 1][j];
                }
                if heights[i][j] >= heights[i][j - 1] {
                    visited[i][j] |= visited[i][j - 1];
                }
            }
        }
        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                if heights[i][j] >= heights[i + 1][j] {
                    visited[i][j] |= visited[i + 1][j];
                }
                if heights[i][j] >= heights[i][j + 1] {
                    visited[i][j] |= visited[i][j + 1];
                }
            }
        }

        let mut ans = vec![];
        for i in 0..m {
            for j in 0..n {
                if visited[i][j] == 3 {
                    ans.push(vec![i as i32, j as i32])
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_417 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![
                [1, 2, 2, 3, 5],
                [3, 2, 3, 4, 4],
                [2, 4, 5, 3, 1],
                [6, 7, 1, 4, 5],
                [5, 1, 1, 2, 4]
            ]),
            leetcode_vec![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![[2, 1], [1, 2]]),
            leetcode_vec![[0, 0], [0, 1], [1, 0], [1, 1]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![[1]]),
            leetcode_vec![[0, 0]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![[1, 2, 3], [8, 9, 4], [7, 6, 5]]),
            leetcode_vec![[0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]
        );
    }
}
