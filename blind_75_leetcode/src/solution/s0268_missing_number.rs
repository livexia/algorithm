#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut xor = 0;
        for (i, j) in nums.into_iter().enumerate() {
            xor ^= j;
            xor ^= i as i32;
        }
        xor ^= n;
        xor
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
