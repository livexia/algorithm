import unittest

from collections import Counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return Counter(s) == Counter(t)


class TestS242(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.isAnagram(s="anagram", t="nagaram"), True)
        self.assertEqual(s.isAnagram(s="rat", t="car"), False)
