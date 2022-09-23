#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut right = 1;
        while left < right {
            ans = ans.max((height[right].min(height[left])) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
        // with loop
        // let mut ans = 0;
        // let l = height.len();
        // let mut most_right = l;
        // for left in 0..l {
        //     for right in (left + 1..most_right).rev() {
        //         ans = ans.max((height[right].min(height[left])) * (right - left) as i32);
        //         if height[left] < height[right] {
        //             most_right = right + 1;
        //             break;
        //         }
        //     }
        // }
        // ans
    }
}

#[cfg(test)]
mod tests_11 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 10, 6, 2, 5, 4, 10, 3, 7]), 50);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![1, 3, 2, 5, 25, 24, 5]), 24);
    }
}
