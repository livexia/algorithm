from typing import Optional, List
import unittest


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        trie = Trie()
        for w in words:
            trie.add(w)
        print(trie.children)
        r = trie.search_board(board)
        print(trie.children)
        return r


class Trie:
    word: Optional[str]
    children: List[Optional["Trie"]]

    def __init__(self) -> None:
        self.word = None
        self.children = [None] * 26
        pass

    def add(self, word: str):
        node: Trie = self
        for c in (ord(c) - ord("a") for c in word):
            if node.children[c] is None:
                node.children[c] = Trie()
            node = node.children[c]  # type: ignore
        node.word = word

    def search_board(self, board: List[List[str]]) -> List[str]:
        m, n = len(board), len(board[0])
        searched = []
        for i in range(m):
            for j in range(n):
                self._search_board(board, i, j, searched)
        return searched

    def _search_board(
        self, board: List[List[str]], i: int, j: int, searched: List[str]
    ):
        m, n = len(board), len(board[0])
        c = board[i][j]
        board[i][j] = "#"
        index = ord(c) - ord("a")
        node = self.children[index]
        if node is not None:
            if node.word is not None:
                searched.append(node.word)
                node.word = None
            for (x, y) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
                if 0 <= x < m and 0 <= y < n and board[x][y] != "#":
                    node._search_board(board, x, y, searched)

            if all([child is None for child in node.children]):
                self.children[index] = None
        board[i][j] = c


class TestS212(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            set(
                Solution().findWords(
                    board=[
                        ["o", "a", "a", "n"],
                        ["e", "t", "a", "e"],
                        ["i", "h", "k", "r"],
                        ["i", "f", "l", "v"],
                    ],
                    words=["oath", "pea", "eat", "rain"],
                )
            ),
            set(["eat", "oath"]),
        )
        self.assertEqual(
            Solution().findWords(board=[["a", "b"], ["c", "d"]], words=["abcb"]),
            [],
        )
