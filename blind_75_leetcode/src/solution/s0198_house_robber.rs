#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut before_last_max = 0;
        let mut last_max = nums[0];

        for num in &nums[1..] {
            let temp = before_last_max;
            before_last_max = before_last_max.max(last_max);
            last_max = temp + num;
        }

        before_last_max.max(last_max)
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
