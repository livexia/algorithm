#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // with bottom up
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

        // with official solution
        // let l = s.len();
        // let mut dp = vec![false; l + 1];
        // dp[0] = true;
        // for i in 0..=l {
        //     for j in 0..i {
        //         let word = &s[j..i];
        //         if dp[j] && word_dict.contains(&word.to_string()) {
        //             dp[i] = true;
        //             break;
        //         }
        //     }
        // }
        // dp[l]

        // with top down
        // let l = s.len();
        // let mut memo = vec![false; l + 1];
        // let mut visited = vec![false; l + 1];
        // Solution::dp(&s, &word_dict, &mut memo, &mut visited, l)
    }

    fn dp(
        s: &str,
        word_dict: &[String],
        memo: &mut [bool],
        visited: &mut [bool],
        index: usize,
    ) -> bool {
        if index == 0 {
            return true;
        }
        if visited[index] {
            return memo[index];
        }
        visited[index] = true;
        // for w in word_dict {
        //     if index < w.len() {
        //         continue;
        //     }
        //     let word = &s[(index - w.len())..index];
        //     if word == w && Solution::dp(s, word_dict, memo, visited, index - w.len()) {
        //         memo[index] = true;
        //         return true;
        //     }
        // }
        for j in 0..index {
            let word = &s[j..index];
            if word_dict.contains(&word.to_string()) && Solution::dp(s, word_dict, memo, visited, j)
            {
                memo[index] = true;
                return true;
            }
        }
        memo[index]
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
