#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        for mut i in 0..=n {
            let mut count = 0;
            while i != 0 {
                i &= i - 1;
                count += 1;
            }
            ans.push(count);
        }
        ans
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
