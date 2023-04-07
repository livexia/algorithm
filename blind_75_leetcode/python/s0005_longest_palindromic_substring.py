import unittest


class Solution:
    def longestPalindrome(self, s: str) -> str:
        def dp(i, j) -> bool:
            return i == j or s[i] == s[j] and (i + 1 == j or dp(i + 1, j - 1))

        result = (0, 0)
        for i in range(len(s)):
            if len(s) - 1 - i < result[1] - result[0]:
                break
            for j in reversed(range(result[1] - result[0] + i, len(s))):
                if dp(i, j):
                    result = (i, j)
                    break
        return "".join(s[result[0] : result[1] + 1])


class TestS5(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.longestPalindrome("babad") in ["bab", "aba"], True)
        self.assertEqual(s.longestPalindrome("cbbd"), "bb")
