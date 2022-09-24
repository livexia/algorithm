#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut missing = vec![true; l + 1];
        for i in nums {
            missing[i as usize] = false;
        }
        missing.into_iter().enumerate().find(|(_, b)| *b).unwrap().0 as i32
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
