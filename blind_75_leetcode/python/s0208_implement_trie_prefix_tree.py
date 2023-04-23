import unittest
from typing import Optional


class Trie:  # type: ignore
    def __init__(self):
        self.is_word = False
        self.children = [None] * 26

    def insert(self, word: str) -> None:
        node = self
        for index in (ord(c) - ord("a") for c in word):
            if node.children[index] is None:
                node.children[index] = Trie()
            node = node.children[index]
        node.is_word = True

    def _search(self, word: str) -> Optional[bool]:
        node = self
        for index in (ord(c) - ord("a") for c in word):
            if node.children[index] is None:
                return None
            node = node.children[index]
        return node.is_word

    def search(self, word: str) -> bool:
        return bool(self._search(word))

    def startsWith(self, prefix: str) -> bool:
        return self._search(prefix) is not None


class TestS208(unittest.TestCase):
    def test_works(self):
        trie = Trie()
        trie.insert("apple")
        self.assertEqual(trie.search("apple"), True)
        self.assertEqual(trie.search("app"), False)
        self.assertEqual(trie.startsWith("app"), True)
        trie.insert("app")
        self.assertEqual(trie.search("app"), True)
