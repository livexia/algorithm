#![allow(dead_code)]
pub struct Solution {}

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_length = begin_word.len();
        let word_list: HashSet<Vec<u8>> = word_list
            .into_iter()
            .map(|s| s.bytes().into_iter().map(|c| c as u8 - 'a' as u8).collect())
            .collect();
        let end_word: Vec<u8> = end_word.bytes().map(|c| c as u8 - 'a' as u8).collect();
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut chars: Vec<HashSet<u8>> = vec![HashSet::new(); word_length];
        for word in &word_list {
            for (i, &c) in word.iter().enumerate() {
                chars[i].insert(c);
            }
        }
        let mut queue: VecDeque<(Vec<u8>, i32)> = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((begin_word.bytes().map(|c| c as u8 - 'a' as u8).collect(), 1));
        while let Some((cur, count)) = queue.pop_front() {
            if cur == end_word {
                return count;
            }
            if visited.insert(cur.clone()) {
                for i in 0..word_length {
                    for &c in &chars[i] {
                        if cur[i] == c {
                            continue;
                        }
                        let mut next = cur.clone();
                        next[i] = c;
                        if !word_list.contains(&next) {
                            continue;
                        }
                        queue.push_back((next, count + 1));
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests_127 {
    use super::*;

    fn ladder_length(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> i32 {
        Solution::ladder_length(
            begin_word.to_string(),
            end_word.to_string(),
            word_list.into_iter().map(|s| s.to_string()).collect(),
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(
            ladder_length("hit", "cog", vec!["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );
        assert_eq!(
            ladder_length("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"]),
            0
        );
    }
}
