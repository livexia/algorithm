#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        use std::collections::HashSet;
        let mut visited = HashSet::new();
        let mut possible = vec![amount];
        let mut count = 0;
        while !possible.is_empty() {
            let mut new_possible = vec![];
            while let Some(remain) = possible.pop() {
                if !visited.insert(remain) || remain < 0 {
                    continue;
                }
                if remain == 0 {
                    return count;
                }
                for c in &coins {
                    new_possible.push(remain - c);
                }
            }
            possible = new_possible;
            count += 1;
        }
        -1
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
