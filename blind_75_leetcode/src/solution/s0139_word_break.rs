#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let l = s.len();
        let mut dp = vec![false; l + 1];
        dp[0] = true;
        for i in 0..=l {
            for w in &word_dict {
                if i < w.len() {
                    continue;
                }
                let word = &s[(i - w.len())..i];
                dp[i] |= dp[i - w.len()] && word == w;
            }
        }
        dp[l]
    }
}

#[cfg(test)]
mod tests_139 {
    use super::*;

    fn helper(s: &str, word_dict: &[&str]) -> bool {
        Solution::word_break(
            s.to_string(),
            word_dict.into_iter().map(|s| s.to_string()).collect(),
        )
    }
    #[test]
    fn it_works() {
        assert_eq!(helper("leetcode", &["leet", "code"]), true);
        assert_eq!(helper("applepenapple", &["apple", "pen"]), true);
        assert_eq!(
            helper("catsandog", &["cats", "dog", "sand", "and", "cat"]),
            false
        );
    }
}
