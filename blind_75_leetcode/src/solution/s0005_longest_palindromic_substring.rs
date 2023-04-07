#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let n = s.len();
        let mut result = (0, 0);
        for i in 0..n {
            if n - 1 - i < result.1 - result.0 {
                break;
            }
            for j in (result.1 - result.0 + i..n).rev() {
                if Solution::dp(i, j, &s) {
                    result = (i, j);
                    break;
                }
            }
        }
        s[result.0..=result.1].iter().collect()
    }

    fn dp(i: usize, j: usize, s: &[char]) -> bool {
        i == j || s[i] == s[j] && (i + 1 == j || Solution::dp(i + 1, j - 1, s))
    }
}

#[cfg(test)]
mod tests_5 {
    use super::*;

    #[test]
    fn it_works() {
        assert!(["bab".to_string(), "aba".to_string()]
            .contains(&Solution::longest_palindrome("babad".to_string())));
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }
}
