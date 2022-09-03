#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut prefix_sum = vec![0; l];
        prefix_sum[0] = nums[0];
        for i in 1..l {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }
        let mut ans = nums[0];
        let mut left = 0;
        for p in prefix_sum {
            ans = ans.max(p - left);
            left = left.min(p);
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
