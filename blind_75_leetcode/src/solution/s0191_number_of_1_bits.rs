#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n != 0 {
            count += n & 0b1;
            n >>= 1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests_191 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000000001011),
            3
        );
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000010000000),
            1
        );
        assert_eq!(
            Solution::hammingWeight(0b11111111111111111111111111111101),
            31
        );
        // assert_eq!(Solution::hammingWeight(-3), 31);
    }
}
