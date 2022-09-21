#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) >> 1;
            if target == nums[mid] {
                return mid as i32;
            };
            println!("{} {} {} {}", left, mid, right, target);

            if nums[right] > nums[mid] {
                // nums[mid..=right] is sorted
                // nums[left..=mid] is unsorted
                println!("R {} {} {} {}", left, mid, right, target);
                if target > nums[right] {
                    // target maybe in nums[left..=mid]
                    right = mid;
                } else if target > nums[mid] {
                    // target maybe in nums[mid..=right]
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                // nums[left..=mid] is sorted
                // nums[mid..=right] is unsorted
                println!("L {} {} {} {}", left, mid, right, target);
                assert!(nums[right] <= nums[mid]);
                if target <= nums[right] {
                    // target maybe in nums[mid..=right]
                    left = mid + 1;
                    assert!(nums[right] < nums[mid])
                } else if target > nums[mid] {
                    // target maybe in nums[mid..=right]
                    left = mid + 1;
                } else {
                    right = mid;
                }
                println!("AL {} {} {} {}", left, mid, right, target);
            }
        }
        println!("S {} {} {}", left, right, target);
        if nums[left] == target {
            left as i32
        } else {
            -1
        }
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
    }
}
