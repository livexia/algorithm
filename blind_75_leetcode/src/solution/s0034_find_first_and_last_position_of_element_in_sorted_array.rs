#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                (left, right) = (mid, mid);
                while left >= 1 && nums[left - 1] == target {
                    left -= 1;
                }
                while right + 1 < nums.len() && nums[right + 1] == target {
                    right += 1;
                }
                return vec![left as i32, right as i32];
            } else if nums[mid] < target {
                left = mid + 1;
            } else if mid > 0 {
                right = mid - 1;
            } else {
                break;
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests_34 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
