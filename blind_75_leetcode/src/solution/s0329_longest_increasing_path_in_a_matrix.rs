#![allow(dead_code)]

use std::collections::VecDeque;
pub struct Solution {}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        // Solution::longest_increasing_path_dfs(matrix)
        Solution::longest_increasing_path_bfs(matrix)
        // Solution::longest_increasing_path_bfs2(matrix)
    }

    pub fn longest_increasing_path_bfs2(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut memo = vec![vec![0; n]; m];

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if memo[i][j] > 0 {
                    continue;
                }
                let mut queue = VecDeque::new();
                queue.push_back((i, j, 1));
                while let Some((x, y, count)) = queue.pop_front() {
                    if count <= memo[x][y] {
                        continue;
                    }
                    memo[x][y] = count;
                    res = res.max(count);
                    let cur = matrix[x][y];
                    if x > 0 && cur < matrix[x - 1][y] {
                        queue.push_back((x - 1, y, count + 1))
                    }
                    if x < m - 1 && cur < matrix[x + 1][y] {
                        queue.push_back((x + 1, y, count + 1))
                    }
                    if y > 0 && cur < matrix[x][y - 1] {
                        queue.push_back((x, y - 1, count + 1))
                    }
                    if y < n - 1 && cur < matrix[x][y + 1] {
                        queue.push_back((x, y + 1, count + 1))
                    }
                }
            }
        }
        res
    }

    pub fn longest_increasing_path_bfs(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut out_degrees = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let cur = matrix[i][j];
                if i > 0 && cur < matrix[i - 1][j] {
                    out_degrees[i][j] += 1;
                }
                if i < m - 1 && cur < matrix[i + 1][j] {
                    out_degrees[i][j] += 1;
                }
                if j > 0 && cur < matrix[i][j - 1] {
                    out_degrees[i][j] += 1;
                }
                if j < n - 1 && cur < matrix[i][j + 1] {
                    out_degrees[i][j] += 1;
                }
            }
        }

        let mut queue: VecDeque<(usize, usize)> = (0..m)
            .map(|i| (0..n).map(move |j| (i, j)))
            .flatten()
            .filter(|&(i, j)| out_degrees[i][j] == 0)
            .collect();

        let mut layer_number = 0;
        while !queue.is_empty() {
            layer_number += 1;
            let layer_size = queue.len();
            for _ in 0..layer_size {
                let (x, y) = queue.pop_front().unwrap();
                let cur = matrix[x][y];
                if x > 0 && cur > matrix[x - 1][y] {
                    Solution::add_to_next_layer(x - 1, y, &mut out_degrees, &mut queue)
                }
                if x < m - 1 && cur > matrix[x + 1][y] {
                    Solution::add_to_next_layer(x + 1, y, &mut out_degrees, &mut queue)
                }
                if y > 0 && cur > matrix[x][y - 1] {
                    Solution::add_to_next_layer(x, y - 1, &mut out_degrees, &mut queue)
                }
                if y < n - 1 && cur > matrix[x][y + 1] {
                    Solution::add_to_next_layer(x, y + 1, &mut out_degrees, &mut queue)
                }
            }
        }
        layer_number
    }

    fn add_to_next_layer(
        x: usize,
        y: usize,
        out_degrees: &mut [Vec<i32>],
        queue: &mut VecDeque<(usize, usize)>,
    ) {
        out_degrees[x][y] -= 1;
        if out_degrees[x][y] == 0 {
            queue.push_back((x, y));
        }
    }

    pub fn longest_increasing_path_dfs(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut max_length = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if max_length[i][j] == 0 {
                    ans = ans.max(Solution::dfs(i, j, &matrix, &mut max_length))
                }
            }
        }
        ans as i32
    }

    fn dfs(i: usize, j: usize, matrix: &[Vec<i32>], max_length: &mut [Vec<usize>]) -> usize {
        if max_length[i][j] != 0 {
            return max_length[i][j];
        }
        let cur = matrix[i][j];
        let mut res = 1;
        if i > 0 && cur < matrix[i - 1][j] {
            res = res.max(1 + Solution::dfs(i - 1, j, matrix, max_length));
        }
        if i < matrix.len() - 1 && cur < matrix[i + 1][j] {
            res = res.max(1 + Solution::dfs(i + 1, j, matrix, max_length));
        }
        if j > 0 && cur < matrix[i][j - 1] {
            res = res.max(1 + Solution::dfs(i, j - 1, matrix, max_length));
        }
        if j < matrix[0].len() - 1 && cur < matrix[i][j + 1] {
            res = res.max(1 + Solution::dfs(i, j + 1, matrix, max_length));
        }

        max_length[i][j] = res;
        res
    }
}

#[cfg(test)]
mod tests_329 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![[3, 4, 5], [3, 8, 6]]),
            5
        );
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![[0, 2, 9], [13, 11, 10]]),
            6
        );
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![[9, 9, 4], [6, 6, 8], [2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![[3, 4, 5], [3, 2, 6], [2, 2, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![
                [13, 5, 13, 9],
                [5, 0, 2, 9],
                [10, 13, 11, 10],
                [0, 0, 13, 13]
            ]),
            6
        );
        assert_eq!(Solution::longest_increasing_path(leetcode_vec![[1]]), 1);
        assert_eq!(
            Solution::longest_increasing_path(leetcode_vec![
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                [19, 18, 17, 16, 15, 14, 13, 12, 11, 10],
                [20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
                [39, 38, 37, 36, 35, 34, 33, 32, 31, 30],
                [40, 41, 42, 43, 44, 45, 46, 47, 48, 49],
                [59, 58, 57, 56, 55, 54, 53, 52, 51, 50],
                [60, 61, 62, 63, 64, 65, 66, 67, 68, 69],
                [79, 78, 77, 76, 75, 74, 73, 72, 71, 70],
                [80, 81, 82, 83, 84, 85, 86, 87, 88, 89],
                [99, 98, 97, 96, 95, 94, 93, 92, 91, 90],
                [100, 101, 102, 103, 104, 105, 106, 107, 108, 109],
                [119, 118, 117, 116, 115, 114, 113, 112, 111, 110],
                [120, 121, 122, 123, 124, 125, 126, 127, 128, 129],
                [139, 138, 137, 136, 135, 134, 133, 132, 131, 130],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]),
            140
        );
    }
}
