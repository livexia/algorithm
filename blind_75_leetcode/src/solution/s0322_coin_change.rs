#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut coins = coins;
        coins.sort();
        let mut path = vec![(amount, 0)];
        let mut ans = amount / coins[0] + 1;

        while let Some((remain, count)) = path.pop() {
            if remain == 0 {
                ans = ans.min(count);
                break;
            }
            if ans <= count + 1 || remain < coins[0] {
                continue;
            }
            for c in &coins {
                if c > &remain {
                    break;
                }
                path.push((remain - c, count + 1));
            }
        }
        if ans == amount / coins[0] + 1 {
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
        assert_eq!(Solution::coin_change(vec![4, 3, 2], 9), 3);
        assert_eq!(Solution::coin_change(vec![4, 3, 2, 8], 9), 3);
        assert_eq!(Solution::coin_change(vec![4, 3, 2, 8], 10), 2);
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
