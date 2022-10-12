#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        // Solution::sort_left(points)
        Solution::sort_right(points)
    }

    pub fn sort_right(mut points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<(i32, i32)> = points.into_iter().map(|p| (p[0], p[1])).collect();
        points.sort_by(|a, b| a.1.cmp(&b.1));
        let mut count = 1;
        let mut last = points[0].1;
        for &(start, end) in &points[1..] {
            if last < start {
                count += 1;
                last = end;
            }
        }
        count
    }

    pub fn sort_left(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<(i32, i32)> = points.into_iter().map(|p| (p[0], p[1])).collect();
        points.sort();
        let mut count = points.len() as i32;
        let mut last = points[0].1;
        for &(start, end) in &points[1..] {
            if start > last {
                last = end;
            } else {
                count -= 1;
                last = last.min(end)
            }
        }
        count
    }
}

#[cfg(test)]
mod tests_452 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_min_arrow_shots(leetcode_vec![[10, 16], [2, 8], [1, 6], [7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(leetcode_vec![[1, 2], [3, 4], [5, 6], [7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(leetcode_vec![[1, 2], [2, 3], [3, 4], [4, 5]]),
            2
        );
    }
}
