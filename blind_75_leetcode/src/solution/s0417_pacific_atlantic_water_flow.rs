#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut pacific_visited = vec![vec![false; n]; m];
        for (i, j) in (0..n).map(|j| (0, j)).chain((1..m).map(|i| (i, 0))) {
            Solution::dfs(i, j, &heights, &mut pacific_visited);
        }
        let mut atlantic_visited = vec![vec![false; n]; m];
        for (i, j) in (0..n)
            .map(|j| (m - 1, j))
            .chain((0..m - 1).map(|i| (i, n - 1)))
        {
            Solution::dfs(i, j, &heights, &mut atlantic_visited);
        }

        let mut ans = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific_visited[i][j] && atlantic_visited[i][j] {
                    ans.push(vec![i as i32, j as i32])
                }
            }
        }
        ans
    }

    fn dfs(x: usize, y: usize, heights: &[Vec<i32>], visited: &mut [Vec<bool>]) {
        if visited[x][y] {
            return;
        }
        let h = heights[x][y];
        visited[x][y] = true;
        if x > 0 && h <= heights[x - 1][y] {
            Solution::dfs(x - 1, y, heights, visited)
        }
        if y > 0 && h <= heights[x][y - 1] {
            Solution::dfs(x, y - 1, heights, visited)
        }
        if x < heights.len() - 1 && h <= heights[x + 1][y] {
            Solution::dfs(x + 1, y, heights, visited)
        }
        if y < heights[0].len() - 1 && h <= heights[x][y + 1] {
            Solution::dfs(x, y + 1, heights, visited)
        }
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
