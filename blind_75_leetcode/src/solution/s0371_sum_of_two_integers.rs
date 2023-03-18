#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // let mask = 0x1;
        // let mut ans = 0;
        // let mut carry = 0;
        // for i in 0..32 {
        //     let a_bit = a & (mask << i);
        //     let b_bit = b & (mask << i);
        //     let bit = (a_bit ^ b_bit) ^ carry;

        //     if a_bit ^ b_bit == 0 {
        //         carry = a_bit & b_bit;
        //     }
        //     carry <<= 1;
        //     // carry = (a_bit & b_bit) << 1;
        //     // println!("a_bit: {:#034b}", a_bit);
        //     // println!("b_bit: {:#034b}", b_bit);
        //     // println!("bit:   {:#034b}", bit);
        //     // println!("carry: {:#034b}", carry);
        //     // println!("ans:   {:#034b}", ans);
        //     // println!();
        //     ans = ans | (bit);
        // }
        // ans
        // https://leetcode.cn/problems/sum-of-two-integers/solution/liang-zheng-shu-zhi-he-by-leetcode-solut-c1s3/
        let (mut a, mut b) = (a, b);
        // println!("a: {:#034b}", a);
        // println!("b: {:#034b}", b);
        while b != 0 {
            let carry = (a & b) << 1;
            a ^= b;
            b = carry;
            // println!("a ^ b        {:#034b}", a);
            // println!("(a & b) << 1 {:#034b}", carry);
        }
        a
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
        assert_eq!(Solution::get_sum(-1, -1), -2);
    }
}
