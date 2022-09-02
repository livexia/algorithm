#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut answer = vec![1; l];
        for i in 1..l {
            answer[i] = nums[i - 1] * answer[i - 1]; // calculate prefix product
        }
        let mut suffix_product = 1;
        for i in (0..l).rev() {
            answer[i] *= suffix_product; // calcuate the result
            suffix_product *= nums[i]; // calculate suffix product
        }

        answer
    }
}

#[cfg(test)]
mod tests_238 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
