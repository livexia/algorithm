#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        if nums.is_empty() {
            return 0;
        }
        let mut init_heap = BinaryHeap::new();
        init_heap.push(Reverse(nums[0]));

        let mut dp = vec![];
        dp.push(init_heap);

        for i in 1..nums.len() {
            for count in (0..dp.len()).rev() {
                if let Some(min) = dp[count].peek() {
                    if min <= &Reverse(nums[i]) {
                        if count == 0 {
                            dp[count].push(Reverse(nums[i]))
                        }
                    } else {
                        if count == dp.len() - 1 {
                            let mut new_heap = BinaryHeap::new();
                            new_heap.push(Reverse(nums[i]));
                            dp.push(new_heap);
                        } else {
                            dp[count + 1].push(Reverse(nums[i]))
                        }
                        break;
                    }
                }
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod tests_300 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
