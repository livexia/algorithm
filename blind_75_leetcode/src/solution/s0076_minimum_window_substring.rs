#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let mut hash = HashMap::new();
        for c in t.chars() {
            *hash.entry(c).or_insert(0) += 1;
        }
        let mut t_count = t.len();
        let s: Vec<_> = s.chars().collect();
        let (mut left, mut result) = (0, (0, s.len()));
        for (right, &c) in s.iter().enumerate() {
            let c_count = hash.entry(c).or_insert(0);
            if *c_count > 0 {
                t_count -= 1;
            }
            *c_count -= 1;

            if t_count == 0 {
                while let Some(&count) = hash.get(&s[left]) {
                    if count == 0 {
                        break;
                    }
                    *hash.entry(s[left]).or_insert(0) += 1;
                    left += 1;
                }
                if right - left < result.1 - result.0 {
                    result = (left, right)
                }
                *hash.entry(s[left]).or_insert(0) += 1;
                t_count += 1;
                left += 1;
            }
        }

        if result.1 == s.len() {
            "".to_string()
        } else {
            s[result.0..=result.1].iter().collect()
        }
    }
}

#[cfg(test)]
mod tests_76 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "b".to_string()),
            "".to_string()
        );
    }
}
