#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) >> 1;
            if target == nums[mid] {
                return mid as i32;
            };
            if nums[mid] >= nums[left] {
                // mid 在左区间
                if target >= nums[left] && target < nums[mid] {
                    // target 处于 nums[left..mid] 中的时候
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // mid 在右区间
                if target > nums[mid] && target <= nums[right] {
                    // target 处于 nusm[mid..right] 中的时候
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests_33 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
        assert_eq!(Solution::search(vec![1, 3], 3), 1);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }
}
