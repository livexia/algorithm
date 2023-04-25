#![allow(dead_code)]

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        trie.add_from_board(&board);
        words.into_iter().filter(|w| trie.search_word(w)).collect()
    }
}

#[derive(Default, Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Trie {
        Trie::default()
    }
}

impl Trie {
    fn add_from_board(&mut self, board: &[Vec<char>]) {
        let board: Vec<Vec<_>> = board
            .iter()
            .map(|r| r.iter().map(|&c| (c as u8 - b'a') as usize).collect())
            .collect();
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            for j in 0..n {
                self._add_from_board(&board, i, j, &mut HashSet::new());
            }
        }
    }

    fn _add_from_board(
        &mut self,
        board: &[Vec<usize>],
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        visited.insert((i, j));
        let (m, n) = (board.len(), board[0].len());
        let node = self.children[board[i][j]].get_or_insert(Default::default());
        if i > 0 && !visited.contains(&(i - 1, j)) {
            node._add_from_board(board, i - 1, j, visited)
        }
        if i + 1 < m && !visited.contains(&(i + 1, j)) {
            node._add_from_board(board, i + 1, j, visited)
        }
        if j > 0 && !visited.contains(&(i, j - 1)) {
            node._add_from_board(board, i, j - 1, visited)
        }
        if j + 1 < n && !visited.contains(&(i, j + 1)) {
            node._add_from_board(board, i, j + 1, visited)
        }
        visited.remove(&(i, j));
    }

    fn search_word(&self, word: &str) -> bool {
        let mut node = self;
        for b in word.bytes().map(|c| (c - b'a') as usize) {
            if let Some(child) = &node.children[b] {
                node = child
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests_212 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_string(),
                    "pea".to_string(),
                    "eat".to_string(),
                    "rain".to_string()
                ]
            )
            .iter()
            .collect::<HashSet<_>>(),
            ["eat".to_string(), "oath".to_string()]
                .iter()
                .collect::<HashSet<_>>()
        );
        assert_eq!(
            Solution::find_words(
                vec![vec!['a', 'b'], vec!['c', 'd'],],
                vec!["abcb".to_string(),]
            )
            .is_empty(),
            true
        );
    }
}
