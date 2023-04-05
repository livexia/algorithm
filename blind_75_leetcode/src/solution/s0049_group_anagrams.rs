#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!()
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
