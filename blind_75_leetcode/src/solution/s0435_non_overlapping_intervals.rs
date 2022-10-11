#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals: Vec<(i32, i32)> = intervals.into_iter().map(|v| (v[0], v[1])).collect();
        intervals.sort();
        let mut count = 0;
        let mut last = intervals[0].1;
        for &(li, ri) in &intervals[1..] {
            if last <= li {
                last = ri
            } else {
                count += 1;
                last = last.min(ri)
            }
        }
        count
    }
}

#[cfg(test)]
mod tests_435 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::erase_overlap_intervals(leetcode_vec![[1, 2], [2, 3], [3, 4], [1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(leetcode_vec![[1, 2], [1, 2], [1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(leetcode_vec![[1, 2], [2, 3]]),
            0
        );
    }
}
