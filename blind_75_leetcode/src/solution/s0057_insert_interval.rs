#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut ans = vec![];
        let left = Solution::binary_search_by(&intervals, new_interval[0], 0).saturating_sub(1);
        let right = Solution::binary_search_by(&intervals, new_interval[1], 1);
        ans.extend_from_slice(&intervals[..left]);
        ans.extend(Solution::merge_interval(
            new_interval,
            intervals[left].clone(),
        ));
        if left != right {
            let last = ans.pop().unwrap();
            ans.extend(Solution::merge_interval(last, intervals[right].clone()));
        }

        ans.extend_from_slice(&intervals[right + 1..]);
        ans
    }

    fn binary_search_by(intervals: &[Vec<i32>], num: i32, by: usize) -> usize {
        let mut left = 0;
        let mut right = intervals.len() - 1;
        while left < right {
            let mid = (left + right) >> 1;
            if num < intervals[mid][by] {
                right = mid
            } else {
                left = mid + 1
            }
        }
        left
    }

    fn merge_interval(a: Vec<i32>, b: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let (a, b) = if a[0] < b[0] { (a, b) } else { (b, a) };
        if b[1] <= a[1] {
            res.push(vec![a[0], a[1]])
        } else if b[0] > a[1] {
            res.push(a);
            res.push(b);
        } else {
            res.push(vec![a[0], b[1]])
        }
        res
    }
}

#[cfg(test)]
mod tests_57 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn test_merge_interval() {
        assert_eq!(
            Solution::merge_interval(vec![1, 3], vec![0, 5]),
            leetcode_vec![[0, 5]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 3], vec![2, 5]),
            leetcode_vec![[1, 5]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 3], vec![4, 5]),
            leetcode_vec![[1, 3], [4, 5]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 3], vec![-2, 0]),
            leetcode_vec![[-2, 0], [1, 3]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 3], vec![-2, 2]),
            leetcode_vec![[-2, 3]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 4], vec![2, 3]),
            leetcode_vec![[1, 4]]
        );
        assert_eq!(
            Solution::merge_interval(vec![1, 5], vec![1, 5]),
            leetcode_vec![[1, 5]]
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::insert(leetcode_vec![[1, 5]], vec![0, 3]),
            leetcode_vec![[0, 5]]
        );
        assert_eq!(
            Solution::insert(leetcode_vec![[1, 5]], vec![6, 8]),
            leetcode_vec![[1, 5], [6, 8]]
        );
        assert_eq!(
            Solution::insert(leetcode_vec![[6, 9]], vec![2, 5]),
            leetcode_vec![[2, 5], [6, 9]]
        );
        assert_eq!(
            Solution::insert(leetcode_vec![[1, 3], [6, 9]], vec![2, 5]),
            leetcode_vec![[1, 5], [6, 9]]
        );
        assert_eq!(
            Solution::insert(
                leetcode_vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]],
                vec![4, 8]
            ),
            leetcode_vec![[1, 2], [3, 10], [12, 16]]
        );
        assert_eq!(Solution::insert(vec![], vec![5, 7]), leetcode_vec![[5, 7]]);
        assert_eq!(
            Solution::insert(leetcode_vec![[1, 5]], vec![2, 3]),
            leetcode_vec![[1, 5]]
        );
        assert_eq!(
            Solution::insert(leetcode_vec![[1, 5]], vec![2, 7]),
            leetcode_vec![[1, 7]]
        );
    }
}
