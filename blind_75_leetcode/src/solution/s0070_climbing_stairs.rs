#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut before_last_count = 1;
        let mut last_count = 1;
        let n = n as usize;
        for _ in 2..=n {
            let temp = last_count;
            last_count = last_count + before_last_count;
            before_last_count = temp;
        }
        last_count
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
