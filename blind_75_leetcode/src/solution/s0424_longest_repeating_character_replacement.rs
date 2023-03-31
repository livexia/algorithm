#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests_424 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
