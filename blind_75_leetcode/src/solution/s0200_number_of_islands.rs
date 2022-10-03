#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut islands = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    islands.insert((i as i32, j as i32));
                }
            }
        }
        let mut count = 0;
        let mut visited = HashSet::new();
        for &(i, j) in &islands {
            if !visited.contains(&(i, j)) {
                count += 1;
                Solution::dfs(i, j, &islands, &mut visited);
            }
        }
        count
    }

    fn dfs(i: i32, j: i32, islands: &HashSet<(i32, i32)>, visited: &mut HashSet<(i32, i32)>) {
        if visited.insert((i, j)) && islands.contains(&(i, j)) {
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dx, dy) in dirs {
                if islands.contains(&(i + dx, j + dy)) {
                    Solution::dfs(i + dx, j + dy, islands, visited)
                }
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
