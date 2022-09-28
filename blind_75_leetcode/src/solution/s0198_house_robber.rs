#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        dp[2] = nums[0];

        for (i, num) in nums.iter().enumerate().skip(1) {
            dp[i + 2] = dp[i + 1].max(dp[i] + num);
        }

        dp[nums.len() + 1]
    }
}

#[cfg(test)]
mod tests_198 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![1, 1]), 1);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
