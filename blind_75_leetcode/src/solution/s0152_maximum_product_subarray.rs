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
        // with prefix product
        // let mut prefix_product = vec![1; nums.len()];
        // prefix_product[0] = nums[0];
        // for i in 1..nums.len() {
        //     if prefix_product[i - 1] == 0 {
        //         prefix_product[i] = nums[i];
        //         continue;
        //     }
        //     prefix_product[i] = prefix_product[i - 1] * nums[i];
        // }
        // let mut min_minus_prefix_product = 1;
        // let mut ans = nums[0];
        // for p in prefix_product {
        //     if p < 0 {
        //         ans = ans.max(p / min_minus_prefix_product);
        //         if min_minus_prefix_product == 1 {
        //             min_minus_prefix_product = p;
        //         }
        //         min_minus_prefix_product = min_minus_prefix_product.max(p);
        //     } else {
        //         if p == 0 {
        //             min_minus_prefix_product = 1
        //         }
        //         ans = ans.max(p);
        //     }
        // }
        // ans
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
        assert_eq!(Solution::max_product(vec![2, 3, -1, -2, 4, -5]), 40);
    }
}
