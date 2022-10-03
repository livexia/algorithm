#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums.dedup();
        if nums.len() == 0 {
            return 0;
        }
        let mut start = 0;
        let mut pointer = 1;
        let mut ans = 0;
        while pointer < nums.len() {
            if nums[pointer - 1] + 1 != nums[pointer] {
                ans = ans.max(pointer - start);
                start = pointer;
            }
            pointer += 1;
        }
        ans = ans.max(pointer - start);
        ans as i32
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
