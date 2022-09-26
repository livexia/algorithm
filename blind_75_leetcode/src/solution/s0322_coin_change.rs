#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut sums = vec![(0, 0)];
        for c in coins {
            let last_sums = sums.clone();
            sums = vec![];
            for (a, count) in &last_sums {
                for i in 0..=(amount - a) / c {
                    sums.push((a + i * c, count + i));
                }
            }
        }
        sums.into_iter()
            .filter(|(a, _)| a == &amount)
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or((0, -1))
            .1
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
