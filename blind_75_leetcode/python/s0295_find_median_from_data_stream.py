from typing import List
import unittest

from heapq import heappush, heappop


class MedianFinder:
    left_heap: List[int]
    right_heap: List[int]

    def __init__(self):
        self.left_heap = []
        self.right_heap = []

    def addNum(self, num: int) -> None:
        if not self.left_heap or -self.left_heap[0] >= num:
            heappush(self.left_heap, -num)
            if len(self.left_heap) > len(self.right_heap) + 1:
                heappush(self.right_heap, -heappop(self.left_heap))
        else:
            heappush(self.right_heap, num)
            if len(self.right_heap) > len(self.left_heap):
                heappush(self.left_heap, -heappop(self.right_heap))

    def findMedian(self) -> float:
        if len(self.left_heap) > len(self.right_heap):
            return -self.left_heap[0]
        else:
            return (-self.left_heap[0] + self.right_heap[0]) / 2


class TestS295(unittest.TestCase):
    def test_works(self):
        mf = MedianFinder()
        mf.addNum(1)
        mf.addNum(2)
        self.assertEqual(mf.findMedian(), 1.5)
        mf.addNum(3)
        self.assertEqual(mf.findMedian(), 2)
