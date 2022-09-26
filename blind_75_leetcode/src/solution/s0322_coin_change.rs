#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut coins = coins;
        let mut amount = amount;
        coins.sort();
        let mut ans = 0;
        while let Some(coin) = coins.pop() {
            if amount >= coin {
                ans += amount / coin;
                amount %= coin;
            }
            if amount == 0 {
                break;
            }
        }
        if ans == 0 || amount != 0 {
            -1
        } else {
            ans
        }
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
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
