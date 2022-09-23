#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mask = 0x1;
        let mut ans = 0;
        let mut carry = 0;
        for i in 0..32 {
            let mut bit = 0;
            let mut a_bit = (a & (mask << i)) >> i;
            let mut b_bit = (b & (mask << i)) >> i;
            if a_bit == -1 {
                a_bit = 1;
            }
            if b_bit == -1 {
                b_bit = 1;
            }
            if (a_bit == 1 && b_bit == 0) || (a_bit == 0 && b_bit == 1) {
                if carry == 0 {
                    bit = 1;
                }
            } else if a_bit == 1 && b_bit == 1 {
                if carry == 1 {
                    bit = 1;
                } else {
                    carry = 1;
                }
            } else {
                if carry == 1 {
                    bit = 1;
                    carry = 0;
                }
            }
            ans = ans | (bit << i);
        }
        ans
    }
}

#[cfg(test)]
mod tests_371 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(2, 3), 5);
        assert_eq!(Solution::get_sum(-1, 1), 0);
        assert_eq!(Solution::get_sum(-2, 3), 1);
        assert_eq!(Solution::get_sum(-2, -3), -5);
    }
}
