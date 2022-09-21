#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) >> 1;
            if nums[right] > nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        match nums[..left].binary_search(&target) {
            Ok(n) => return n as i32,
            Err(_) => (),
        };
        match nums[left..].binary_search(&target) {
            Ok(n) => return (n + left) as i32,
            Err(_) => (),
        };
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
    }
}
