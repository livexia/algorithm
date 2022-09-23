#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let l = height.len();
        let mut ans = 0;
        for left in 0..l {
            for right in left + 1..l {
                ans = ans.max(volume(&height, left, right))
            }
        }
        ans
    }
}

fn volume(height: &[i32], left: usize, right: usize) -> i32 {
    let width = (right - left) as i32;
    let h = height[left].min(height[right]);
    width * h
}

#[cfg(test)]
mod tests_11 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
