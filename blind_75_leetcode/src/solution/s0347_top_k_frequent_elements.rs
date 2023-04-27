#![allow(dead_code)]
pub struct Solution {}

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1
        }
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for (k, v) in map {
            heap.push((v, k));
        }
        (0..k).filter_map(|_| heap.pop()).map(|(_, k)| k).collect()
    }
}

#[cfg(test)]
mod tests_347 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
        assert_eq!(
            Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2),
            vec![2, -1]
        );
    }
}
