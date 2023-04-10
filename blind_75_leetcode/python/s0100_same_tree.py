import unittest
from typing import Optional

from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if (not p and not q) or (
            p
            and q
            and p.val == q.val
            and self.isSameTree(p.left, q.left)
            and self.isSameTree(p.right, q.right)
        ):
            return True
        else:
            return False


class TestS100(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().isSameTree(
                create_tree_from_list([1, 2, 3]), create_tree_from_list([1, 2, 3])
            ),
            True,
        )
