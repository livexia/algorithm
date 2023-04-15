import unittest
from typing import Optional


from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def is_identical(t1: Optional[TreeNode], t2: Optional[TreeNode]) -> bool:
            if not t1 and not t2:
                return True
            elif t1 and t2:
                return (
                    t1.val == t2.val
                    and is_identical(t1.left, t2.left)
                    and is_identical(t1.right, t2.right)
                )
            else:
                return False

        if not subRoot:
            return True
        elif not root:
            return False
        else:
            return (
                is_identical(root, subRoot)
                or self.isSubtree(root.left, subRoot)
                or self.isSubtree(root.right, subRoot)
            )


class TestS572(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().isSubtree(
                create_tree_from_list([3, 4, 5, 1, 2]), create_tree_from_list([4, 1, 2])
            ),
            True,
        )
        self.assertEqual(
            Solution().isSubtree(
                create_tree_from_list([3, 4, 5, 1, 2, None, None, None, None, 0]),
                create_tree_from_list([4, 1, 2]),
            ),
            True,
        )
