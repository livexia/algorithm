from typing import Tuple
import unittest


class Solution:
    def longestPalindrome(self, s: str) -> str:
        def expand(i, j) -> Tuple[int, int]:
            while i >= 0 and j < len(s) and s[i] == s[j]:
                i -= 1
                j += 1
            return (i + 1, j)

        start, end = (0, 0)
        for i in range(len(s)):
            p1 = expand(i, i)
            p2 = expand(i, i + 1)
            p = p1 if p1[1] - p1[0] > p2[1] - p2[0] else p2
            start, end = p if p[1] - p[0] > end - start else (start, end)
        return "".join(s[start:end])


class TestS5(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.longestPalindrome("babad") in ["bab", "aba"], True)
        self.assertEqual(s.longestPalindrome("cbbd"), "bb")
