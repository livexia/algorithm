#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    // with dfs
                    // Solution::dfs(i as i32, j as i32, &mut grid, m as i32, n as i32);

                    // with bfs
                    use std::collections::VecDeque;
                    let mut queue = VecDeque::new();
                    queue.push_back((i, j));
                    while let Some((x, y)) = queue.pop_front() {
                        if grid[x][y] == '0' {
                            continue;
                        }
                        grid[x][y] = '0';

                        if x > 0 && grid[x - 1][y] == '1' {
                            queue.push_back((x - 1, y))
                        }
                        if x < m - 1 && grid[x + 1][y] == '1' {
                            queue.push_back((x + 1, y))
                        }
                        if y > 0 && grid[x][y - 1] == '1' {
                            queue.push_back((x, y - 1))
                        }
                        if y < n - 1 && grid[x][y + 1] == '1' {
                            queue.push_back((x, y + 1))
                        }
                    }
                }
            }
        }
        count
    }

    fn dfs(i: i32, j: i32, grid: &mut [Vec<char>], m: i32, n: i32) {
        if i >= 0 && i < m && j >= 0 && j < n && grid[i as usize][j as usize] == '1' {
            grid[i as usize][j as usize] = '0';
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dx, dy) in dirs {
                Solution::dfs(i + dx, j + dy, grid, m, n)
            }
        }
    }
}

#[cfg(test)]
mod tests_200 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::num_islands(leetcode_vec![
                ['1', '1', '1', '1', '0'],
                ['1', '1', '0', '1', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(leetcode_vec![
                ['1', '1', '0', '0', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '1', '0', '0'],
                ['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
