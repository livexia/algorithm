#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];

        for (i, num) in nums.iter().enumerate() {
            dp[i + 2] = dp[..=i].iter().max().unwrap() + num;
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests_198 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
