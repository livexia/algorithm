import unittest


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        def hash(c) -> int:
            return ord(c) - ord("A")

        left = max_count = result = 0
        freq = [0] * 26

        for (right, c) in enumerate(s):
            freq[hash(c)] += 1
            max_count = max(max_count, freq[hash(c)])
            if right - left + 1 > max_count + k:
                freq[hash(s[left])] -= 1
                left += 1
            result = max(result, right - left + 1)
        return result


class TestSolution(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.characterReplacement("ABAB", 2), 4)
        self.assertEqual(s.characterReplacement("AABABBA", 1), 4)
