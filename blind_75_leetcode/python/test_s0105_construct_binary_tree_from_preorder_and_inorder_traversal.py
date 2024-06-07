import unittest
from typing import List, Optional

from .test_s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        self.pre_index = 0
        inorder_index = {v: i for (i, v) in enumerate(inorder)}

        def build_node(in_start: int, in_end: int) -> Optional[TreeNode]:
            if in_start == in_end:
                return None
            else:
                node = TreeNode(preorder[self.pre_index])
                self.pre_index += 1
                in_index = inorder_index[node.val]
                node.left = build_node(in_start, in_index)
                node.right = build_node(in_index + 1, in_end)
                return node

        return build_node(0, len(inorder))

    def buildTree2(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder:
            return None
        root = TreeNode(preorder[0])
        stack = [root]
        in_index = 0
        for node_val in preorder[1:]:
            node = stack[-1]
            if node.val != inorder[in_index]:
                node.left = TreeNode(node_val)
                stack.append(node.left)
            else:
                while stack and stack[-1].val == inorder[in_index]:
                    node = stack.pop()
                    in_index += 1
                node.right = TreeNode(node_val)
                stack.append(node.right)
        return root


class TestS105(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().buildTree([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]),
            create_tree_from_list([3, 9, 20, None, None, 15, 7]),
        )
        self.assertEqual(
            Solution().buildTree([-1], [-1]),
            create_tree_from_list([-1]),
        )

    def test_bfs_works(self):
        self.assertEqual(
            Solution().buildTree2([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]),
            create_tree_from_list([3, 9, 20, None, None, 15, 7]),
        )
        self.assertEqual(
            Solution().buildTree2([-1], [-1]),
            create_tree_from_list([-1]),
        )
