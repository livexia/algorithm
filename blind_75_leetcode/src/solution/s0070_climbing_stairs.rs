#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2)
    }
}

#[cfg(test)]
mod tests_70 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(44), 1134903170);
    }
}
