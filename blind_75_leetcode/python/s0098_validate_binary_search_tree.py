import unittest
from typing import Optional


from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def validation(
            node: Optional[TreeNode],
            min: Optional[int],
            max: Optional[int],
        ) -> bool:
            if node is None:
                return True
            else:
                l_flag, r_flag = True, True
                if min is not None:
                    l_flag = min < node.val
                if max is not None:
                    r_flag = max > node.val
                return (
                    l_flag
                    and r_flag
                    and validation(node.left, min, node.val)
                    and validation(node.right, node.val, max)
                )

        return validation(root, None, None)


class TestS98(unittest.TestCase):
    def test_works(self):
        self.assertEqual(Solution().isValidBST(create_tree_from_list([2, 1, 3])), True)
        print(create_tree_from_list([0, None, -1]))
        self.assertEqual(
            Solution().isValidBST(create_tree_from_list([0, None, -1])), False
        )
        self.assertEqual(
            Solution().isValidBST(create_tree_from_list([5, 1, 4, None, None, 3, 6])),
            False,
        )
