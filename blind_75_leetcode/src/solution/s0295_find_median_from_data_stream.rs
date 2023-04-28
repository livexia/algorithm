#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    // This is a template
}

use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        self.heap.push(num);
    }

    fn find_median(&mut self) -> f64 {
        let l = self.heap.len();
        let mut temp = Vec::with_capacity(l / 2 + 1);
        for _ in 0..l / 2 {
            temp.push(self.heap.pop().unwrap());
        }
        let result = if l % 2 == 0 {
            (self.heap.peek().unwrap() + temp.last().unwrap()) as f64 / 2.0f64
        } else {
            *self.heap.peek().unwrap() as f64
        };
        for num in temp.into_iter().rev() {
            self.heap.push(num);
        }

        result
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
