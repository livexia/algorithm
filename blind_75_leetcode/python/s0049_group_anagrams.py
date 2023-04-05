import unittest
from typing import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        return []


class TestS49(unittest.TestCase):
    def test_works(self):
        def equal_helper(result: List[List[str]], expected: List[List[str]]) -> bool:
            if len(result) != len(expected):
                return False
            result_set = [set(l1) for l1 in result]
            expected_set = [set(l1) for l1 in expected]
            result_set.sort(key=len)
            for (v1, v2) in zip(result_set, expected_set):
                if len(v1) != len(v2) or any([s not in v2 for s in v1]):
                    return False

            return True

        s = Solution()
        self.assertEqual(
            s.groupAnagrams(strs=["eat", "tea", "tan", "ate", "nat", "bat"]),
            [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]],
        )
        self.assertEqual(s.groupAnagrams(strs=[""]), [[""]])
        self.assertEqual(s.groupAnagrams(strs=["a"]), [["a"]])
