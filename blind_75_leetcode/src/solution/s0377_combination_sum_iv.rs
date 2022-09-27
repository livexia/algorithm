#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        for &num in &nums {
            if target < num {
                continue;
            }
            dp[num as usize] += 1;
        }
        for i in 0..=target {
            for num in &nums {
                if &i < num {
                    continue;
                }
                dp[i as usize] += dp[(i - num) as usize];
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests_377 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
        assert_eq!(
            Solution::combination_sum4(
                vec![
                    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                    24, 25
                ],
                10
            ),
            9
        )
    }
}
