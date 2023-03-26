#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        todo!()
    }

    fn dfs(x: usize, y: usize, word: &[char], mut matched: usize, board: &[Vec<char>]) -> bool {
        if matched >= word.len() {
            return true;
        }
        if word[matched] == board[x][y] {
            matched += 1;
        } else {
            matched = 0;
        }

        if x > 0 && Solution::dfs(x - 1, y, word, matched, board) {}
        if x < board.len() - 1 && Solution::dfs(x + 1, y, word, matched, board) {}
        if y > 0 && Solution::dfs(x, y - 1, word, matched, board) {}
        if y < board[0].len() - 1 && Solution::dfs(x, y + 1, word, matched, board) {}
        todo!()
    }
}

#[cfg(test)]
mod tests_79 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
    }
}
