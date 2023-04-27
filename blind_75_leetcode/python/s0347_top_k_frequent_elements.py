import unittest
from typing import List

from heapq import nlargest
from collections import Counter


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        counter = Counter(nums)
        heap = [(v, k) for k, v in counter.items()]
        return [k for _, k in nlargest(k, heap)]


class TestS347(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            set(Solution().topKFrequent([1, 1, 1, 2, 2, 3], 2)), set([1, 2])
        )
        self.assertEqual(Solution().topKFrequent([1], 1), [1])
