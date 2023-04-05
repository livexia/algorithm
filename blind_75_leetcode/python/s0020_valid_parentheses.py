import unittest


class Solution:
    def isValid(self, s: str) -> bool:
        if len(s) & 1:
            return False
        stack = []
        parentheses = {"(": ")", "[": "]", "{": "}"}
        for c in s:
            if c in parentheses:
                stack.append(c)
            else:
                if not stack or parentheses[stack.pop()] != c:
                    return False

        return not stack


class TestS20(unittest.TestCase):
    def test_works(self):
        s = Solution()
        self.assertEqual(s.isValid("()"), True)
        self.assertEqual(s.isValid("()[]{}"), True)
        self.assertEqual(s.isValid("(]"), False)
        self.assertEqual(s.isValid("}("), False)
