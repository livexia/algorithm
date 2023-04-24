from __future__ import annotations
from typing import List, Optional
import unittest


class WordDictionary:
    is_word: bool
    children: List[Optional[WordDictionary]]

    def __init__(self):
        self.is_word = False
        self.children = [None] * 26

    def addWord(self, word: str) -> None:
        node: WordDictionary = self
        for index in (ord(c) - ord("a") for c in word):
            if node.children[index] is None:
                node.children[index] = WordDictionary()
            node = node.children[index]  # type: ignore
        node.is_word = True

    def _search(self, word: str, index: int) -> bool:
        if index == len(word):
            return self.is_word
        elif word[index] == ".":
            return any(
                (
                    child._search(word, index + 1)
                    for child in self.children
                    if child is not None
                )
            )
        else:
            child = self.children[ord(word[index]) - ord("a")]
            if child is None:
                return False
            else:
                return child._search(word, index + 1)

    def search(self, word: str) -> bool:
        return self._search(word, 0)


class TestS211(unittest.TestCase):
    def test_works(self):
        w = WordDictionary()
        w.addWord("bad")
        w.addWord("dad")
        w.addWord("mad")
        self.assertEqual(w.search("pad"), False)
        self.assertEqual(w.search("bad"), True)
        self.assertEqual(w.search(".ad"), True)
        self.assertEqual(w.search("b.."), True)
