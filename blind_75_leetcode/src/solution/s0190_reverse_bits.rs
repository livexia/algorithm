#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        for i in 0..16 {
            let l = (x & (1 << i)) << (31 - i - i);
            let r = (x & (1 << (31 - i))) >> (31 - i - i);
            x = x & !(1 << i) & !(1 << (31 - i));
            x |= r | l;
        }
        x
    }
}

#[cfg(test)]
mod tests_190 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_bits(0b1), 0x80000000);
        // assert_eq!(
        //     Solution::reverse_bits(0b00000010100101000001111010011100),
        //     964176192
        // );
        // assert_eq!(
        //     Solution::reverse_bits(0b11111111111111111111111111111101),
        //     3221225471
        // );
    }
}