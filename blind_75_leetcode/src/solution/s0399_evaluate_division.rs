#![allow(dead_code)]
pub struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Solution::calc_equation_dfs(equations, values, queries)
        Solution::calc_equation_bfs(equations, values, queries)
    }
    fn calc_equation_dfs(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adjacency_list: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        for (e, &v) in equations.iter().zip(values.iter()) {
            adjacency_list
                .entry(&e[0])
                .or_insert(vec![])
                .push((&e[1], v));
            adjacency_list
                .entry(&e[1])
                .or_insert(vec![])
                .push((&e[0], 1.0 / v));
        }

        let mut ans = vec![];
        for q in &queries {
            if !adjacency_list.contains_key(&q[0][..]) || !adjacency_list.contains_key(&q[1][..]) {
                ans.push(-1.0);
                continue;
            }
            let mut visited = HashSet::new();
            match Solution::dfs(&adjacency_list, &q[0], &q[1], &mut visited) {
                Ok(f) => ans.push(f),
                Err(_) => ans.push(-1.0),
            }
        }
        ans
    }

    fn dfs(
        adjacency_list: &HashMap<&str, Vec<(&str, f64)>>,
        dividend: &str,
        divisor: &str,
        visited: &mut HashSet<String>,
    ) -> Result<f64, ()> {
        visited.insert(dividend.to_string());
        for &(possible_d, v) in adjacency_list.get(&dividend).unwrap() {
            if possible_d == divisor {
                return Ok(v);
            } else {
                if visited.contains(possible_d) {
                    continue;
                }
                match Solution::dfs(adjacency_list, possible_d, divisor, visited) {
                    Ok(f) => return Ok(v * f),
                    Err(_) => (),
                }
            }
        }
        Err(())
    }

    fn calc_equation_bfs(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adjacency_list: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        for (e, &v) in equations.iter().zip(values.iter()) {
            adjacency_list
                .entry(&e[0])
                .or_insert(vec![])
                .push((&e[1], v));
            adjacency_list
                .entry(&e[1])
                .or_insert(vec![])
                .push((&e[0], 1.0 / v));
        }

        let mut ans = vec![-1.0; queries.len()];
        for i in 0..queries.len() {
            let q = &queries[i];
            if !adjacency_list.contains_key(&q[0][..]) || !adjacency_list.contains_key(&q[1][..]) {
                continue;
            }
            let mut queue = VecDeque::new();
            queue.push_back((&q[0][..], 1.0));

            let mut visited = HashSet::new();
            while let Some((dividend, res)) = queue.pop_front() {
                if visited.insert(dividend) {
                    for (divisor, f) in adjacency_list.get(&dividend[..]).unwrap() {
                        if divisor == &q[1] {
                            ans[i] = res * f;
                            queue.clear();
                        } else {
                            queue.push_back((divisor, res * f))
                        }
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_399 {
    use super::*;
    use crate::leetcode_vec;

    fn helper(input: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        input
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect()
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::calc_equation(
                helper(leetcode_vec![["a", "b"], ["c", "d"]]),
                vec![1.0, 1.0],
                helper(leetcode_vec![
                    ["a", "c"],
                    ["b", "d"],
                    ["b", "a"],
                    ["d", "c"]
                ])
            ),
            vec![-1.0, -1.0, 1.0, 1.0]
        );
        assert_eq!(
            Solution::calc_equation(
                helper(leetcode_vec![["a", "e"], ["b", "e"]]),
                vec![4.0, 3.0],
                helper(leetcode_vec![["a", "b"], ["e", "e"], ["x", "x"]])
            ),
            vec![1.3333333333333333, 1.0, -1.0]
        );
        assert_eq!(
            Solution::calc_equation(
                helper(leetcode_vec![["a", "b"], ["b", "c"]]),
                vec![2.0, 3.0],
                helper(leetcode_vec![
                    ["a", "c"],
                    ["b", "a"],
                    ["a", "e"],
                    ["a", "a"],
                    ["x", "x"]
                ])
            ),
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
        );
        assert_eq!(
            Solution::calc_equation(
                helper(leetcode_vec![["a", "b"], ["b", "c"], ["bc", "cd"]]),
                vec![1.5, 2.5, 5.0],
                helper(leetcode_vec![
                    ["a", "c"],
                    ["c", "b"],
                    ["bc", "cd"],
                    ["cd", "bc"]
                ])
            ),
            vec![3.75000, 0.40000, 5.00000, 0.20000]
        );
        assert_eq!(
            Solution::calc_equation(
                helper(leetcode_vec![["a", "b"]]),
                vec![0.5],
                helper(leetcode_vec![
                    ["a", "b"],
                    ["b", "a"],
                    ["a", "c"],
                    ["x", "y"]
                ])
            ),
            vec![0.50000, 2.00000, -1.00000, -1.00000]
        );
    }
}
