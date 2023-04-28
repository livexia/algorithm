#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    // This is a template
}

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct MedianFinder {
    left_heap: BinaryHeap<i32>,           // with max heap
    right_heap: BinaryHeap<Reverse<i32>>, // with min heap
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.left_heap.is_empty() || self.left_heap.peek().unwrap() >= &num {
            self.left_heap.push(num);
            if self.left_heap.len() > self.right_heap.len() + 1 {
                let val = self.left_heap.pop().unwrap();
                self.right_heap.push(Reverse(val));
            }
        } else {
            self.right_heap.push(Reverse(num));
            if self.right_heap.len() > self.left_heap.len() {
                let Reverse(val) = self.right_heap.pop().unwrap();
                self.left_heap.push(val);
            }
        }
    }

    fn find_median(&mut self) -> f64 {
        if self.left_heap.len() > self.right_heap.len() {
            *self.left_heap.peek().unwrap() as f64
        } else {
            (self.left_heap.peek().unwrap() + self.right_heap.peek().unwrap().0) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests_295 {
    use super::*;

    #[test]
    fn it_works() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5f64);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0f64);
    }
}
