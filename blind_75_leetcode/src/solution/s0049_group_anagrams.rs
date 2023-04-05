#![allow(dead_code)]
pub struct Solution {}

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return vec![];
        }
        let mut strs_counter = HashMap::new();
        for s in strs {
            strs_counter
                .entry(Solution::str_counter_hash(&s))
                .or_insert(vec![])
                .push(s);
        }
        strs_counter.into_values().collect()
    }

    fn str_counter_hash(s: &str) -> u64 {
        let counter = Solution::str_counter(s);
        let mut hash = DefaultHasher::new();
        counter.hash(&mut hash);
        hash.finish()
    }

    fn str_counter(s: &str) -> [i32; 26] {
        let mut counter = [0; 26];
        s.bytes().for_each(|b| counter[(b - b'a') as usize] += 1);
        counter
    }

    fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counter = [0; 26];
        s.bytes().for_each(|b| counter[(b - b'a') as usize] += 1);
        t.bytes().for_each(|b| counter[(b - b'a') as usize] -= 1);

        counter.iter().all(|&i| i == 0)
    }
}

#[cfg(test)]
mod tests_49 {
    use super::*;

    fn helper(mut strs: Vec<Vec<String>>, mut expected: Vec<Vec<String>>) -> bool {
        if strs.len() != expected.len() {
            return false;
        }
        strs.sort_by_key(|v| v.len());
        expected.sort_by_key(|v| v.len());
        for (v1, v2) in strs.iter().zip(expected.iter()) {
            if v1.len() != v2.len() || v1.iter().any(|s| !v2.contains(s)) {
                return false;
            }
        }
        true
    }

    fn slice_str_to_vec_string(strs: &[&str]) -> Vec<String> {
        strs.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn it_works() {
        assert!(helper(
            Solution::group_anagrams(slice_str_to_vec_string(&[
                "eat", "tea", "tan", "ate", "nat", "bat"
            ])),
            vec![
                slice_str_to_vec_string(&["bat"]),
                slice_str_to_vec_string(&["nat", "tan"]),
                slice_str_to_vec_string(&["ate", "eat", "tea"])
            ]
        ));
        assert!(helper(
            Solution::group_anagrams(slice_str_to_vec_string(&[""])),
            vec![slice_str_to_vec_string(&[""]),]
        ));
        assert!(helper(
            Solution::group_anagrams(slice_str_to_vec_string(&["a"])),
            vec![slice_str_to_vec_string(&["a"]),]
        ));
    }
}
