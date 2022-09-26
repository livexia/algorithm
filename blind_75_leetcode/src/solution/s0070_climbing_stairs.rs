#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut path = vec![];
        let mut ans = 0;
        if n == 0 || n == 1 {
            return 1;
        } else {
            path.push(vec![0]);
            while let Some(last_path) = path.pop() {
                if let Some(&last_step) = last_path.last() {
                    if last_step == n {
                        ans += 1;
                    } else if last_step < n {
                        path.push([last_path.clone(), vec![last_step + 1]].concat());
                        path.push([last_path.clone(), vec![last_step + 2]].concat());
                    }
                }
            }
        }
        ans
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
    }
}
