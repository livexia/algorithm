#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while right > left {
            let mid = (left + right) >> 1;
            if nums[left] < nums[right] {
                return nums[left];
            } else {
                if nums[left] > nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }
        nums[right]
    }
}

#[cfg(test)]
mod tests_153 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }
}
