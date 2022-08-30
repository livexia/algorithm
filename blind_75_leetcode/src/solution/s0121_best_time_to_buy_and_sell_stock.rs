#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = 0;
        let days = prices.len();
        while right < days {
            result = result.max(prices[right] - prices[left]);
            if prices[left] > prices[right] {
                left = right;
            }
            right += 1;
        }
        result
        // let mut result = 0;
        // let mut min_price = prices[0];
        // for price in prices {
        //     result = result.max(price - min_price);
        //     min_price = min_price.min(price);
        // }
        // result
    }
}

#[cfg(test)]
mod tests_121 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
        assert_eq!(Solution::max_profit(vec![1, 5, 0, 3]), 4);
    }
}
