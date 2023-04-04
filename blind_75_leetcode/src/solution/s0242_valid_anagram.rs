#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counter = [0; 26];
        for (b1, b2) in s
            .bytes()
            .zip(t.bytes())
            .map(|(b1, b2)| ((b1 - b'a') as usize, (b2 - b'a') as usize))
        {
            counter[b1] += 1;
            counter[b2] -= 1;
        }
        counter.into_iter().all(|i| i == 0)
    }
}

#[cfg(test)]
mod tests_242 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
