from typing import List
import unittest


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m = len(board)
        n = len(board[0])
        visited = [[False for _ in range(n)] for _ in range(m)]

        def dfs(x: int, y: int, matched: int) -> bool:
            if board[x][y] != word[matched]:
                return False
            if matched == len(word) - 1:
                return True
            visited[x][y] = True
            resullt = (
                (x > 0 and not visited[x - 1][y] and dfs(x - 1, y, matched + 1))
                or (x < m - 1 and not visited[x + 1][y] and dfs(x + 1, y, matched + 1))
                or (y > 0 and not visited[x][y - 1] and dfs(x, y - 1, matched + 1))
                or (y < n - 1 and not visited[x][y + 1] and dfs(x, y + 1, matched + 1))
            )
            visited[x][y] = False
            return resullt

        for x in range(m):
            for y in range(n):
                if dfs(x, y, 0):
                    return True

        return False


class TestWordSearch(unittest.TestCase):
    def test_it_works(self):
        s = Solution()
        self.assertTrue(
            s.exist(
                board=[
                    ["A", "B", "C", "E"],
                    ["S", "F", "C", "S"],
                    ["A", "D", "E", "E"],
                ],
                word="ABCCED",
            )
        )

    def test_example2(self):
        s = Solution()
        self.assertTrue(
            s.exist(
                board=[
                    ["A", "B", "C", "E"],
                    ["S", "F", "C", "S"],
                    ["A", "D", "E", "E"],
                ],
                word="SEE",
            )
        )

    def test_example3(self):
        s = Solution()
        self.assertFalse(
            s.exist(
                board=[
                    ["A", "B", "C", "E"],
                    ["S", "F", "C", "S"],
                    ["A", "D", "E", "E"],
                ],
                word="ABCB",
            )
        )


# if __name__ == "__main__":
# unittest.main()
