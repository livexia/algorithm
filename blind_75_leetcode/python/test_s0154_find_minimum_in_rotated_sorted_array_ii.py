import unittest
from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        left = 0
        right = len(nums) - 1
        while left < right:
            if nums[left] < nums[right]:
                return nums[left]
            pivot = (left + right) >> 1
            if nums[pivot] > nums[left]:
                left = pivot + 1
            elif nums[pivot] < nums[left]:
                right = pivot
            else:
                left += 1
        return nums[right]


class TestS154(unittest.TestCase):
    def test_works(self):
        self.assertEqual(Solution().findMin([1, 3, 5]), 1)
        self.assertEqual(Solution().findMin([2, 2, 2, 0, 1]), 0)
