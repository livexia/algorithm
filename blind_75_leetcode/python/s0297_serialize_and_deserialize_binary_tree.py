from re import split
import unittest

from .s0104_maximum_depth_of_binary_tree import TreeNode, create_tree_from_list


class Solution:
    # This is a template
    pass


class Codec:
    def serialize(self, root):
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """

        def preorder(node):
            if node:
                return (
                    f"{node.val}"
                    + ","
                    + preorder(node.left)
                    + ","
                    + preorder(node.right)
                )
            else:
                return "None"

        return preorder(root)

    def deserialize(self, data):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """

        def preorder():
            val = data.pop()
            if val == "None":
                return None
            node = TreeNode(int(val))
            node.left = preorder()
            node.right = preorder()

            return node

        data = data.split(",")
        data.reverse()
        return preorder()


class TestS297(unittest.TestCase):
    def test_works(self):
        codec = Codec()
        self.assertEqual(
            codec.serialize(create_tree_from_list([1, 2, 3, None, None, 4, 5])),
            "1,2,None,None,3,4,None,None,5,None,None",
        )

        self.assertEqual(
            codec.deserialize("1,2,None,None,3,4,None,None,5,None,None"),
            create_tree_from_list([1, 2, 3, None, None, 4, 5]),
        )
