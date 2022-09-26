#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = 1;
        let mut ans = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                dp = dp + 1;
            } else {
                dp = 1;
            }
            ans = ans.max(dp);
        }
        ans
    }
}

#[cfg(test)]
mod tests_300 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
