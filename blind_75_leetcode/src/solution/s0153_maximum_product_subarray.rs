#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut dp = (1, 1);
        let mut ans = nums[0];
        for num in nums {
            dp = (
                num.min(dp.0 * num).min(dp.1 * num),
                num.max(dp.0 * num).max(dp.1 * num),
            );
            ans = ans.max(dp.1);
        }
        ans
    }
}

#[cfg(test)]
mod tests_153 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-2]), -2);
        assert_eq!(Solution::max_product(vec![0, 2]), 2);
        assert_eq!(Solution::max_product(vec![-3, 0, 1, -2]), 1);
        assert_eq!(Solution::max_product(vec![0, 1, 0]), 1);
    }
}
