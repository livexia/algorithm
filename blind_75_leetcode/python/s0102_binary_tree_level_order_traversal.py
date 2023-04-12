import unittest
from typing import Optional, List

from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list

from collections import deque


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        queue = deque([(root, int(0))])
        r = []
        while queue:
            node, level = queue.popleft()
            if node:
                if level == len(r):
                    r.append([])
                r[-1].append(node.val)
                queue.append((node.left, level + 1))
                queue.append((node.right, level + 1))
        return r


class TestS102(unittest.TestCase):
    def test_works(self):
        self.assertEqual(
            Solution().levelOrder(create_tree_from_list([3, 9, 20, None, None, 15, 7])),
            [[3], [9, 20], [15, 7]],
        )
