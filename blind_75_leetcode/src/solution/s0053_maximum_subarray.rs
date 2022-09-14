#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // solution from problem 217
        // let mut prefix = 0;
        // let mut ans = nums[0];
        // let mut least_prefix = 0;
        // for num in nums {
        //     prefix = prefix + num;
        //     ans = ans.max(prefix - least_prefix);
        //     least_prefix = least_prefix.min(prefix);
        // }
        // ans

        // with DP
        // no need for verctor, only need to record the last sum
        // let mut dp = vec![0; l];
        // dp[0] = nums[0];
        // let mut dp = nums[0];
        // let mut ans = nums[0];
        // let l = nums.len();
        // for i in 1..l {
        //     dp = nums[i].max(dp + nums[i]);
        //     ans = ans.max(dp);
        // }

        // ans

        // with Divide and Conquer
        Sum::get_sum(&nums, 0, nums.len() - 1).max_sum
    }
}

struct Sum {
    left_sum: i32,
    right_sum: i32,
    max_sum: i32,
    sum: i32,
}

impl Sum {
    fn new(s: i32) -> Sum {
        Sum {
            left_sum: s,
            right_sum: s,
            max_sum: s,
            sum: s,
        }
    }
    fn push_up(l: &Sum, r: &Sum) -> Sum {
        let left_sum = l.left_sum.max(l.sum + r.left_sum);
        let right_sum = r.right_sum.max(r.sum + l.right_sum);
        let max_sum = l.max_sum.max(r.max_sum).max(l.right_sum + r.left_sum);
        let sum = l.sum + r.sum;
        Sum {
            left_sum,
            right_sum,
            max_sum,
            sum,
        }
    }
    fn get_sum(nums: &[i32], start: usize, end: usize) -> Sum {
        if start == end {
            return Sum::new(nums[start]);
        }
        let mid = (start + end) >> 1;
        let l = Sum::get_sum(nums, start, mid);
        let r = Sum::get_sum(nums, mid + 1, end);
        Sum::push_up(&l, &r)
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
