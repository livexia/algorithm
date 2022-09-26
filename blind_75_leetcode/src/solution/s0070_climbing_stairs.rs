#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut dp = vec![1, 1];
        let n = n as usize;
        for i in 2..=n {
            dp.push(dp[i - 1] + dp[i - 2]);
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests_70 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
