#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        if first_list.is_empty() || second_list.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut left = 0;
        let mut right = 0;
        while left < first_list.len() && right < second_list.len() {
            let (s0, e0) = (first_list[left][0], first_list[left][1]);
            let (s1, e1) = (second_list[right][0], second_list[right][1]);
            if e0 >= e1 {
                right += 1;
            } else {
                left += 1;
            }
            if s1 <= e0 && e1 >= s0 {
                res.push(vec![s0.max(s1), e0.min(e1)]);
            }

            // another way to calculate the intersection
            // let s = s0.max(s1);
            // let e = e0.min(e1);
            // if s <= e {
            //     res.push(vec![s, e])
            // }
        }
        res
    }
}

#[cfg(test)]
mod tests_986 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::interval_intersection(
                leetcode_vec![[0, 2], [5, 10], [13, 23], [24, 25]],
                leetcode_vec![[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            leetcode_vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
        assert_eq!(
            Solution::interval_intersection(leetcode_vec![[1, 3], [5, 9]], vec![]).is_empty(),
            true
        );
        assert_eq!(
            Solution::interval_intersection(vec![], leetcode_vec![[4, 8], [10, 12]]).is_empty(),
            true
        );
        assert_eq!(
            Solution::interval_intersection(leetcode_vec![[1, 7]], leetcode_vec![[3, 10]]),
            leetcode_vec![[3, 7]]
        );
    }
}
