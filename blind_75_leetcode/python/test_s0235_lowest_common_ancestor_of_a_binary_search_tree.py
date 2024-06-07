import unittest

from .test_s0104_maximum_depth_of_binary_tree import (
    TreeNode,
    create_tree_from_list,
    search_val,
)


class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        if root.val < p.val and root.val < q.val:
            return self.lowestCommonAncestor(root.right, p, q)
        elif root.val > p.val and root.val > q.val:
            return self.lowestCommonAncestor(root.left, p, q)
        else:
            return root


class Solution2:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        while True:
            if root.val < p.val and root.val < q.val:
                root = root.right
            elif root.val > p.val and root.val > q.val:
                root = root.left
            else:
                return root


class TestS235(unittest.TestCase):
    def test_works(self):
        root = create_tree_from_list([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])
        self.assertEqual(
            Solution().lowestCommonAncestor(
                root, search_val(root, 2), search_val(root, 8)
            ),
            search_val(root, 6),
        )
        self.assertEqual(
            Solution().lowestCommonAncestor(
                root, search_val(root, 2), search_val(root, 4)
            ),
            search_val(root, 2),
        )

    def test_solution2_works(self):
        Solution = Solution2
        root = create_tree_from_list([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])
        self.assertEqual(
            Solution().lowestCommonAncestor(
                root, search_val(root, 2), search_val(root, 8)
            ),
            search_val(root, 6),
        )
        self.assertEqual(
            Solution().lowestCommonAncestor(
                root, search_val(root, 2), search_val(root, 4)
            ),
            search_val(root, 2),
        )
