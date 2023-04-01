#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut result = 0;

        let mut freq = [0; 26];
        let mut max_count = 0;
        let s: Vec<_> = s.bytes().map(|b| (b - b'A') as usize).collect();
        for (right, &b) in s.iter().enumerate() {
            freq[b] += 1;
            max_count = max_count.max(freq[b]);
            if right - left + 1 > max_count as usize + k as usize {
                freq[s[left]] -= 1;
                left += 1;
            }
            result = result.max(right - left);
        }
        result.max(s.len() - left) as i32
    }
}

#[cfg(test)]
mod tests_424 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
