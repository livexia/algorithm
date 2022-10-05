#![allow(dead_code)]
pub struct Solution {}

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut vars = HashMap::new();
        let mut index = 0usize;
        for e in &equations {
            if !vars.contains_key(&e[0]) {
                vars.insert(&e[0], index);
                index += 1;
            }
            if !vars.contains_key(&e[1]) {
                vars.insert(&e[1], index);
                index += 1;
            }
        }
        let mut adjacency_list: Vec<Vec<(usize, f64)>> = vec![vec![]; index];
        for (e, &v) in equations.iter().zip(values.iter()) {
            let &va = vars.get(&e[0]).unwrap();
            let &vb = vars.get(&e[1]).unwrap();
            adjacency_list[va].push((vb, v));
            adjacency_list[vb].push((va, 1.0 / v));
        }
        // Solution::calc_equation_dfs(&vars, &adjacency_list, queries)
        Solution::calc_equation_bfs(&vars, &adjacency_list, queries)
        // Solution::union_find(&vars, &equations, values, queries)
        // Solution::floyd(&vars, &equations, values, queries)
    }

    fn calc_equation_dfs(
        vars: &HashMap<&String, usize>,
        adjacency_list: &[Vec<(usize, f64)>],
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut ans = vec![];
        for q in &queries {
            if !vars.contains_key(&q[0]) || !vars.contains_key(&q[1]) {
                ans.push(-1.0);
                continue;
            }
            let mut visited = vec![false; vars.len()];
            match Solution::dfs(
                &adjacency_list,
                *vars.get(&q[0]).unwrap(),
                *vars.get(&q[1]).unwrap(),
                &mut visited,
            ) {
                Ok(f) => ans.push(f),
                Err(_) => ans.push(-1.0),
            }
        }
        ans
    }

    fn dfs(
        adjacency_list: &[Vec<(usize, f64)>],
        dividend: usize,
        divisor: usize,
        visited: &mut [bool],
    ) -> Result<f64, ()> {
        visited[dividend] = true;
        for &(possible_d, v) in &adjacency_list[dividend] {
            if possible_d == divisor {
                return Ok(v);
            } else {
                if visited[possible_d] {
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
        vars: &HashMap<&String, usize>,
        adjacency_list: &[Vec<(usize, f64)>],
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut ans = vec![-1.0; queries.len()];
        for i in 0..queries.len() {
            let q = &queries[i];
            if !vars.contains_key(&q[0]) || !vars.contains_key(&q[1]) {
                continue;
            }
            let mut queue = VecDeque::new();
            queue.push_back((vars.get(&q[0]).unwrap(), 1.0));

            let target = vars.get(&q[1]).unwrap();
            let mut visited = vec![false; vars.len()];
            while let Some((&dividend, res)) = queue.pop_front() {
                if !visited[dividend] {
                    visited[dividend] = true;
                    for (divisor, f) in &adjacency_list[dividend] {
                        if divisor == target {
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

    fn union_find(
        vars: &HashMap<&String, usize>,
        equations: &Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut parent: Vec<usize> = (0..vars.len()).collect();
        let mut weight = vec![1.0; vars.len()];
        let mut rank = vec![0; vars.len()];

        for (e, v) in equations.iter().zip(values.iter()) {
            let &va = vars.get(&e[0]).unwrap();
            let &vb = vars.get(&e[1]).unwrap();
            Solution::union(va, vb, *v, &mut parent, &mut weight, &mut rank);
        }
        let mut ans = vec![-1.0; queries.len()];
        for i in 0..queries.len() {
            if !vars.contains_key(&queries[i][0]) || !vars.contains_key(&queries[i][1]) {
                continue;
            }
            let &va = vars.get(&queries[i][0]).unwrap();
            let &vb = vars.get(&queries[i][1]).unwrap();
            let dividend = Solution::find(va, &mut parent, &mut weight);
            let divisor = Solution::find(vb, &mut parent, &mut weight);
            if dividend == divisor {
                ans[i] = weight[va] / weight[vb];
            }
        }
        ans
    }

    fn find(a: usize, parent: &mut [usize], weight: &mut [f64]) -> usize {
        if a != parent[a] {
            let father = Solution::find(parent[a], parent, weight);
            weight[a] *= weight[parent[a]];
            parent[a] = father;
        }
        parent[a]
    }

    fn union(
        dividend: usize,
        divisor: usize,
        value: f64,
        parent: &mut [usize],
        weight: &mut [f64],
        rank: &mut [usize],
    ) {
        let mut dividend_root = Solution::find(dividend, parent, weight);
        let mut divisor_root = Solution::find(divisor, parent, weight);
        let mut value = value;
        let mut dividend = dividend;
        let mut divisor = divisor;
        if dividend_root != divisor_root {
            if rank[divisor_root] < rank[dividend_root] {
                use std::mem::swap;
                swap(&mut dividend_root, &mut divisor_root);
                swap(&mut dividend, &mut divisor);
                value = 1.0 / value;
            }
            parent[dividend_root] = divisor_root;
            weight[dividend_root] = weight[divisor] * value / weight[dividend];
            if rank[dividend_root] == rank[divisor_root] {
                rank[divisor_root] += 1;
            }
        }
    }

    fn floyd(
        vars: &HashMap<&String, usize>,
        equations: &Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut dp = vec![vec![-1.0; vars.len()]; vars.len()];
        for (e, &v) in equations.iter().zip(values.iter()) {
            let &va = vars.get(&e[0]).unwrap();
            let &vb = vars.get(&e[1]).unwrap();
            dp[va][vb] = v;
            dp[vb][va] = 1.0 / v;
        }
        let num_vars = vars.len();
        for k in 0..num_vars {
            for i in 0..num_vars {
                for j in 0..num_vars {
                    if dp[i][k] > 0.0 && dp[k][j] > 0.0 {
                        dp[i][j] = dp[i][k] * dp[k][j]
                    }
                }
            }
        }
        let mut ans = vec![-1.0; queries.len()];
        for i in 0..queries.len() {
            if !vars.contains_key(&queries[i][0]) || !vars.contains_key(&queries[i][1]) {
                continue;
            }
            let &va = vars.get(&queries[i][0]).unwrap();
            let &vb = vars.get(&queries[i][1]).unwrap();
            ans[i] = dp[va][vb]
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
                helper(leetcode_vec![
                    ["x1", "x2"],
                    ["x2", "x3"],
                    ["x3", "x4"],
                    ["x4", "x5"]
                ]),
                vec![3.0, 4.0, 5.0, 6.0],
                helper(leetcode_vec![
                    ["x1", "x5"],
                    ["x5", "x2"],
                    ["x2", "x4"],
                    ["x2", "x2"],
                    ["x2", "x9"],
                    ["x9", "x9"]
                ])
            ),
            vec![360.0, 0.00833, 20.0, 1.0, -1.0, -1.0]
        );
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
