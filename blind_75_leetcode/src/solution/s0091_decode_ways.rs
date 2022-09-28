#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let l = s.len();
        let mut dp = vec![0; l + 1];
        dp[0] = 1;
        for i in 0..=l {
            if i > 0 {
                if let Ok(n) = s[(i - 1)..i].parse::<i32>() {
                    if n > 0 && n <= 9 {
                        dp[i] += dp[i - 1]
                    }
                }
            }
            if i > 1 {
                if let Ok(n) = s[(i - 2)..i].parse::<i32>() {
                    if n > 9 && n <= 26 {
                        dp[i] += dp[i - 2]
                    }
                }
            }
        }
        dp[l]
    }
}

#[cfg(test)]
mod tests_91 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }
}
