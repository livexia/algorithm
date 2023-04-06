#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_uppercase());
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a != b {
                return false;
            }
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
