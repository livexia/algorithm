from typing import List, Optional
from collections import deque
import unittest


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self) -> str:
        return f"<val:{self.val} left:{self.left} right:{self.right}>"


def create_tree_from_list(vals: List[Optional[int]]) -> Optional[TreeNode]:
    if not vals[0]:
        return None
    head = TreeNode(vals[0])
    queue = deque([head])
    i = 1
    while i < len(vals):
        node = queue.popleft()
        if node:
            a, b = vals[i], vals[i + 1]
            if a:
                node.left = TreeNode(a)
                queue.append(node.left)
            if b:
                node.right = TreeNode(b)
                queue.append(node.right)
        i += 2
    return head


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root:
            return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))
        else:
            return 0


class TestS104(unittest.TestCase):
    def test_works(self):
        root = create_tree_from_list([3, 9, 20, None, None, 15, 7])
        self.assertEqual(Solution().maxDepth(root), 3)
