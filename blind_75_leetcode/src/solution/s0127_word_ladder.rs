#![allow(dead_code)]
pub struct Solution {}

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_length = begin_word.len();
        let mut chars: Vec<HashSet<u8>> = vec![HashSet::new(); word_length];
        for word in &word_list {
            for (i, c) in word.bytes().enumerate() {
                chars[i].insert(c - 'a' as u8);
            }
        }
        let word_list: HashSet<u32> = word_list
            .into_iter()
            .map(|s| Solution::vec_to_num(Solution::word_to_vec(s)))
            .collect();
        let end_word = Solution::vec_to_num(Solution::word_to_vec(end_word));
        let begin_word = Solution::vec_to_num(Solution::word_to_vec(begin_word));
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut queue: VecDeque<(u32, i32)> = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((begin_word, 1));
        while let Some((cur, count)) = queue.pop_front() {
            if cur == end_word {
                return count;
            }
            if visited.insert(cur.clone()) {
                for i in 0..word_length {
                    for &c in &chars[i] {
                        let next = Solution::replace_at(cur, c, i);
                        // println!(
                        //     "{} {} {} {} {:?} {:?} {:?}",
                        //     cur,
                        //     next,
                        //     c,
                        //     i,
                        //     Solution::num_to_vec(cur),
                        //     Solution::num_to_vec(next),
                        //     Solution::num_to_vec(end_word),
                        // );
                        if cur == next {
                            continue;
                        }
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

    fn word_to_vec(word: String) -> Vec<u8> {
        word.bytes().map(|c| c as u8 - 'a' as u8 + 1).collect()
    }

    fn num_to_vec(num: u32) -> Vec<u8> {
        let mut num = num;
        let mut res = vec![];
        while num != 0 {
            res.push((num % 27) as u8);
            num /= 27;
        }
        res.into_iter().rev().collect()
    }

    fn vec_to_num(word: Vec<u8>) -> u32 {
        word.into_iter().fold(0, |sum, i| sum * 27 + i as u32)
    }

    fn replace_at(a: u32, c: u8, index: usize) -> u32 {
        let mut v = Solution::num_to_vec(a);
        v[index] = c + 1;
        Solution::vec_to_num(v)
    }
}

#[cfg(test)]
mod tests_127 {
    use super::*;

    #[test]
    fn test_word_to_num() {
        assert_eq!(
            28,
            Solution::vec_to_num(Solution::word_to_vec("aa".to_string()))
        );
        assert_eq!(
            26,
            Solution::vec_to_num(Solution::word_to_vec("z".to_string()))
        );
        assert_eq!(vec![26], Solution::word_to_vec("z".to_string()));
        assert_eq!(
            56,
            Solution::vec_to_num(Solution::word_to_vec("bb".to_string()))
        );
        assert_eq!(
            Solution::word_to_vec("bb".to_string()),
            Solution::num_to_vec(56)
        );
        assert_eq!(
            Solution::word_to_vec("aa".to_string()),
            Solution::num_to_vec(28)
        );
        assert_eq!(
            Solution::word_to_vec("z".to_string()),
            Solution::num_to_vec(26)
        );
        assert_eq!(
            Solution::replace_at(28, 1, 0),
            Solution::vec_to_num([2, 1].to_vec())
        );
    }

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
        assert_eq!(
            ladder_length(
                "ymain",
                "oecij",
                vec![
                    "ymann", "yycrj", "oecij", "ymcnj", "yzcrj", "yycij", "xecij", "yecij",
                    "ymanj", "yzcnj", "ymain"
                ]
            ),
            10
        );
    }
}
