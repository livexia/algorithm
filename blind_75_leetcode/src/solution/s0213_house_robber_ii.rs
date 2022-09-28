#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        Solution::no_cycle_rob(&nums[..nums.len() - 1]).max(Solution::no_cycle_rob(&nums[1..]))
    }

    fn no_cycle_rob(nums: &[i32]) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut before_last_max = nums[0];
        let mut last_max = nums[1].max(before_last_max);
        for num in &nums[2..] {
            let temp = last_max;
            last_max = last_max.max(before_last_max + num);
            before_last_max = temp;
        }
        last_max
    }
}

#[cfg(test)]
mod tests_213 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
}
