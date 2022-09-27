#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 0..=target {
            for num in &nums {
                if &i < num {
                    continue;
                }
                dp[i as usize] += dp[(i - num) as usize];
            }
        }
        dp[target as usize]

        // with top down
        // let mut memo = vec![-1; target as usize + 1];
        // memo[0] = 1;
        // Solution::dp(&nums, target as usize, &mut memo)
    }

    fn dp(nums: &[i32], target: usize, memo: &mut [i32]) -> i32 {
        if memo[target] >= 0 {
            memo[target]
        } else {
            memo[target] = nums
                .iter()
                .filter(|&&num| num <= target as i32)
                .fold(0, |s, &num| {
                    s + Solution::dp(nums, target - num as usize, memo)
                });
            memo[target]
        }
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
