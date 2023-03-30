#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut result) = (0, 0);
        let s: Vec<_> = s.chars().collect();
        let mut set = std::collections::HashSet::new();
        for (right, &c) in s.iter().enumerate() {
            if result + left >= s.len() {
                return result as i32;
            }
            if set.contains(&c) {
                result = result.max(right - left);
                while s[left] != c {
                    set.remove(&s[left]);
                    left += 1;
                }
                left += 1;
            } else {
                set.insert(c);
            }
        }
        result.max(s.len() - left) as i32
    }
}

#[cfg(test)]
mod tests_3 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("abcb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    }
}
