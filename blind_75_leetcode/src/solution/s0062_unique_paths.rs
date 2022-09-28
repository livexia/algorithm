#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m.max(n) as usize, n.min(m) as usize);
        let mut cur = vec![1; n];
        for _ in 1..m {
            for j in 1..n {
                cur[j] += cur[j - 1]
            }
        }
        cur[n - 1]
    }
}

#[cfg(test)]
mod tests_62 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
