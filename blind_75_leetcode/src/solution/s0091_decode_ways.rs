#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // let alphabet: Vec<usize> = (0..1)
        //     .chain((0..9).map(|_| 1))
        //     .chain((0..17).map(|_| 2))
        //     .collect();
        let alphabet = vec![
            0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        let l = s.len();
        let mut dp = vec![0; l + 1];
        dp[0] = 1;
        for i in 0..=l {
            for (j, &c) in alphabet.iter().enumerate() {
                if i < c {
                    continue;
                }
                if s[(i - c)..i] == j.to_string() {
                    dp[i] += dp[i - c]
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
    }
}
