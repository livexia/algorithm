from typing import Optional
import unittest

from .test_s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        stack = []
        while stack or root is not None:
            while root is not None:
                stack.append(root)
                root = root.left

            root = stack.pop()
            k -= 1
            if k == 0:
                return root.val

            root = root.right
        return 0


class TestS230(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().kthSmallest(create_tree_from_list([3, 1, 4, None, 2]), 1), 1
        )
        self.assertEqual(
            Solution().kthSmallest(
                create_tree_from_list([5, 3, 6, 2, 4, None, None, 1]), 3
            ),
            3,
        )
