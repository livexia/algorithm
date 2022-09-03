#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix = 0;
        let mut ans = nums[0];
        let mut least_prefix = 0;
        for num in nums {
            prefix = prefix + num;
            ans = ans.max(prefix - least_prefix);
            least_prefix = least_prefix.min(prefix);
        }
        ans
    }
}

#[cfg(test)]
mod tests_53 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
