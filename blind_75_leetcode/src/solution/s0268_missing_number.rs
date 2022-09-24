#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut sum = ((l + 0) * (l + 1) / 2) as i32;
        for n in nums {
            sum -= n;
        }
        sum
    }
}

#[cfg(test)]
mod tests_268 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
        assert_eq!(Solution::missing_number(vec![4, 2, 3, 5, 7, 0, 1]), 6);
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
