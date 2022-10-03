#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut count = 0;
        let mut union_find = vec![];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    union_find.push(i * n + j);
                    count += 1;
                } else {
                    union_find.push(m * n);
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    grid[i][j] = '0';
                    if i > 0 && grid[i - 1][j] == '1' {
                        Solution::union(&mut union_find, i * n + j, (i - 1) * n + j, &mut count)
                    }
                    if i < m - 1 && grid[i + 1][j] == '1' {
                        Solution::union(&mut union_find, i * n + j, (i + 1) * n + j, &mut count)
                    }
                    if j > 0 && grid[i][j - 1] == '1' {
                        Solution::union(&mut union_find, i * n + j, i * n + j - 1, &mut count)
                    }
                    if j < n - 1 && grid[i][j + 1] == '1' {
                        Solution::union(&mut union_find, i * n + j, i * n + j + 1, &mut count)
                    }
                }
            }
        }
        count
    }

    fn find(union_find: &mut [usize], a: usize) -> usize {
        if union_find[a] != a {
            union_find[a] = Solution::find(union_find, union_find[a])
        }
        union_find[a]
    }

    fn union(union_find: &mut [usize], a: usize, b: usize, count: &mut i32) {
        let root_a = Solution::find(union_find, a);
        let root_b = Solution::find(union_find, b);
        if root_a != root_b {
            union_find[root_b] = root_a;
            *count -= 1;
        }
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
