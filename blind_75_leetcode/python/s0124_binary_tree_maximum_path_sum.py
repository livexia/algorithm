from typing import Optional
import unittest


from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def __init__(self):
        self.max_sum = float("-inf")

    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def max_gain(node: Optional[TreeNode]) -> int:
            if node:
                left = max(max_gain(node.left), 0)
                right = max(max_gain(node.right), 0)
                self.max_sum = max(self.max_sum, left + right + node.val)

                return max(left, right) + node.val
            else:
                return 0

        max_gain(root)
        return int(self.max_sum)


class TestS124(unittest.TestCase):
    def test_works(self):
        self.assertEqual(Solution().maxPathSum(create_tree_from_list([1, 2, 3])), 6)
        self.assertEqual(
            Solution().maxPathSum(
                create_tree_from_list([-10, 9, 20, None, None, 15, 7])
            ),
            42,
        )
