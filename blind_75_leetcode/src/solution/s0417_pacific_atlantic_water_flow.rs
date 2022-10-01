#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut status = vec![vec![0u8; n]; m];
        status[0] = vec![1; n];
        (0..m).for_each(|i| status[i][0] = 1);
        status[m - 1] = status[m - 1].iter().map(|&v| v | 2).collect();
        (0..m).for_each(|i| status[i][n - 1] |= 2);

        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    Solution::dfs(i, j, &heights, &mut visited, &mut status);
                }
            }
        }

        let mut ans = vec![];
        for i in 0..m {
            for j in 0..n {
                if status[i][j] == 3 {
                    ans.push(vec![i as i32, j as i32])
                }
            }
        }
        ans
    }

    fn dfs(
        x: usize,
        y: usize,
        heights: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        status: &mut [Vec<u8>],
    ) -> u8 {
        if visited[x][y] {
            return status[x][y];
        }
        let h = heights[x][y];
        visited[x][y] = true;
        if x > 0 && h >= heights[x - 1][y] {
            status[x][y] |= Solution::dfs(x - 1, y, heights, visited, status)
        }
        if y > 0 && h >= heights[x][y - 1] {
            status[x][y] |= Solution::dfs(x, y - 1, heights, visited, status)
        }
        if x < heights.len() - 1 && h >= heights[x + 1][y] {
            status[x][y] |= Solution::dfs(x + 1, y, heights, visited, status)
        }
        if y < heights[0].len() - 1 && h >= heights[x][y + 1] {
            status[x][y] |= Solution::dfs(x, y + 1, heights, visited, status)
        }
        if status[x][y] != 3 {
            visited[x][y] = false
        }
        status[x][y]
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
            Solution::pacific_atlantic(leetcode_vec![[1, 1], [1, 1], [1, 1]]),
            leetcode_vec![[0, 0], [0, 1], [1, 0], [1, 1], [2, 0], [2, 1]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![[1]]),
            leetcode_vec![[0, 0]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![[1, 2, 3], [8, 9, 4], [7, 6, 5]]),
            leetcode_vec![[0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]
        );
        assert_eq!(
            Solution::pacific_atlantic(leetcode_vec![
                [3, 3, 3, 3, 3, 3],
                [3, 0, 3, 3, 0, 3],
                [3, 3, 3, 3, 3, 3]
            ]),
            leetcode_vec![
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [0, 5],
                [1, 0],
                [1, 2],
                [1, 3],
                [1, 5],
                [2, 0],
                [2, 1],
                [2, 2],
                [2, 3],
                [2, 4],
                [2, 5]
            ]
        );
    }
}
