import unittest
from typing import Tuple


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        def dp(i: int) -> Tuple[int, int, int]:
            if i == len(s):
                return (0, 0, 0)
            hi = 1 << (ord(s[i]) - ord("a"))
            (m, l, h) = dp(i + 1)
            if hi & h != 0:
                return (max(m, 1), 1, hi)
            else:
                return (max(m, l + 1), l + 1, h | hi)

        return dp(0)[0]


class Test0003(unittest.TestCase):
    def helper(self, s):
        solution = Solution()
        return solution.lengthOfLongestSubstring(s)

    def test01(self):
        self.assertEqual(self.helper("abcabcbb"), 3)

    def test02(self):
        self.assertEqual(self.helper("bbbbb"), 1)

    def test03(self):
        self.assertEqual(self.helper("pwwkew"), 3)

    def test04(self):
        self.assertEqual(self.helper(" "), 1)
