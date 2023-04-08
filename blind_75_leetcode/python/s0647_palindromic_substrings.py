import unittest


class Solution:
    def countSubstrings(self, s: str) -> int:
        def palindromic(i, j) -> int:
            count = 0
            while i >= 0 and j < len(s) and s[i] == s[j]:
                count += 1
                i -= 1
                j += 1
            return count

        return sum(palindromic(i, i) + palindromic(i, i + 1) for i in range(len(s)))


class TestS647(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.countSubstrings("abc"), 3)
        self.assertEqual(s.countSubstrings("aaa"), 6)
