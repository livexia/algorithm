#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut ans = 0;
        for i in 0..32 {
            let bit = if i < 16 {
                (x & (1 << i)) << (31 - i - i)
            } else {
                (x & (1 << i)) >> (i + i - 31)
            };
            ans |= bit;
        }
        ans
    }
}

#[cfg(test)]
mod tests_190 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_bits(0b1), 0x80000000);
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            3221225471
        );
    }
}
