#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        words.iter().for_each(|w| trie.add_word(w));
        trie.search_board(&board)
    }
}

#[derive(Default, Debug)]
struct Trie {
    is_word: Option<String>,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Trie {
        Trie::default()
    }
}

// build from words, search with board
impl Trie {
    fn add_word(&mut self, word: &str) {
        let node = word
            .bytes()
            .map(|c| (c - b'a') as usize)
            .fold(self, |node, b| {
                node.children[b].get_or_insert(Default::default())
            });

        node.is_word = Some(word.to_string());
    }

    fn search_board(&mut self, board: &[Vec<char>]) -> Vec<String> {
        let mut board: Vec<Vec<_>> = board
            .iter()
            .map(|r| r.iter().map(|&c| (c as u8 - b'a') as usize).collect())
            .collect();
        let (m, n) = (board.len(), board[0].len());
        let mut r = vec![];
        for i in 0..m {
            for j in 0..n {
                self._search_board(&mut board, i, j, &mut r)
            }
        }
        r
    }

    fn _search_board(
        &mut self,
        board: &mut [Vec<usize>],
        i: usize,
        j: usize,
        searched: &mut Vec<String>,
    ) {
        let byte = board[i][j];
        board[i][j] = 27;
        if let Some(node) = &mut self.children[byte] {
            let (m, n) = (board.len(), board[0].len());
            if let Some(word) = node.is_word.take() {
                searched.push(word)
            }
            if i > 0 && board[i - 1][j] != 27 {
                node._search_board(board, i - 1, j, searched);
            }
            if i + 1 < m && board[i + 1][j] != 27 {
                node._search_board(board, i + 1, j, searched);
            }
            if j > 0 && board[i][j - 1] != 27 {
                node._search_board(board, i, j - 1, searched);
            }
            if j + 1 < n && board[i][j + 1] != 27 {
                node._search_board(board, i, j + 1, searched);
            }
            if node.children.iter().all(|c| c.is_none()) {
                self.children[byte] = None;
            }
        }
        board[i][j] = byte;
    }
}

// biild form board, search with words
impl Trie {
    fn add_from_board(&mut self, board: &[Vec<char>]) {
        let mut board: Vec<Vec<_>> = board
            .iter()
            .map(|r| r.iter().map(|&c| (c as u8 - b'a') as usize).collect())
            .collect();
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            for j in 0..n {
                self._add_from_board(&mut board, i, j);
            }
        }
    }

    fn _add_from_board(&mut self, board: &mut [Vec<usize>], i: usize, j: usize) {
        let byte = board[i][j];
        board[i][j] = 27;
        let (m, n) = (board.len(), board[0].len());
        let node = self.children[byte].get_or_insert(Default::default());
        if i > 0 && board[i - 1][j] != 27 {
            node._add_from_board(board, i - 1, j)
        }
        if i + 1 < m && board[i + 1][j] != 27 {
            node._add_from_board(board, i + 1, j)
        }
        if j > 0 && board[i][j - 1] != 27 {
            node._add_from_board(board, i, j - 1)
        }
        if j + 1 < n && board[i][j + 1] != 27 {
            node._add_from_board(board, i, j + 1)
        }
        board[i][j] = byte;
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
    use std::collections::HashSet;

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
        assert_eq!(
            Solution::find_words(vec![vec!['a'],], vec!["a".to_string()]),
            vec!["a".to_string()]
        );
    }
}
