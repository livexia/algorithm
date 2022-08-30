#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut shown = HashSet::new();
        nums.into_iter().any(|num| !shown.insert(num))

        // sort with windows
        // https://leetcode.cn/problems/contains-duplicate/solution/rust-0ms-shi-shi-zheng-ming-you-shi-hou-lbldt/
        // let mut nums = nums;
        // nums.sort();
        // nums.windows(2).any(|x| x[0] == x[1])

        // vec dedup
        // https://leetcode.cn/problems/contains-duplicate/solution/in-rust-we-trust-vector-contains_duplica-f1fz/
        // let mut nums = nums;
        // let l = nums.len();
        // nums.sort();
        // nums.dedup();
        // l != nums.len()
    }
}

#[cfg(test)]
mod tests_217 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
