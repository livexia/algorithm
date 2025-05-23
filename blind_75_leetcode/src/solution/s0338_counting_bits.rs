#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits = Vec::with_capacity(n as usize + 1);
        bits.push(0);
        let mut offset = 0;
        for i in 1..=n as usize {
            if i & (i - 1) == 0 {
                offset = i;
            }
            bits.push(bits[i - offset] + 1);
        }
        bits
    }
}

#[cfg(test)]
mod tests_338 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
