import unittest


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        left = right = result = 0
        while right < len(s):
            if s[right] in s[left:right]:
                result = max(result, right - left)
                while s[left] != s[right]:
                    left += 1
                left += 1
            right += 1
        result = max(result, right - left)
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
