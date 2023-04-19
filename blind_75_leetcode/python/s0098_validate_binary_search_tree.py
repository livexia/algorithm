import unittest
from typing import Optional, Tuple


from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def post_order(
            node: Optional[TreeNode],
        ) -> Tuple[(bool, Optional[Tuple[(int, int)]])]:
            if node is None:
                return True, None
            else:
                l_flag, left_result = post_order(node.left)
                r_flag, right_result = post_order(node.right)
                if not r_flag or not l_flag:
                    return False, None
                if not left_result and not right_result:
                    return True, (node.val, node.val)
                if left_result and right_result:
                    if left_result[1] < node.val and right_result[0] > node.val:
                        return True, (left_result[0], right_result[1])
                elif not left_result and right_result:
                    if right_result[0] > node.val:
                        return True, (node.val, right_result[1])
                elif not right_result and left_result:
                    if left_result[1] < node.val:
                        return True, (left_result[0], node.val)
                return False, None

        return post_order(root)[0]


class TestS98(unittest.TestCase):
    def test_works(self):
        self.assertEqual(Solution().isValidBST(create_tree_from_list([2, 1, 3])), True)
        self.assertEqual(
            Solution().isValidBST(create_tree_from_list([5, 1, 4, None, None, 3, 6])),
            False,
        )
