import unittest
from typing import Optional


from .s0104_maximum_depth_of_binary_tree import create_tree_from_list, TreeNode


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return root
        root.left, root.right = root.right, root.left
        # self.invertTree(root.left)
        # self.invertTree(root.right)
        root.left = self.invertTree(root.left)
        root.right = self.invertTree(root.right)
        return root


class TestS226(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().invertTree(create_tree_from_list([4, 2, 7, 1, 3, 6, 9])),
            create_tree_from_list([4, 7, 2, 9, 6, 3, 1]),
        )
