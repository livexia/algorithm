#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<_> = s
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if s.is_empty() {
            return true;
        }
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests_125 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome("".to_string()), true);
        assert_eq!(Solution::is_palindrome("a.".to_string()), true);
    }
}
