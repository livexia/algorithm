#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let pivot = (left + right) >> 1;
            match nums[pivot].cmp(&nums[right]) {
                std::cmp::Ordering::Less => right = pivot,
                std::cmp::Ordering::Greater => left = pivot + 1,
                std::cmp::Ordering::Equal => right -= 1,
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests_154 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
