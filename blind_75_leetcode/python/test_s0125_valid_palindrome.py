import unittest


class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = "".join(c.upper() for c in s if c.isalnum())
        for i in range(len(s) // 2):
            # negative indexing
            # ~0 == -1
            if s[i] != s[~i]:
                return False
        return True
        # return s == s[::-1]


class TestS125(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.isPalindrome("A man, a plan, a canal: Panama"), True)
        self.assertEqual(s.isPalindrome("race a car"), False)
        self.assertEqual(s.isPalindrome(""), True)
        self.assertEqual(s.isPalindrome("a."), True)
        self.assertEqual(s.isPalindrome(".?"), True)
