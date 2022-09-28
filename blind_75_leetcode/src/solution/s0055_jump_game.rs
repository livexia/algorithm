#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        if l == 1 {
            return true;
        }
        let mut max = nums[0];
        for i in 1..l {
            if i as i32 <= max {
                max = max.max(nums[i] + i as i32);
                if max as usize >= l - 1 {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests_55 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![2, 0]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
    }
}
