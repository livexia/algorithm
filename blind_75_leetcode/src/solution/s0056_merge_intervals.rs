#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<(i32, i32)> = intervals.into_iter().map(|v| (v[0], v[1])).collect();
        intervals.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut last = 0;
        for (li, ri) in intervals {
            if ans.is_empty() || ans[last - 1][1] < li {
                last += 1;
                ans.push(vec![li, ri])
            } else {
                ans[last - 1][1] = ri.max(ans[last - 1][1])
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_56 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::merge(leetcode_vec![[2, 3], [4, 5], [1, 6]]),
            leetcode_vec![[1, 6]]
        );
        assert_eq!(
            Solution::merge(leetcode_vec![[1, 3], [2, 6], [8, 10], [15, 18]]),
            leetcode_vec![[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(
            Solution::merge(leetcode_vec![[1, 4], [4, 5]]),
            leetcode_vec![[1, 5]]
        );
        assert_eq!(
            Solution::merge(leetcode_vec![[1, 10], [2, 3], [4, 5], [6, 7], [8, 9]]),
            leetcode_vec![[1, 10]]
        );
        assert_eq!(
            Solution::merge(leetcode_vec![[2, 3], [4, 5], [6, 7], [8, 9], [1, 10]]),
            leetcode_vec![[1, 10]]
        );
    }
}
