#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // (1..=m + n - 2).product::<i32>()
        //     / ((1..=m - 1).product::<i32>() * (1..=n - 1).product::<i32>())
        // (n..=m + n - 2).product::<i32>() / (1..=m - 1).product::<i32>()
        let (m, n) = (m.min(n) as u64, n.max(m) as u64);
        (1..m).fold(1, |ans, k| ans * (n + k - 1) / k) as i32
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
        assert_eq!(Solution::unique_paths(10, 10), 48620);
        assert_eq!(Solution::unique_paths(23, 12), 193536720);
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
    }
}
