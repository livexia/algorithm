#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut adjacency_list = vec![vec![]; num_courses];
        for p in prerequisites {
            adjacency_list[p[1] as usize].push(p[0] as usize);
        }
        let mut ans = vec![];
        let mut visited = vec![0; num_courses];
        for i in 0..num_courses {
            if visited[i] == 0 {
                match Solution::dfs(i, &adjacency_list, &mut visited, &mut ans) {
                    Ok(_) => (),
                    Err(_) => return vec![],
                }
            }
        }
        ans.into_iter().rev().collect()
    }

    fn dfs(
        num: usize,
        adjacency_list: &[Vec<usize>],
        visited: &mut [u8],
        ans: &mut Vec<i32>,
    ) -> Result<(), ()> {
        visited[num] = 1;
        for &next in &adjacency_list[num] {
            if visited[next] == 2 {
                continue;
            } else if visited[next] == 1 {
                return Err(());
            } else {
                Solution::dfs(next, adjacency_list, visited, ans)?
            }
        }

        visited[num] = 2;
        ans.push(num as i32);
        Ok(())
    }
}

#[cfg(test)]
mod tests_210 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_order(2, leetcode_vec![[1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(2, leetcode_vec![[1, 0], [0, 1]]),
            vec![]
        );
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
        assert_eq!(
            Solution::find_order(4, leetcode_vec![[1, 0], [2, 0], [3, 1], [3, 2]]),
            vec![0, 2, 1, 3]
        );
    }
}
