#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut dp = vec![false; l];
        dp[0] = true;
        for i in 1..l {
            if dp[i] {
                continue;
            }
            for j in (0..i).rev() {
                if dp[j] && j + nums[j] as usize >= i {
                    let upperbond = nums.len().min(j + nums[j] as usize + 1);
                    for k in i..upperbond {
                        dp[k] = true;
                    }
                    break;
                }
            }
        }
        dp[l - 1]
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
    }
}
