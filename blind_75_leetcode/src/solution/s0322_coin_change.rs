#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let amount = amount as usize;
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for i in 0..=amount {
            for &c in &coins {
                if c as usize <= i {
                    dp[i] = dp[i].min(dp[i - c as usize] + 1)
                }
            }
        }
        if dp[amount] == amount + 1 {
            -1
        } else {
            dp[amount] as i32
        }
        // Top Down DP with memorization
        // if amount == 0 {
        //     return 0;
        // }
        // let mut visited = vec![0; amount as usize + 1];
        // Solution::dp(&coins, amount, &mut visited)
    }

    fn dp(coins: &[i32], remain: i32, visited: &mut [i32]) -> i32 {
        if remain < 0 {
            return -1;
        }
        if remain == 0 {
            return 0;
        }
        if visited[remain as usize] != 0 {
            return visited[remain as usize];
        }
        let mut min = -1;
        for c in coins {
            let res = Solution::dp(coins, remain - c, visited);
            if (res >= 0 && res + 1 < min) || (res >= 0 && min == -1) {
                min = res + 1;
            }
        }
        visited[remain as usize] = min;
        visited[remain as usize]
    }
}

#[cfg(test)]
mod tests_322 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![4, 3, 2], 9), 3);
        assert_eq!(Solution::coin_change(vec![4, 3, 2, 8], 9), 3);
        assert_eq!(Solution::coin_change(vec![4, 3, 2, 8], 10), 2);
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
        assert_eq!(Solution::coin_change(vec![3, 7, 405, 436], 8839), 25);
    }
}
