#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if Solution::dfs(x, y, 0, &word, &board, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        x: usize,
        y: usize,
        matched: usize,
        word: &[char],
        board: &[Vec<char>],
        visited: &mut [Vec<bool>],
    ) -> bool {
        if board[x][y] != word[matched] {
            return false;
        }
        if matched == word.len() - 1 {
            return true;
        }
        visited[x][y] = true;
        let result = (x > 0
            && !visited[x - 1][y]
            && Solution::dfs(x - 1, y, matched + 1, word, board, visited))
            || (x < board.len() - 1
                && !visited[x + 1][y]
                && Solution::dfs(x + 1, y, matched + 1, word, board, visited))
            || (y > 0
                && !visited[x][y - 1]
                && Solution::dfs(x, y - 1, matched + 1, word, board, visited))
            || (y < board[0].len() - 1
                && !visited[x][y + 1]
                && Solution::dfs(x, y + 1, matched + 1, word, board, visited));

        visited[x][y] = false;
        result
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
