#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;
        for &num in &nums_set {
            if !nums_set.contains(&(num - 1)) {
                let mut start = num;
                let mut count = 1;
                while nums_set.contains(&(start + 1)) {
                    start += 1;
                    count += 1;
                }
                ans = ans.max(count);
                if ans * 2 > nums_set.len() as i32 {
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_128 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
        assert_eq!(
            Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]),
            7
        );
    }
}
