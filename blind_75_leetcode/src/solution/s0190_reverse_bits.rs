#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mask8 = 0x00ff00ff; // 0b00000000111111110000000011111111
        let mask4 = 0x0f0f0f0f; // 0b00001111000011110000111100001111
        let mask2 = 0x33333333; // 0b00110011001100110011001100110011
        let mask1 = 0x55555555; // 0b01010101010101010101010101010101
        x = x >> 16 | x << 16;
        x = (x & mask8) << 8 | (x >> 8) & mask8;
        x = (x & mask4) << 4 | (x >> 4) & mask4;
        x = (x & mask2) << 2 | (x >> 2) & mask2;
        x = (x & mask1) << 1 | (x >> 1) & mask1;

        x
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
