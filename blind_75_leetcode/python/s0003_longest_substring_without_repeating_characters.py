import unittest


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        left = result = 0
        n = len(s)
        for (right, c) in enumerate(s):
            if left + result >= n:
                return result
            if c in s[left:right]:
                result = max(result, right - left)
                while s[left] != s[right]:
                    left += 1
                left += 1
        result = max(result, n - left)
        return result


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

    def test05(self):
        self.assertEqual(self.helper("abcb"), 3)
