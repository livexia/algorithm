import unittest

from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        counter = Counter(t)
        left = 0
        result = (0, len(s))

        t_count = len(t)
        for (right, c) in enumerate(s):
            if c in counter:
                if counter[c] > 0:
                    t_count -= 1
                counter[c] -= 1
            if t_count == 0:
                while True:
                    if s[left] in counter:
                        if counter[s[left]] == 0:
                            break
                        counter[s[left]] += 1
                    left += 1
                if right - left < result[1] - result[0]:
                    result = (left, right)
                counter[s[left]] += 1
                t_count += 1
                left += 1

        return "" if result[1] == len(s) else s[result[0] : result[1] + 1]


class TestS76(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.minWindow(s="ADOBECODEBANC", t="ABC"), "BANC")
        self.assertEqual(s.minWindow(s="a", t="a"), "a")
        self.assertEqual(s.minWindow(s="a", t="aa"), "")
